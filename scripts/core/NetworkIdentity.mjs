const BASE58_PUBLIC_KEY_PATTERN = /^[1-9A-HJ-NP-Za-km-z]+$/u;
const NETWORK_PROGRAM_PATTERN = /programId:\s*"([1-9A-HJ-NP-Za-km-z]+)"/u;
const RUST_PROGRAM_PATTERN = /declare_id!\("([1-9A-HJ-NP-Za-km-z]+)"\);/gu;
const CLIENT_PROGRAM_PATTERN = /PROGRAM_ID = new PublicKey\("([1-9A-HJ-NP-Za-km-z]+)"\)/u;

function requiredSingleMatch(source, pattern, sourceName) {
    const matches = [...source.matchAll(pattern)];
    if (matches.length !== 1) {
        throw new Error(`${sourceName} must contain exactly one program identity, got ${String(matches.length)}`);
    }
    return matches[0][1];
}

function requiredMatch(source, pattern, sourceName) {
    const match = pattern.exec(source);
    if (match === null) throw new Error(`${sourceName} is missing`);
    return match[1];
}

export function requireBase58PublicKeyText(value, sourceName) {
    if (typeof value !== "string" || !BASE58_PUBLIC_KEY_PATTERN.test(value)) {
        throw new Error(`${sourceName} must be base58 public-key text`);
    }
    return value;
}

export function parseRustProgramIdentity(source, sourceName) {
    return requireBase58PublicKeyText(
        requiredSingleMatch(source, RUST_PROGRAM_PATTERN, sourceName),
        `${sourceName} program identity`
    );
}

export function replaceRustProgramIdentity(source, programId, sourceName) {
    parseRustProgramIdentity(source, sourceName);
    const validatedProgramId = requireBase58PublicKeyText(programId, "replacement Rust program identity");
    return source.replace(/declare_id!\("[1-9A-HJ-NP-Za-km-z]+"\);/u, `declare_id!("${validatedProgramId}");`);
}

export function parseClientProgramIdentity(source, sourceName) {
    return requireBase58PublicKeyText(
        requiredMatch(source, CLIENT_PROGRAM_PATTERN, sourceName),
        `${sourceName} program identity`
    );
}

export function replaceClientProgramIdentity(source, programId, sourceName) {
    parseClientProgramIdentity(source, sourceName);
    const validatedProgramId = requireBase58PublicKeyText(programId, "replacement client program identity");
    return source.replace(
        /PROGRAM_ID = new PublicKey\("[1-9A-HJ-NP-Za-km-z]+"\)/u,
        `PROGRAM_ID = new PublicKey("${validatedProgramId}")`
    );
}

export function replaceNetworkConfigProgramIdentity(source, programId, sourceName) {
    requiredMatch(source, NETWORK_PROGRAM_PATTERN, `${sourceName} programId`);
    const validatedProgramId = requireBase58PublicKeyText(programId, "replacement network program identity");
    return source.replace(
        /programId:\s*"[1-9A-HJ-NP-Za-km-z]+"/u,
        `programId: "${validatedProgramId}"`
    );
}

export function replaceNetworkConfigFaucetIdentity(source, programId, sourceName) {
    requiredSingleMatch(source, /faucetProgramId:\s*"([1-9A-HJ-NP-Za-km-z]+)"/gu, sourceName + " faucetProgramId");
    const replacement = requireBase58PublicKeyText(programId, "replacement faucet identity");
    return source.replace(/faucetProgramId:\s*"[1-9A-HJ-NP-Za-km-z]+"/u, 'faucetProgramId: "' + replacement + '"');
}

export function replaceNetworkConfigIssuedMintIdentity(source, mint, sourceName) {
    const pattern = /\busdvMint:\s*(null|"[1-9A-HJ-NP-Za-km-z]+")/gu;
    const current = requiredSingleMatch(source, pattern, sourceName + " usdvMint");
    const replacement = requireBase58PublicKeyText(mint, "verified issued mint");
    if (current !== "null" && current !== JSON.stringify(replacement)) {
        throw new Error(sourceName + " usdvMint already identifies a different issued mint");
    }
    return source.replace(/\busdvMint:\s*(null|"[1-9A-HJ-NP-Za-km-z]+")/u, 'usdvMint: "' + replacement + '"');
}
