//! Syntax-aware release gates for the shared state-loader boundary.
//!
//! Every function that returns an `AccountInfo`-backed `Ref`/`RefMut`
//! must return a value whose provenance reaches `state_loader.rs`. Detection is
//! based on the return type and source-wide location, not a short method-name
//! allowlist or a `state/` directory convention. The analysis follows guarded
//! wrapper calls to a fixed point and validates the returned value, so a decoy
//! loader call cannot satisfy the gate.
//!
//! Macro-generated loader/state items and unreviewed attributes/derives are
//! forbidden because this source-AST gate cannot inspect expansion output.
//! Loader candidates also use a deliberately simple control-flow form: no
//! assignments and no non-error early returns. Unsafe blocks, unsafe functions,
//! unsafe impls, and raw-pointer types are forbidden in state-bearing files for
//! the same reason: they can detach returned references from `AccountInfo`'s
//! `RefCell` guards.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use syn::visit::Visit;
use syn::{
    Block, Expr, FnArg, GenericArgument, ImplItem, ImplItemFn, ItemFn, ItemImpl,
    ItemMacro, ItemStruct, Pat, PathArguments, ReturnType, Signature, Stmt, Type,
    TypePath, TypePtr,
};

fn collect_rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read_dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            collect_rust_files(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

fn type_identity(value: &Type) -> String {
    if let Type::Path(type_path) = value {
        return type_path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
    }

    "<complex-impl-type>".to_string()
}

#[derive(Default)]
struct GuardedReferenceTypeVisitor {
    found: bool,
}

impl<'ast> Visit<'ast> for GuardedReferenceTypeVisitor {
    fn visit_type_path(&mut self, type_path: &'ast TypePath) {
        if type_path
            .path
            .segments
            .iter()
            .any(|segment| {
                let name = segment.ident.to_string();
                name == "Ref" || name == "RefMut"
            })
        {
            self.found = true;
        }

        syn::visit::visit_type_path(self, type_path);
    }
}

fn returns_guarded_reference(return_type: &ReturnType) -> bool {
    let ReturnType::Type(_, value) = return_type else {
        return false;
    };

    let mut visitor = GuardedReferenceTypeVisitor::default();
    visitor.visit_type(value);
    visitor.found
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum GuardedReturnShape {
    Other,
    Guarded,
    Tuple(Vec<Self>),
    Wrapper(Vec<Self>),
    Unsupported,
}

impl GuardedReturnShape {
    fn contains_guarded_reference(&self) -> bool {
        match self {
            Self::Other => false,
            Self::Guarded | Self::Unsupported => true,
            Self::Tuple(elements) | Self::Wrapper(elements) => elements
                .iter()
                .any(Self::contains_guarded_reference),
        }
    }
}

fn unsupported_shape_if_guarded(value: &Type) -> GuardedReturnShape {
    let mut visitor = GuardedReferenceTypeVisitor::default();
    visitor.visit_type(value);
    if visitor.found {
        GuardedReturnShape::Unsupported
    } else {
        GuardedReturnShape::Other
    }
}

fn type_guarded_return_shape(value: &Type) -> GuardedReturnShape {
    match value {
        Type::Path(type_path) => {
            let Some(segment) = type_path.path.segments.last() else {
                return unsupported_shape_if_guarded(value);
            };
            let name = segment.ident.to_string();
            if name == "Ref" || name == "RefMut" {
                return GuardedReturnShape::Guarded;
            }

            let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return unsupported_shape_if_guarded(value);
            };
            let children = arguments
                .args
                .iter()
                .filter_map(|argument| match argument {
                    GenericArgument::Type(argument_type) => {
                        Some(type_guarded_return_shape(argument_type))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();

            if name == "Result" {
                return children
                    .into_iter()
                    .next()
                    .unwrap_or(GuardedReturnShape::Other);
            }

            if children
                .iter()
                .any(GuardedReturnShape::contains_guarded_reference)
            {
                GuardedReturnShape::Wrapper(children)
            } else {
                GuardedReturnShape::Other
            }
        }
        Type::Tuple(tuple) => GuardedReturnShape::Tuple(
            tuple
                .elems
                .iter()
                .map(type_guarded_return_shape)
                .collect(),
        ),
        Type::Reference(reference) => type_guarded_return_shape(&reference.elem),
        Type::Paren(paren) => type_guarded_return_shape(&paren.elem),
        Type::Group(group) => type_guarded_return_shape(&group.elem),
        Type::Array(array) => {
            let child = type_guarded_return_shape(&array.elem);
            if child.contains_guarded_reference() {
                GuardedReturnShape::Wrapper(vec![child])
            } else {
                GuardedReturnShape::Other
            }
        }
        Type::Slice(slice) => {
            let child = type_guarded_return_shape(&slice.elem);
            if child.contains_guarded_reference() {
                GuardedReturnShape::Wrapper(vec![child])
            } else {
                GuardedReturnShape::Other
            }
        }
        _ => unsupported_shape_if_guarded(value),
    }
}

fn guarded_return_shape(return_type: &ReturnType) -> Option<GuardedReturnShape> {
    let ReturnType::Type(_, value) = return_type else {
        return None;
    };
    let shape = type_guarded_return_shape(value);

    if shape.contains_guarded_reference() {
        Some(shape)
    } else if returns_guarded_reference(return_type) {
        Some(GuardedReturnShape::Unsupported)
    } else {
        None
    }
}

#[derive(Default)]
struct AccountInfoTypeVisitor {
    found: bool,
}

impl<'ast> Visit<'ast> for AccountInfoTypeVisitor {
    fn visit_type_path(&mut self, type_path: &'ast TypePath) {
        if type_path
            .path
            .segments
            .iter()
            .any(|segment| segment.ident.to_string() == "AccountInfo")
        {
            self.found = true;
        }

        syn::visit::visit_type_path(self, type_path);
    }
}

fn accepts_account_info(signature: &Signature) -> bool {
    let mut visitor = AccountInfoTypeVisitor::default();

    for input in &signature.inputs {
        if let FnArg::Typed(argument) = input {
            visitor.visit_type(&argument.ty);
        }
    }

    visitor.found
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct LoaderKey {
    owner: Option<String>,
    name: String,
}

impl LoaderKey {
    fn label(&self) -> String {
        match &self.owner {
            Some(owner) => format!("{owner}::{}", self.name),
            None => self.name.clone(),
        }
    }
}

#[derive(Clone)]
struct LoaderCandidate {
    key:          LoaderKey,
    block:        Block,
    return_shape: GuardedReturnShape,
}

fn path_matches_state_loader(path: &syn::Path) -> bool {
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();

    segments.len() == 3
        && segments[0] == "crate"
        && segments[1] == "state_loader"
        && matches!(
            segments[2].as_str(),
            "load_state" | "load_state_mut" | "load_uninitialized_state_mut"
        )
}

fn call_target_key(path: &syn::Path, current: &LoaderKey) -> Option<LoaderKey> {
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();

    match segments.as_slice() {
        [name] => Some(LoaderKey {
            owner: None,
            name: name.clone(),
        }),
        [owner, name] if owner == "Self" => Some(LoaderKey {
            owner: current.owner.clone(),
            name: name.clone(),
        }),
        [.., owner, name] => Some(LoaderKey {
            owner: Some(owner.clone()),
            name: name.clone(),
        }),
        _ => None,
    }
}

fn expression_is_error_return(expression: &Expr) -> bool {
    match expression {
        Expr::Call(call) => matches!(
            call.func.as_ref(),
            Expr::Path(function_path)
                if function_path
                    .path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == "Err")
        ),
        Expr::Paren(value) => expression_is_error_return(&value.expr),
        Expr::Group(value) => expression_is_error_return(&value.expr),
        _ => false,
    }
}

fn expression_has_guarded_provenance(
    expression: &Expr,
    expected_shape: &GuardedReturnShape,
    locals: &HashMap<String, bool>,
    guarded_loaders: &HashSet<LoaderKey>,
    current: &LoaderKey,
) -> bool {
    match expression {
        Expr::Try(value) => expression_has_guarded_provenance(
            &value.expr,
            expected_shape,
            locals,
            guarded_loaders,
            current,
        ),
        Expr::Paren(value) => expression_has_guarded_provenance(
            &value.expr,
            expected_shape,
            locals,
            guarded_loaders,
            current,
        ),
        Expr::Group(value) => expression_has_guarded_provenance(
            &value.expr,
            expected_shape,
            locals,
            guarded_loaders,
            current,
        ),
        Expr::Return(value) => value.expr.as_deref().is_some_and(|returned| {
            expression_has_guarded_provenance(
                returned,
                expected_shape,
                locals,
                guarded_loaders,
                current,
            )
        }),
        Expr::Block(value) => block_returns_guarded_reference(
            &value.block,
            expected_shape,
            guarded_loaders,
            current,
        ),
        Expr::Call(call) => {
            let Expr::Path(function_path) = call.func.as_ref() else {
                return matches!(expected_shape, GuardedReturnShape::Other);
            };

            if function_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "Ok")
            {
                return call.args.len() == 1
                    && call.args.first().is_some_and(|argument| {
                        expression_has_guarded_provenance(
                            argument,
                            expected_shape,
                            locals,
                            guarded_loaders,
                            current,
                        )
                    });
            }

            if expected_shape.contains_guarded_reference()
                && (path_matches_state_loader(&function_path.path)
                    || call_target_key(&function_path.path, current)
                        .is_some_and(|target| guarded_loaders.contains(&target)))
            {
                return true;
            }

            match expected_shape {
                GuardedReturnShape::Other => true,
                GuardedReturnShape::Wrapper(children) if call.args.len() == children.len() => call
                    .args
                    .iter()
                    .zip(children)
                    .all(|(argument, child)| {
                        expression_has_guarded_provenance(
                            argument,
                            child,
                            locals,
                            guarded_loaders,
                            current,
                        )
                    }),
                _ => false,
            }
        }
        Expr::Tuple(tuple) => match expected_shape {
            GuardedReturnShape::Other => true,
            GuardedReturnShape::Tuple(children) if tuple.elems.len() == children.len() => tuple
                .elems
                .iter()
                .zip(children)
                .all(|(element, child)| {
                    expression_has_guarded_provenance(
                        element,
                        child,
                        locals,
                        guarded_loaders,
                        current,
                    )
                }),
            _ => false,
        },
        Expr::Path(value) if value.path.segments.len() == 1 => match expected_shape {
            GuardedReturnShape::Other => true,
            GuardedReturnShape::Guarded => value
                .path
                .segments
                .first()
                .and_then(|segment| locals.get(&segment.ident.to_string()))
                .copied()
                .unwrap_or(false),
            _ => false,
        },
        _ => matches!(expected_shape, GuardedReturnShape::Other),
    }
}

#[derive(Default)]
struct MacroInvocationCounter {
    count: usize,
}

impl<'ast> Visit<'ast> for MacroInvocationCounter {
    fn visit_macro(&mut self, value: &'ast syn::Macro) {
        self.count += 1;
        syn::visit::visit_macro(self, value);
    }
}

#[derive(Default)]
struct AssignmentCounter {
    count: usize,
}

impl<'ast> Visit<'ast> for AssignmentCounter {
    fn visit_expr_assign(&mut self, value: &'ast syn::ExprAssign) {
        self.count += 1;
        syn::visit::visit_expr_assign(self, value);
    }
}

#[derive(Default)]
struct ExplicitReturnCollector<'ast> {
    values: Vec<Option<&'ast Expr>>,
}

impl<'ast> Visit<'ast> for ExplicitReturnCollector<'ast> {
    fn visit_expr_return(&mut self, value: &'ast syn::ExprReturn) {
        self.values.push(value.expr.as_deref());
        syn::visit::visit_expr_return(self, value);
    }

    // A `return` inside a closure or nested item belongs to that inner body,
    // not to the loader being classified.
    fn visit_expr_closure(&mut self, _value: &'ast syn::ExprClosure) {}
    fn visit_item_fn(&mut self, _value: &'ast ItemFn) {}
    fn visit_impl_item_fn(&mut self, _value: &'ast ImplItemFn) {}
}

fn local_identifier(pattern: &Pat) -> Option<String> {
    match pattern {
        Pat::Ident(value) if value.ident.to_string() != "_" => Some(value.ident.to_string()),
        _ => None,
    }
}

fn block_returns_guarded_reference(
    block: &Block,
    return_shape: &GuardedReturnShape,
    guarded_loaders: &HashSet<LoaderKey>,
    current: &LoaderKey,
) -> bool {
    let mut macro_counter = MacroInvocationCounter::default();
    macro_counter.visit_block(block);
    if macro_counter.count != 0 {
        return false;
    }

    // Flow-sensitive assignment analysis is intentionally not approximated.
    // Reassignment inside a branch can otherwise replace a guarded reference
    // after the source gate has classified the original binding as safe.
    let mut assignment_counter = AssignmentCounter::default();
    assignment_counter.visit_block(block);
    if assignment_counter.count != 0 {
        return false;
    }

    let mut locals: HashMap<String, bool> = HashMap::new();

    for statement in &block.stmts {
        match statement {
            Stmt::Local(local) => {
                let Some(name) = local_identifier(&local.pat) else {
                    continue;
                };
                let guarded = local.init.as_ref().is_some_and(|initialization| {
                    expression_has_guarded_provenance(
                        &initialization.expr,
                        &GuardedReturnShape::Guarded,
                        &locals,
                        guarded_loaders,
                        current,
                    )
                });
                locals.insert(name, guarded);
            }
            Stmt::Expr(_, _) | Stmt::Item(_) | Stmt::Macro(_) => {}
        }
    }

    let mut explicit_returns = ExplicitReturnCollector::default();
    explicit_returns.visit_block(block);
    for returned in explicit_returns.values {
        let Some(expression) = returned else {
            return false;
        };

        if expression_is_error_return(expression) {
            continue;
        }

        // Non-error early returns require path-sensitive local provenance.
        // Keep loader wrappers in the auditable form used by this crate:
        // guarded locals plus a single tail return.
        return false;
    }

    let Some(Stmt::Expr(tail, None)) = block.stmts.last() else {
        return false;
    };

    expression_has_guarded_provenance(
        tail,
        return_shape,
        &locals,
        guarded_loaders,
        current,
    )
}

#[derive(Default)]
struct StateFileAnalysis {
    state_structs: usize,
    unsafe_blocks: usize,
    unsafe_functions: usize,
    unsafe_impls: usize,
    raw_pointer_types: usize,
    forbidden_item_macros: usize,
    unreviewed_attributes: usize,
    unreviewed_derives: usize,
    candidates: Vec<LoaderCandidate>,
    missing_loader_delegations: Vec<String>,
    current_impl: Option<String>,
}

fn guarded_loader_keys(candidates: &[LoaderCandidate]) -> HashSet<LoaderKey> {
    let mut guarded_loaders: HashSet<LoaderKey> = HashSet::new();

    loop {
        let mut changed = false;
        for candidate in candidates {
            if guarded_loaders.contains(&candidate.key) {
                continue;
            }

            if block_returns_guarded_reference(
                &candidate.block,
                &candidate.return_shape,
                &guarded_loaders,
                &candidate.key,
            ) {
                guarded_loaders.insert(candidate.key.clone());
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    guarded_loaders
}

impl StateFileAnalysis {
    fn apply_loader_provenance(&mut self, guarded_loaders: &HashSet<LoaderKey>) {
        self.missing_loader_delegations = self
            .candidates
            .iter()
            .filter(|candidate| !guarded_loaders.contains(&candidate.key))
            .map(|candidate| {
                format!(
                    "{} must return a reference derived from crate::state_loader",
                    candidate.key.label(),
                )
            })
            .collect();
    }

    fn finish_loader_provenance(&mut self) {
        let guarded_loaders = guarded_loader_keys(&self.candidates);
        self.apply_loader_provenance(&guarded_loaders);
    }

    fn security_relevant(&self) -> bool {
        self.state_structs != 0 || !self.candidates.is_empty()
    }
}

impl<'ast> Visit<'ast> for StateFileAnalysis {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        let attribute_name = attribute
            .path()
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_default();

        if attribute_name == "derive" {
            let parsed = attribute.parse_nested_meta(|meta| {
                let derive_name = meta
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string())
                    .unwrap_or_default();
                if !matches!(
                    derive_name.as_str(),
                    "BorshDeserialize"
                        | "BorshSerialize"
                        | "Clone"
                        | "Copy"
                        | "Debug"
                        | "Default"
                        | "Eq"
                        | "Error"
                        | "Hash"
                        | "PartialEq"
                        | "Pod"
                        | "Zeroable"
                ) {
                    self.unreviewed_derives += 1;
                }
                Ok(())
            });
            if parsed.is_err() {
                self.unreviewed_derives += 1;
            }
        } else if !matches!(
            attribute_name.as_str(),
            "allow"
                | "cfg"
                | "cfg_attr"
                | "cold"
                | "doc"
                | "error"
                | "inline"
                | "path"
                | "repr"
                | "test"
                | "unsafe"
        ) {
            self.unreviewed_attributes += 1;
        }

        syn::visit::visit_attribute(self, attribute);
    }

    fn visit_item_struct(&mut self, structure: &'ast ItemStruct) {
        if structure
            .attrs
            .iter()
            .any(|attribute| attribute.path().is_ident("repr"))
        {
            self.state_structs += 1;
        }

        syn::visit::visit_item_struct(self, structure);
    }

    fn visit_expr_unsafe(&mut self, expression: &'ast syn::ExprUnsafe) {
        self.unsafe_blocks += 1;
        syn::visit::visit_expr_unsafe(self, expression);
    }

    fn visit_item_impl(&mut self, implementation: &'ast ItemImpl) {
        if implementation.unsafety.is_some() {
            self.unsafe_impls += 1;
        }

        self.forbidden_item_macros += implementation
            .items
            .iter()
            .filter(|item| matches!(item, ImplItem::Macro(_)))
            .count();

        let previous = self.current_impl.replace(type_identity(&implementation.self_ty));
        syn::visit::visit_item_impl(self, implementation);
        self.current_impl = previous;
    }

    fn visit_item_macro(&mut self, item_macro: &'ast ItemMacro) {
        // Local macro definitions are analyzable source. Invocations capable of
        // generating items are not, and therefore cannot appear in the program
        // source tree covered by this release gate.
        if !item_macro.mac.path.is_ident("macro_rules") {
            self.forbidden_item_macros += 1;
        }
        syn::visit::visit_item_macro(self, item_macro);
    }

    fn visit_type_ptr(&mut self, pointer: &'ast TypePtr) {
        self.raw_pointer_types += 1;
        syn::visit::visit_type_ptr(self, pointer);
    }

    fn visit_impl_item_fn(&mut self, method: &'ast ImplItemFn) {
        if method.sig.unsafety.is_some() {
            self.unsafe_functions += 1;
        }

        if accepts_account_info(&method.sig) {
            if let Some(return_shape) = guarded_return_shape(&method.sig.output) {
                self.candidates.push(LoaderCandidate {
                    key: LoaderKey {
                        owner: self.current_impl.clone(),
                        name: method.sig.ident.to_string(),
                    },
                    block: method.block.clone(),
                    return_shape,
                });
            }
        }

        syn::visit::visit_impl_item_fn(self, method);
    }

    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        if function.sig.unsafety.is_some() {
            self.unsafe_functions += 1;
        }

        if accepts_account_info(&function.sig) {
            if let Some(return_shape) = guarded_return_shape(&function.sig.output) {
                self.candidates.push(LoaderCandidate {
                    key: LoaderKey {
                        owner: None,
                        name: function.sig.ident.to_string(),
                    },
                    block: (*function.block).clone(),
                    return_shape,
                });
            }
        }

        syn::visit::visit_item_fn(self, function);
    }
}

fn parse_state_source(source: &str) -> Result<StateFileAnalysis, syn::Error> {
    let syntax = syn::parse_file(source)?;
    let mut analysis = StateFileAnalysis::default();
    analysis.visit_file(&syntax);
    Ok(analysis)
}

fn analyze_state_source(source: &str) -> Result<StateFileAnalysis, syn::Error> {
    let mut analysis = parse_state_source(source)?;
    analysis.finish_loader_provenance();
    Ok(analysis)
}

#[test]
fn state_loaders_use_the_shared_guard_preserving_framework() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    let mut files = Vec::new();
    collect_rust_files(&crate_root, &mut files);
    files.sort();
    assert!(!files.is_empty(), "no Rust source files found");

    let trusted_files = [
        crate_root.join("state_loader.rs"),
        crate_root.join("state_owner_guard_lint.rs"),
    ];
    // These two crate-boundary files intentionally use audited platform macros.
    // Every other item-position macro invocation is rejected because this
    // source-AST gate cannot inspect the generated Rust items.
    let trusted_item_macro_files = [
        crate_root.join("entrypoint.rs"),
        crate_root.join("lib.rs"),
    ];

    let mut parsed_files: Vec<(PathBuf, StateFileAnalysis)> = Vec::new();
    for file in &files {
        if trusted_files.iter().any(|trusted| file == trusted) {
            continue;
        }

        let source = fs::read_to_string(file).expect("read Rust source file");
        let analysis = parse_state_source(&source)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", file.display()));
        parsed_files.push((file.clone(), analysis));
    }

    let all_candidates = parsed_files
        .iter()
        .flat_map(|(_, analysis)| analysis.candidates.iter().cloned())
        .collect::<Vec<_>>();
    let candidate_count = all_candidates.len();

    let mut key_counts: HashMap<LoaderKey, usize> = HashMap::new();
    for candidate in &all_candidates {
        *key_counts.entry(candidate.key.clone()).or_default() += 1;
    }
    let ambiguous_keys = key_counts
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(key, count)| format!("{} ({count} definitions)", key.label()))
        .collect::<Vec<_>>();
    assert!(
        ambiguous_keys.is_empty(),
        "state-loader gate found ambiguous loader identities: {}",
        ambiguous_keys.join(", "),
    );

    // Resolve wrappers across the whole crate. This is required for canonical
    // authorization helpers that return a state reference loaded by the state
    // type's defining module.
    let guarded_loaders = guarded_loader_keys(&all_candidates);

    let mut offenders = Vec::new();
    for (file, mut analysis) in parsed_files {
        analysis.apply_loader_provenance(&guarded_loaders);

        let forbidden_item_macros = analysis.forbidden_item_macros != 0
            && !trusted_item_macro_files.iter().any(|trusted| &file == trusted);
        if forbidden_item_macros
            || analysis.unreviewed_attributes != 0
            || analysis.unreviewed_derives != 0
            || (analysis.security_relevant()
                && (analysis.unsafe_blocks != 0
                    || analysis.unsafe_functions != 0
                    || analysis.unsafe_impls != 0
                    || analysis.raw_pointer_types != 0
                    || !analysis.missing_loader_delegations.is_empty()))
        {
            offenders.push(format!(
                "{}: unsafe_blocks={}, unsafe_functions={}, unsafe_impls={}, raw_pointer_types={}, item_macros={}, unreviewed_attributes={}, unreviewed_derives={}, missing=[{}]",
                file.display(),
                analysis.unsafe_blocks,
                analysis.unsafe_functions,
                analysis.unsafe_impls,
                analysis.raw_pointer_types,
                analysis.forbidden_item_macros,
                analysis.unreviewed_attributes,
                analysis.unreviewed_derives,
                analysis.missing_loader_delegations.join(", "),
            ));
        }
    }

    assert!(
        candidate_count >= 100,
        "state-loader gate found only {candidate_count} guarded-reference loaders; source coverage unexpectedly collapsed",
    );
    assert!(
        offenders.is_empty(),
        "state files bypass the syntax-aware state-loader boundary:\n{}",
        offenders.join("\n"),
    );
}

#[test]
fn arbitrary_loader_names_and_locations_are_checked() {
    let source = r#"
        struct Example;
        impl Example {
            pub fn borrow_state<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                unchecked_borrow(account)
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.candidates.len(), 1);
    assert_eq!(analysis.missing_loader_delegations.len(), 1);
}

#[test]
fn returned_value_must_derive_from_the_guarded_loader() {
    let source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub fn borrow_state<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                let _ = crate::state_loader::load_state::<Self>(account, 1, [0; 8], error)?;
                unchecked_borrow(account)
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.missing_loader_delegations.len(), 1);
}

#[test]
fn direct_loader_and_guarded_wrapper_provenance_are_accepted() {
    let source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub fn borrow_state<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                let state = crate::state_loader::load_state::<Self>(
                    account, 1, [0; 8], error,
                )?;
                Ok(state)
            }

            pub fn borrow_verified<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                let state = Self::borrow_state(account)?;
                if state.version == 0 {
                    return Err(error.into());
                }
                Ok(state)
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.candidates.len(), 2);
    assert!(analysis.missing_loader_delegations.is_empty());
}

