export function requireBase58PublicKeyText(value: unknown, sourceName: string): string;
export function parseRustProgramIdentity(source: string, sourceName: string): string;
export function replaceRustProgramIdentity(source: string, programId: string, sourceName: string): string;
export function parseClientProgramIdentity(source: string, sourceName: string): string;
export function replaceClientProgramIdentity(source: string, programId: string, sourceName: string): string;
export function replaceNetworkConfigProgramIdentity(source: string, programId: string, sourceName: string): string;
export function replaceNetworkConfigFaucetIdentity(source: string, programId: string, sourceName: string): string;
export function replaceNetworkConfigIssuedMintIdentity(source: string, mint: string, sourceName: string): string;
