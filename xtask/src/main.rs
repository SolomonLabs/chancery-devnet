//! cargo xtask <command>
//!
//! Local task runner for the devnet build of Chancery.
//!
//! Commands:
//!   build                build programs/chancery with cargo build-sbf.
//!   test                 run the Rust test suite.
//!   idl [extra...]       generate IDL JSON via rust-idl-generator.
//!   idl-check            verify the on-disk IDL matches source. Non-zero on drift.
//!   client [extra...]    generate the TS client via solana-libgen.
//!   codegen              idl + client back to back.
//!   help                 print usage.
//!
//! Requires: Solana CLI with `cargo build-sbf --arch`, and node for the local
//! generators.

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;
use std::process::Stdio;
use std::thread;

const PROGRAM_NAME: &str = "chancery";
const IDL_METADATA_TEMPLATE_FILE: &str = "artifacts/idl.metadata.json";
const IDL_DIR: &str = "programs/chancery/idl";
const CLIENT_OUT_DIR: &str = "clients/ts";
const DEFAULT_IDL_GEN_BIN: &str = "bin/rust-idl-generator.js";
const DEFAULT_LIBGEN_BIN: &str = "bin/solana-libgen.js";

/// SBPF ISA version (`--arch` for cargo build-sbf).
///
/// The SIMD-0178/0189/0377 feature enabling SBPFv3 deployment and execution is
/// not gate-activated on devnet, so v0 is the only artifact devnet accepts.
const SBPF_ARCH: &str = "v0";

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let command = arguments.first().map(String::as_str).unwrap_or("help");
    let rest = if arguments.is_empty() { &[][..] } else { &arguments[1..] };

    let result = match command {
        "build" => run_build(rest),
        "test" => run_test(),
        "idl" => run_idl_gen(false, rest),
        "idl-check" => run_idl_gen(true, &[]),
        "client" => run_client_gen(rest),
        "codegen" => run_codegen(),
        "help" | "--help" | "-h" => {
            usage();
            Ok(())
        }
        other => {
            eprintln!("xtask: unknown command `{other}`");
            usage();
            return ExitCode::from(2);
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("xtask: {message}");
            ExitCode::FAILURE
        }
    }
}

fn usage() {
    eprintln!(
        "usage: cargo xtask <command>

commands:
  build [chancery|faucet]
                       build the named program with cargo build-sbf --arch {SBPF_ARCH}
  test                 run cargo test --workspace --locked
  idl [extra...]       generate IDL via rust-idl-generator
                         in:  programs/chancery/src/lib.rs
                         out: programs/chancery/idl/idl.json
  idl-check            verify the on-disk IDL matches source
  client [extra...]    generate the TS client via solana-libgen from the IDL
                         in:  programs/chancery/idl/idl.json
                         out: clients/ts/
  codegen              idl + client back to back
"
    );
}

fn workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .expect("xtask must live one level below workspace root")
        .to_path_buf()
}

fn idl_output_path(root: &Path) -> PathBuf {
    root.join(IDL_DIR).join("idl.json")
}

fn rust_entry_path(root: &Path) -> PathBuf {
    root.join("programs").join(PROGRAM_NAME).join("src").join("lib.rs")
}

fn resolve_node_bin(default_rel: &str, root: &Path) -> Result<PathBuf, String> {
    let candidate = root.join(default_rel);
    if !candidate.exists() {
        return Err(format!("node bin not found: {}", candidate.display()));
    }
    Ok(candidate)
}

fn run_build(args: &[String]) -> Result<(), String> {
    let crate_directory = match args.first().map(String::as_str) {
        None | Some("chancery") => PROGRAM_NAME,
        Some("faucet") => "faucet",
        Some(other) => return Err(format!("unknown program: {other} (expected chancery or faucet)")),
    };
    let root = workspace_root();
    let mut build = Command::new("cargo");
    build.current_dir(root.join("programs").join(crate_directory));
    build.args(["build-sbf", "--arch", SBPF_ARCH]);
    spawn(&mut build, "cargo build-sbf")
}