#[test]
fn tuple_metadata_wrapper_provenance_is_accepted() {
    let source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub fn load<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                crate::state_loader::load_state::<Self>(account, 1, [0; 8], error)
            }

            pub fn load_with_bump<'a>(account: &'a AccountInfo<'a>)
                -> Result<(core::cell::Ref<'a, Self>, u8), ProgramError>
            {
                let state = Self::load(account)?;
                let bump = 7u8;
                Ok((state, bump))
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.candidates.len(), 2);
    assert!(analysis.missing_loader_delegations.is_empty());
}

#[test]
fn every_guarded_tuple_component_requires_guarded_provenance() {
    let source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub fn load<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                crate::state_loader::load_state::<Self>(account, 1, [0; 8], error)
            }

            pub fn load_pair<'a>(account: &'a AccountInfo<'a>)
                -> Result<(
                    core::cell::Ref<'a, Self>,
                    core::cell::Ref<'a, Self>,
                ), ProgramError>
            {
                let guarded = Self::load(account)?;
                let unchecked = unchecked_borrow(account)?;
                Ok((guarded, unchecked))
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.candidates.len(), 2);
    assert_eq!(analysis.missing_loader_delegations, vec![
        "Example::load_pair must return a reference derived from crate::state_loader".to_string(),
    ]);
}

