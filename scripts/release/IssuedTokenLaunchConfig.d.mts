import type { Buffer } from "node:buffer";

export interface IssuedTokenLaunchMetadata {
    readonly name: "USDv";
    readonly symbol: "USDv";
    readonly uri: "https://soletechltd.org/assets/usdv.json";
    readonly offchain_json_sha256: string;
}

export interface IssuedTokenAuthorityPosture {
    readonly mode: "chancery_pda_reserved";
    readonly metadata_updates_require_chancery_upgrade: true;
    readonly extension_authorities_reserved_for_future_use: true;
}

export interface IssuedTokenLaunchConfig {
    readonly schema_version: 1;
    readonly token_program: typeof expectedIssuedTokenProgram;
    readonly decimals: typeof expectedIssuedTokenDecimals;
    readonly metadata: IssuedTokenLaunchMetadata;
    readonly authority_posture: IssuedTokenAuthorityPosture;
    readonly reserved_extensions: readonly string[];
}

export interface IssuedTokenLaunchConfigSource {
    readonly path: string;
    readonly source: Buffer;
    readonly config: IssuedTokenLaunchConfig;
    readonly configSha256: string;
}

export interface IssuedTokenOffchainMetadata {
    readonly observedHash: string;
    readonly document: {
        readonly name: "USDv";
        readonly symbol: "USDv";
    };
}

export declare const issuedTokenLaunchConfigPath: "artifacts/issued-token-launch.json";
export declare const expectedIssuedTokenProgram: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
export declare const expectedIssuedTokenDecimals: 6;
export declare const issuedTokenMetadataTimeoutMs: 15000;
export declare const issuedTokenMetadataMaximumBytes: 262144;

export declare function validateIssuedTokenLaunchConfig(config: unknown): IssuedTokenLaunchConfig;
export declare function readIssuedTokenLaunchConfig(root?: string): Promise<IssuedTokenLaunchConfigSource>;
export declare function verifyIssuedTokenOffchainMetadata(
    config: IssuedTokenLaunchConfig,
    fetchImplementation?: typeof globalThis.fetch,
): Promise<IssuedTokenOffchainMetadata>;