fn run_test() -> Result<(), String> {
    let root = workspace_root();
    let mut test = Command::new("cargo");
    test.current_dir(&root).args(["test", "--workspace", "--locked"]);
    spawn(&mut test, "cargo test")
}

fn run_idl_gen(check_only: bool, extra: &[String]) -> Result<(), String> {
    let root = workspace_root();
    let rust_entry = rust_entry_path(&root);
    if !rust_entry.exists() {
        return Err(format!("rust entry missing: {}", rust_entry.display()));
    }

    let out_path = idl_output_path(&root);
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }

    let check_path = root.join(IDL_DIR).join("idl.check.json");
    let generation_path = if check_only { &check_path } else { &out_path };

    let bin = resolve_node_bin(DEFAULT_IDL_GEN_BIN, &root)?;
    let mut command = Command::new("node");
    command
        .current_dir(&root)
        .arg(&bin)
        .arg("--rust-entry")
        .arg(&rust_entry)
        .arg("--out")
        .arg(generation_path)
        .arg("--program-name")
        .arg(PROGRAM_NAME)
        .arg("--project-root")
        .arg(&root);

    // Descriptions, repository, contact, docs, cargo dependencies, deployments,
    // and per-node source links all come from the committed metadata template.
    // Required, not optional: an IDL generated without it silently publishes a
    // stripped document.
    let metadata_template = root.join(IDL_METADATA_TEMPLATE_FILE);
    if !metadata_template.exists() {
        return Err(format!("idl metadata template missing: {}", metadata_template.display()));
    }
    command.arg("--metadata").arg(&metadata_template);

    for argument in extra {
        command.arg(argument);
    }

    spawn(&mut command, "rust-idl-generator")?;

    if check_only {
        let expected = fs::read(&out_path).map_err(|error| {
            format!(
                "failed to read expected IDL {}: {error} (run `cargo xtask idl`)",
                out_path.display()
            )
        })?;
        let generated = fs::read(&check_path)
            .map_err(|error| format!("failed to read generated IDL {}: {error}", check_path.display()))?;
        fs::remove_file(&check_path)
            .map_err(|error| format!("failed to remove {}: {error}", check_path.display()))?;
        if expected != generated {
            return Err("IDL drift detected: run `cargo xtask idl` before continuing".to_string());
        }
        eprintln!("xtask: IDL is in sync -> {}", out_path.display());
        return Ok(());
    }

    if !out_path.exists() {
        return Err(format!(
            "rust-idl-generator exited successfully but did not create IDL: {}",
            out_path.display()
        ));
    }

    let metadata = fs::metadata(&out_path)
        .map_err(|error| format!("failed to stat {}: {error}", out_path.display()))?;
    if metadata.len() == 0 {
        return Err(format!("rust-idl-generator created an empty IDL: {}", out_path.display()));
    }

    eprintln!("xtask: wrote IDL -> {}", out_path.display());
    Ok(())
}

fn clear_generated_client(out_dir: &Path) -> Result<(), String> {
    let directory = out_dir.join("src");
    if directory.exists() {
        fs::remove_dir_all(&directory)
            .map_err(|error| format!("failed to remove {}: {error}", directory.display()))?;
    }

    for file_name in ["solana-libgen.report.json", ".idl-generator-history.json"] {
        let file = out_dir.join(file_name);
        if file.exists() {
            fs::remove_file(&file)
                .map_err(|error| format!("failed to remove {}: {error}", file.display()))?;
        }
    }

    Ok(())
}