#[test]
fn private_and_cross_file_wrapper_candidates_participate_in_global_provenance() {
    let state_source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub fn load<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                crate::state_loader::load_state::<Self>(account, 1, [0; 8], error)
            }
        }
    "#;
    let wrapper_source = r#"
        fn private_wrapper<'a>(account: &'a AccountInfo<'a>)
            -> Result<core::cell::Ref<'a, Example>, ProgramError>
        {
            Example::load(account)
        }

        pub fn public_wrapper<'a>(account: &'a AccountInfo<'a>)
            -> Result<core::cell::Ref<'a, Example>, ProgramError>
        {
            private_wrapper(account)
        }
    "#;

    let mut state_analysis = parse_state_source(state_source).expect("parse state source");
    let mut wrapper_analysis = parse_state_source(wrapper_source).expect("parse wrapper source");
    let candidates = state_analysis
        .candidates
        .iter()
        .chain(wrapper_analysis.candidates.iter())
        .cloned()
        .collect::<Vec<_>>();
    let guarded = guarded_loader_keys(&candidates);
    state_analysis.apply_loader_provenance(&guarded);
    wrapper_analysis.apply_loader_provenance(&guarded);

    assert!(state_analysis.missing_loader_delegations.is_empty());
    assert!(wrapper_analysis.missing_loader_delegations.is_empty());
}

#[test]
fn unsafe_function_signatures_are_detected() {
    let source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub unsafe fn borrow_state<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                crate::state_loader::load_state::<Self>(account, 1, [0; 8], error)
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.unsafe_functions, 1);
}

#[test]
fn macro_generated_items_and_loader_bodies_are_rejected() {
    let item_source = r#"
        impl_state_loader!(Example);
    "#;
    let item_analysis = analyze_state_source(item_source).expect("parse item macro source");
    assert_eq!(item_analysis.forbidden_item_macros, 1);
    assert!(!item_analysis.security_relevant());

    let body_source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub fn borrow_state<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                guarded_loader!(account)
            }
        }
    "#;
    let body_analysis = analyze_state_source(body_source).expect("parse body macro source");
    assert_eq!(body_analysis.missing_loader_delegations.len(), 1);
}

#[test]
fn assignments_cannot_replace_a_guarded_return_value() {
    let source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub fn borrow_state<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                let mut state = crate::state_loader::load_state::<Self>(
                    account, 1, [0; 8], error,
                )?;
                if condition() {
                    state = unchecked_borrow(account)?;
                }
                Ok(state)
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.missing_loader_delegations.len(), 1);
}

#[test]
fn non_error_early_returns_are_rejected_without_path_sensitive_provenance() {
    let source = r#"
        #[repr(C)]
        struct Example;
        impl Example {
            pub fn borrow_state<'a>(account: &'a AccountInfo<'a>)
                -> Result<core::cell::Ref<'a, Self>, ProgramError>
            {
                let state = crate::state_loader::load_state::<Self>(
                    account, 1, [0; 8], error,
                )?;
                if condition() {
                    return Ok(state);
                }
                Ok(state)
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.missing_loader_delegations.len(), 1);
}

#[test]
fn unreviewed_attribute_and_derive_macros_are_rejected() {
    let source = r#"
        #[generate_state_loader]
        #[derive(GenerateStateLoader)]
        struct Example;
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.unreviewed_attributes, 1);
    assert_eq!(analysis.unreviewed_derives, 1);
}

#[test]
fn unsafe_blocks_impls_and_raw_pointer_types_are_detected() {
    let source = r#"
        #[repr(C)]
        struct Example;
        unsafe impl Trait for Example {}
        impl Example {
            pub fn other(pointer: *const u8) {
                unsafe { core::ptr::read(pointer); }
            }
        }
    "#;
    let analysis = analyze_state_source(source).expect("parse synthetic source");

    assert_eq!(analysis.unsafe_blocks, 1);
    assert_eq!(analysis.unsafe_impls, 1);
    assert_eq!(analysis.raw_pointer_types, 1);
}