fn run_client_gen(extra: &[String]) -> Result<(), String> {
    let root = workspace_root();
    let idl_path = idl_output_path(&root);
    if !idl_path.exists() {
        return Err(format!("idl missing: {} (run `cargo xtask idl` first)", idl_path.display()));
    }

    let out_dir = root.join(CLIENT_OUT_DIR);
    clear_generated_client(&out_dir)?;
    fs::create_dir_all(&out_dir)
        .map_err(|error| format!("failed to create {}: {error}", out_dir.display()))?;

    let bin = resolve_node_bin(DEFAULT_LIBGEN_BIN, &root)?;
    let mut command = Command::new("node");
    command
        .current_dir(&root)
        .arg(&bin)
        .arg("--idl")
        .arg(&idl_path)
        .arg("--out")
        .arg(&out_dir);
    for argument in extra {
        command.arg(argument);
    }
    spawn(&mut command, "solana-libgen")?;

    eprintln!("xtask: wrote client -> {}", out_dir.display());
    Ok(())
}

fn run_codegen() -> Result<(), String> {
    run_idl_gen(false, &[])?;
    run_client_gen(&[])
}

fn sanitize_command_environment(command: &mut Command) {
    const ALLOWED_KEYS: [&str; 16] = [
        "PATH", "SYSTEMROOT", "WINDIR", "COMSPEC", "PATHEXT", "TMP", "TEMP", "TMPDIR",
        "HOME", "USERPROFILE", "APPDATA", "LOCALAPPDATA", "SSL_CERT_FILE", "SSL_CERT_DIR",
        "LANG", "LC_ALL",
    ];
    let values: Vec<(String, std::ffi::OsString)> = ALLOWED_KEYS
        .iter()
        .filter_map(|key| env::var_os(key).map(|value| ((*key).to_string(), value)))
        .collect();
    command.env_clear();
    command.envs(values);
    command.env("TZ", "UTC");
    command.env("LANG", "C.UTF-8");
    command.env("LC_ALL", "C.UTF-8");
}

fn shell_display(value: &OsStr) -> String {
    let text = value.to_string_lossy();
    if text.is_empty() {
        return "''".to_string();
    }
    let needs_quoting = text
        .chars()
        .any(|character| !(character.is_ascii_alphanumeric() || "-_=/.:,@+".contains(character)));
    if needs_quoting {
        format!("'{}'", text.replace('\'', "'\\''"))
    } else {
        text.into_owned()
    }
}

fn command_display(command: &Command) -> String {
    let mut parts = Vec::new();

    if let Some(current_dir) = command.get_current_dir() {
        parts.push(format!("cd {}", current_dir.display()));
        parts.push("&&".to_string());
    }

    parts.push(command.get_program().to_string_lossy().into_owned());

    for argument in command.get_args() {
        parts.push(shell_display(argument));
    }

    parts.join(" ")
}

fn spawn(command: &mut Command, label: &str) -> Result<(), String> {
    sanitize_command_environment(command);
    eprintln!("xtask: running {label}: {}", command_display(command));

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("{label} failed to spawn: {error}"))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| format!("{label} stdout pipe was unavailable"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| format!("{label} stderr pipe was unavailable"))?;

    let stdout_thread = thread::spawn(move || -> io::Result<()> {
        let mut reader = io::BufReader::new(stdout);
        let mut writer = io::stdout().lock();
        io::copy(&mut reader, &mut writer).map(|_| ())
    });
    let stderr_thread = thread::spawn(move || -> io::Result<()> {
        let mut reader = io::BufReader::new(stderr);
        let mut writer = io::stderr().lock();
        io::copy(&mut reader, &mut writer).map(|_| ())
    });

    let status = child
        .wait()
        .map_err(|error| format!("{label} failed to run: {error}"))?;

    stdout_thread
        .join()
        .map_err(|_| format!("{label} stdout reader panicked"))?
        .map_err(|error| format!("{label} stdout copy failed: {error}"))?;
    stderr_thread
        .join()
        .map_err(|_| format!("{label} stderr reader panicked"))?
        .map_err(|error| format!("{label} stderr copy failed: {error}"))?;

    if !status.success() {
        return Err(format!("{label} exited with status {status}"));
    }
    Ok(())
}
