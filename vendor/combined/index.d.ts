import { after } from 'node:test';
import { afterEach } from 'node:test';
import { AssertionError } from 'node:assert';
import { BaseRestClient } from '@solomon-labs/rest-client';
import { before } from 'node:test';
import { beforeEach } from 'node:test';
import { BigDecimal } from '@solomon-labs/bigdecimal';
import { describe } from 'node:test';
import type { EndpointConfig } from '@solomon-labs/http-client';
import { ExplorerLinkType } from '@solomon-labs/types-enums/explorer';
import { HttpClient } from '@solomon-labs/http-client';
import { it } from 'node:test';
import { LogLevel } from '@solomon-labs/types-enums/logging';
import type { MessageInstruction as MessageInstruction_2 } from '../../../transactions/src';
import type { Mock } from 'node:test';
import { mock } from 'node:test';
import type { ObservabilityFeatureEventsFromEnum } from '@solomon-labs/observability';
import type { ObservabilityLeaf } from '@solomon-labs/observability';
import type { ObservabilityStats } from '@solomon-labs/observability';
import { PublicKeySymbol } from '@solomon-labs/types-symbols/publicKey';
import type { RateLimitConfig } from '@solomon-labs/http-client';
import type { Readable } from 'stream';
import type { RestClientConfig } from '@solomon-labs/rest-client';
import type { RetryConfig } from '@solomon-labs/http-client';
import { run } from 'node:test';
import type { RuntimeEnvironment } from '@solomon-labs/types-enums/environment';
import type { SchemaObject } from '@solomon-labs/types-globals/openapi';
import { SolanaExplorer } from '@solomon-labs/types-enums/solana';
import { SolanaNetwork } from '@solomon-labs/types-enums/solana';
import type { WebsocketResponse } from '@solomon-labs/websocket';

export declare type AbstractConstructor<T = unknown> = abstract new (...args: never[]) => T;

export declare const ACCEPT_ALL: "*/*";

export declare const ACCEPT_ENCODING_GZIP_DEFLATE: "gzip, deflate";

export declare const ACCEPT_ENCODING_GZIP_DEFLATE_BR: "gzip, deflate, br";

export declare const ACCEPT_JSON: "application/json, application/*+json";

export declare const ACCEPT_LANGUAGE_EN: "en";

export declare const ACCEPT_LANGUAGE_EN_US: "en-US,en;q=0.8";

export declare const ACCESS_DENIED_MESSAGE: "access denied";

export declare type AccessToken<S extends string = string> = Tagged<ApiKey<S>, "AccessToken">;

export declare const ACCOUNT_SIZE_MAP: Record<string, number>;

export declare type AccountDataResult = TokenMintDataResult | TokenMint2022DataResult | TokenAccountDataResult | TokenAccount2022DataResult | UnknownAccountDataResult;

export declare enum AccountDataType {
    TokenMint = "mint",
    TokenAccount = "token_account",
    TokenMint2022 = "token_mint_2022",
    TokenAccount2022 = "token_account_2022",
    Unknown = "unknown"
}

/**
 * Thrown when message account key index is out of bounds
 */
export declare class AccountIndexOutOfBoundsError extends MessageError {
    readonly accountKey: SolanaAddress;
    readonly index: number;
    readonly maxIndex: number;
    readonly type = "ACCOUNT_INDEX_OUT_OF_BOUNDS";
    constructor(message: string | undefined, accountKey: SolanaAddress, index: number, maxIndex: number, originalError?: unknown);
}

export declare interface AccountInfo {
    data: [Base64 | null, string] | null;
    executable: boolean;
    lamports: number;
    owner: string;
    rentEpoch: number;
}

export declare interface AccountNotification {
    context: {
        slot: SolanaSlot;
    };
    value: AccountInfo;
}

export declare interface AccountRelationMapping {
    account: string;
    field: string;
}

export declare interface AccountRelationsForAccount {
    relations?: readonly string[];
    xRelations?: readonly AccountRelationMapping[];
}

export declare type AccountRelationsSchema = Record<string, AccountRelationsForAccount>;

declare interface AccountResponse<ResponseValue> {
    context: {
        apiVersion: string;
        slot: SolanaSlot;
    };
    value: ResponseValue;
}

export declare interface AccountSubscribeConfig {
    commitment?: SolanaCommitmentLevel;
    encoding?: "base58" | "base64" | "base64+zstd" | "jsonParsed";
}

export declare class AddressArrayItem implements IsCodable {
    address: PublicKeyLike;
    constructor(properties: EncodableProps<AddressArrayItem>);
    static decode(data: string | Uint8Array | Buffer): AddressArrayItem;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare interface AddressLookupTableAccount {
    key: PublicKeyLike;
    state: {
        authority?: PublicKeyLike;
        addresses: PublicKeyLike[];
        lastExtendedSlot: number;
        lastExtendedSlotStartIndex: number;
        deactivationSlot: bigint;
    };
}

/**
 * Thrown when address lookup table is not found
 */
export declare class AddressLookupTableNotFoundError extends MessageError {
    readonly accountKey?: SolanaAddress | undefined;
    readonly type = "ADDRESS_LOOKUP_TABLE_NOT_FOUND";
    constructor(message?: string, accountKey?: SolanaAddress | undefined, originalError?: unknown);
}

/**
 * Thrown when address lookup tables not supplied but required
 */
export declare class AddressLookupTablesRequiredError extends MessageError {
    readonly version: number | string;
    readonly type = "ADDRESS_LOOKUP_TABLES_REQUIRED";
    constructor(message: string | undefined, version: number | string, originalError?: unknown);
}

/** Address */
export declare type AddressSpan = ["address", LabelValueRole];

export declare interface AddressTableLookup {
    accountKey: SolanaAddress;
    writableIndexes: number[];
    readonlyIndexes: number[];
}

export declare type AddressTableLookupMode = "generate" | "external" | "merge";

export declare type AddTag<T, // string *or* already-tagged value
Label extends PropertyKey, Meta = never> = ExtractStringFromTagged<T> & // the raw `"foo"` literal
Tag<ExtractStringFromTagged<T>, Label, Meta>;

export declare type AdminId = Tagged<PrimaryId, "AdminId">;

/**
 * Advance Nonce Account instruction
 */
export declare class AdvanceNonceAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    nonceAccount: SolanaAddressLike;
    recentBlockhashesSysvar: SolanaAddressLike & EncodableDefault;
    nonceAuthority: SolanaAddressLike;
    systemProgram: SolanaAddressLike & EncodableDefault;
    constructor(props: EncodableProps<AdvanceNonceAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): AdvanceNonceAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare type Aes256Key = Tagged<EncryptionKey, "Aes256Key">;

export declare type Aes512Key = Tagged<EncryptionKey, "Aes512Key">;

export { after }

export declare const AFTER_HOURS_SESSION: AfterHoursSession;

export { afterEach }

export declare type AfterHoursSession = MarketSession<"After Hours">;

export declare interface AggregatedInstructionAccount extends InstructionAccount {
    isFeePayer: boolean;
}

declare type AliasOf<P extends string, Alias extends StringMap> = P extends keyof Alias ? Alias[P] : P;

declare type AllCharsInSet<S extends string, Allowed extends string> = S extends `${infer H}${infer R}` ? H extends Allowed ? AllCharsInSet<R, Allowed> : false : true;

/**
 * 8 = Allocate { space: u64 }
 */
export declare class AllocateInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    space: bigint | number | string;
    constructor(props: EncodableProps<AllocateInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): AllocateInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/**
 * 9 = AllocateWithSeed { base: Pubkey, seed: String, space: u64, owner: Pubkey }
 */
export declare class AllocateWithSeedInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    base: PublicKeyLike;
    seed: string;
    space: bigint | number | string;
    owner: PublicKeyLike;
    constructor(props: EncodableProps<AllocateWithSeedInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): AllocateWithSeedInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

declare type AlphaNumeric = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

export declare const ANCHOR_DISCRIMINATOR_SIZE: 8;

export declare interface AnchorCpiLog extends BaseLog {
    type: "anchorEvent";
    programId: PublicKey_2;
    data: Uint8Array;
    decoded: IsLogDecodable | null;
}

/**
 * Anchor discriminator helpers
 */
export declare type AnchorDiscriminatorNamespace = "account" | "global" | "state" | "event";

/**
 * Anchor Event CPI Instruction
 *
 * This instruction represents events emitted via the emit_cpi!() macro in Anchor programs.
 * The event data is included in the instruction data as a self-CPI, making it more reliable
 * than program logs which can be truncated.
 *
 * Format:
 * - Discriminator: 0x1d9acb512ea545e4 (8 bytes, little-endian)
 * - Event Data: Borsh-serialized event struct
 *
 * Note: The discriminator is Sha256("anchor:event")[..8] in little-endian format.
 * In hex: e445a52e51cb9a1d (big-endian representation)
 */
export declare class AnchorEventCpiInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    /**
     * Raw event data (borsh-serialized bytes after discriminator)
     */
    eventData: Uint8Array;
    constructor(props: EncodableProps<AnchorEventCpiInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): AnchorEventCpiInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare interface AnchorEventCpiInstructionEvent extends CanonicalOperation {
    type: "instruction";
    parsed: ParsedInstruction;
    decoded: AnchorEventCpiInstruction;
}

export declare type AnchorParsedLog = ProgramInstructionLog | ProgramAnchorErrorLog | AnchorCpiLog;

export declare const ANDROID: OperatingSystem<"Android">;

export declare type AnyEconomicEvent = CPIReleaseEvent | PPIReleaseEvent | NFPReleaseEvent | PMIReleaseEvent | FomcDecisionEvent | FomcMinutesReleaseEvent | FedChairPressConferenceEvent | InitialJoblessClaimsEvent | GDPReleaseEvent | RetailSalesReleaseEvent | JacksonHoleSymposium;

export declare type AnyHoliday = WesternHoliday | UnitedStatesHoliday | AustraliaHolday;

export declare type AnyLegacyMarket = NYSE_2 | NASDAQ_2 | CME_2 | CBOE_2 | ICE_2 | LSE_2 | XETR_2 | ASX_2 | HKEX_2 | SSE_2 | SZSE_2 | SGX_2 | JPX_2;

export declare type AnySingleProperty = {
    [K in string]: SingleProperty<K, unknown>;
}[string];

export declare const ANZAC_DAY: ANZACDay;

export declare type ANZACDay = Holiday<"ANZAC Day">;

export declare type ApiCredential<S extends string = string> = Tagged<S, "ApiCredential">;

export declare type ApiKey<S extends string = string> = Tagged<ApiCredential<S>, "ApiKey">;

export declare interface ApiResponse<T = unknown> {
    success: boolean;
    data?: T;
    message?: string;
    error?: string;
}

export declare type ApiSecret<S extends string = string> = Tagged<ApiCredential<S>, "ApiSecret">;

export declare type ApplicationMimeType = "application/json" | "application/xml" | "application/x-www-form-urlencoded" | "application/octet-stream" | "application/pdf" | "application/zip" | "application/gzip" | "application/javascript";

/** 13 = ApproveChecked { amount: u64, decimals: u8 } */
export declare class ApproveCheckedInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    source: SolanaAddressLike;
    mint: SolanaAddressLike;
    delegate: SolanaAddressLike;
    owner: SolanaAddressLike;
    amount: bigint | number | string;
    decimals: number;
    constructor(props: EncodableProps<ApproveCheckedInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): ApproveCheckedInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/** 4 = Approve { amount: u64 } */
export declare class ApproveInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    source: SolanaAddressLike;
    delegate: SolanaAddressLike;
    owner: SolanaAddressLike;
    amount: bigint | number | string;
    constructor(props: EncodableProps<ApproveInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): ApproveInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare type ArrayElementFromIndex<T> = T extends {
    [n: number]: infer E;
} ? E : never;

export declare function arraysEqual<T extends Primitive>(a: readonly T[] | T[], b: readonly T[] | T[]): boolean;

export declare function arraysLooselyEqual<T extends Primitive>(a: readonly T[] | T[], b: readonly T[] | T[]): boolean;

/**
 * Validates and converts unknown value to finite number with optional fallback
 */
export declare function asNumber(value: unknown): number | undefined;

export declare function asNumber<T extends number>(value: unknown, fallback: T): number | T;

export declare const assert: AssertObject;

/**
 * Use this to check if an import i.e. JSON fits a given expected shape.
 * Import attributes i.e. with { type: "json" } doesn't have syntax to attach a TS type
 * so use this assertion to shape it instead.
 */
export declare type AssertAssignable<From, To extends From> = true;

/**
 * Assert that the values of an enum match that of a union
 */
export declare type AssertEnumMatchesUnion<TEnum extends number, TUnion extends number> = [TEnum] extends [TUnion] ? [TUnion] extends [TEnum] ? true : never : never;

export declare function assertInstanceOf<Thing>(item: unknown, thing: new (...args: any[]) => Thing): item is Thing;

export declare function assertInstanceOfAbstract(item: unknown, thing: Function): boolean;

export { AssertionError }

declare type AssertIsNot<T, Excluded> = T extends Excluded ? never : Exclude<T, Excluded>;

export declare function assertLength(item: {
    length: number;
}, item2: number | {
    length: number;
}): void;

export declare type AssertLooselyAssignable<From, To extends Widen<From>> = true;

declare interface AssertObject {
    (value: unknown, message?: string | Error): asserts value;
    ok(value: unknown, message?: string | Error): asserts value;
    equal<T, Expected extends T>(actual: T, expected: Expected, message?: string | Error): asserts actual is Expected;
    notEqual<T, Excluded>(actual: T, expected: Excluded, message?: string | Error): asserts actual is AssertIsNot<T, Excluded>;
    strictEqual<T, Expected extends T>(actual: T, expected: Expected, message?: string | Error): asserts actual is Expected;
    notStrictEqual<T, Excluded>(actual: T, expected: Excluded, message?: string | Error): asserts actual is AssertIsNot<T, Excluded>;
    greaterThan<T extends number>(leftOperand: T, rightOperand: T, message?: string | Error): void;
    greaterThanOrEqual<T extends number>(leftOperand: T, rightOperand: T, message?: string | Error): void;
    lessThan<T extends number>(leftOperand: T, rightOperand: T, message?: string | Error): void;
    lessThanOrEqual<T extends number>(leftOperand: T, rightOperand: T, message?: string | Error): void;
    deepEqual<T, Expected extends T>(actual: T, expected: Expected, message?: string | Error): asserts actual is Expected;
    looselyEqual<T, Expected extends T>(actual: T, expected: Expected, message?: string | Error, ignoreExtraProps?: boolean): asserts actual is Expected;
    notDeepEqual<T, Excluded>(actual: T, expected: Excluded, message?: string | Error): asserts actual is AssertIsNot<T, Excluded>;
    deepStrictEqual<T, Expected extends T>(actual: T, expected: Expected, message?: string | Error): asserts actual is Expected;
    notDeepStrictEqual<T, Excluded>(actual: T, expected: Excluded, message?: string | Error): asserts actual is AssertIsNot<T, Excluded>;
    throws(block: () => unknown, error?: RegExp | Function | Error | string, message?: string | Error): void;
    throwsInstanceOf(block: () => unknown, ErrorConstructor: Constructor<Error, any>, message?: string): void;
    doesNotThrow(block: () => unknown, error?: RegExp | Function | Error | string, message?: string | Error): void;
    rejects(block: (() => Promise<unknown>) | Promise<unknown>, error?: RegExp | Function | Error | string, message?: string | Error): Promise<void>;
    doesNotReject(block: (() => Promise<unknown>) | Promise<unknown>, error?: RegExp | Function | Error | string, message?: string | Error): Promise<void>;
    ifError(value: unknown): asserts value is null | undefined;
    match(string: string, regexp: RegExp, message?: string | Error): void;
    doesNotMatch(string: string, regexp: RegExp, message?: string | Error): void;
    fail(message?: string | Error): never;
    includes<T, C extends string | Array<T> | Set<T> | Map<T, unknown>>(container: C, item: T, message?: string | Error): void;
    AssertionError: typeof AssertionError;
}

export declare type AssetId<T extends string = string> = Tagged<ValidAssetIdPattern<T>, "AssetId">;

export declare type AssetName<T extends string = string> = Tagged<ValidAssetNamePattern<T>, "AssetName">;

export declare type AssetSymbol<T extends string = string> = Tagged<ValidAssetSymbolPattern<T>, "AssetSymbol">;

/**
 * 1 = Assign { owner: Pubkey }
 */
export declare class AssignInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    owner: PublicKeyLike;
    constructor(props: EncodableProps<AssignInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): AssignInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/**
 * 10 = AssignWithSeed { base: Pubkey, seed: String, owner: Pubkey }
 */
export declare class AssignWithSeedInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    base: PublicKeyLike;
    seed: string;
    owner: PublicKeyLike;
    constructor(props: EncodableProps<AssignWithSeedInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): AssignWithSeedInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID_STRING: AssociatedTokenAccountProgram;

export declare type AssociatedTokenAccountProgram = SolanaAddressInfo<"TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", {
    systemProgram: true;
    name: "Associated Token Account Program";
    description: "Associated Token Account SPL Program";
}>;

export declare const ASX: ASX_2;

declare type ASX_2 = ExchangeName<"ASX">;

export declare type ASXId = ExchangeId<ASX_2>;

export declare type AsyncFunction = Function & ((...args: any[]) => Promise<any>);

export declare function asyncRunner(func: () => Promise<void>, { trackSockets, trackTimers, cleanupOptions, testTimeout, isolation, concurrency }?: AsyncRunnerOptions): () => Promise<void>;

declare interface AsyncRunnerOptions {
    readonly cleanupOptions?: CleanupOptions;
    readonly trackSockets?: boolean;
    readonly trackTimers?: boolean;
    readonly testTimeout?: number | false;
    readonly isolation?: "process" | "none" | undefined;
    readonly concurrency?: number | boolean | undefined;
}

/** ATA Address */
export declare type AtaSpan = ["ata", LabelValueRole];

export declare const AU: AU_2;

declare type AU_2 = CountryCode<"AU">;

export declare type AudioMimeType = "audio/mpeg" | "audio/wav" | "audio/ogg";

export declare const AUSTRALIA_DAY: AustraliaDay;

export declare type AustraliaDay = Holiday<"Australia Day">;

export declare type AustraliaHolday = AustraliaDay | ANZACDay | BoxingDay | KingsBirthday | LabourDay;

export declare const AUTH_API_KEY: "ApiKey";

export declare const AUTH_BASIC: "Basic";

export declare const AUTH_BEARER: "Bearer";

export declare const AUTH_DIGEST: "Digest";

export declare const AUTHENTICATION_FAILED_MESSAGE: "authentication failed";

export declare type AuthorizationType = "Basic" | "Bearer" | "Digest" | "ApiKey";

/**
 * 7 = AuthorizeNonceAccount { authorized: Pubkey }
 */
export declare class AuthorizeNonceAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    nonceAccount: SolanaAddressLike;
    nonceAuthority: SolanaAddressLike;
    authorized: PublicKeyLike;
    constructor(props: EncodableProps<AuthorizeNonceAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): AuthorizeNonceAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const BACKPACK_WALLET_URL: Url<"https://backpack.app/">;

export declare const BAD_REQUEST_MESSAGE: "bad request";

/** End badge i.e. end of transaction */
export declare type BadgeEndSpan = ["badge", "end"];

/** Start badge, i.e. beginning of a transaction */
export declare type BadgeStartSpan = ["badge", "start"];

export declare interface BalanceMapEntry {
    mintAddress: PublicKey;
    ownerAddress: PublicKey | null;
    uiAmount: BigDecimal;
    decimals: number;
}

export declare type Base58<S extends string = string, Min extends number = number, Max extends number | undefined = undefined> = Tagged<Encoded & ValidBase58<S, Min, Max>, "Base58Encoded">;

declare type Base58Char = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "J" | "K" | "L" | "M" | "N" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z";

/**
 * Base class for Base58-related errors
 */
export declare abstract class Base58Error extends SolomonLabsError {
    readonly category = "BASE58";
}

export declare type Base58LikeInput<S extends string = string, Min extends number = number, Max extends number | undefined = undefined> = Base58<S, Min, Max> | LiteralBase58<S, Min, Max>;

declare type _Base58OK<S extends string, Min extends number, Max extends number | undefined> = ValidString<S, Base58Char, Min, Max>;

export declare class Base58Util {
    private static throwInvalidBase58CharacterString;
    private static decodeExpectedByteLengthString;
    private static decodeExpectedByteLengthBuffer;
    private static decodeString;
    static decode<S extends string>(str: Base58LikeInput<S, 1>): Uint8Array;
    static decodePrefix<S extends string>(str: Base58LikeInput<S, 1>, maxBytes: number): Uint8Array | null;
    private static _toBase58;
    static ensureBase58(input: string): Base58;
    static encodeString<EncodedType extends Base58 = Base58>(input: UTF8): EncodedType;
    static encode<EncodedType extends Base58 = Base58>(bytes: Uint8Array): EncodedType;
    static isValidBase58Chars(str: string): boolean;
    static isValid(input: string): input is Base58;
    private static isValidPublicKeyLength;
    static isValidPublicKey(input: string): input is SolanaAddress;
    static tryToDecodePublicKey(input: string): Uint8Array;
    private static isValidSignatureLength;
    static isValidSignature(input: string): input is SolanaTransactionSignature;
    static decodeFast64ByteFromBuffer(buffer: Uint8Array, offset: number, length: number): Uint8Array | null;
    static decodeFast32ByteFromBuffer(buffer: Uint8Array, offset: number, length: number): Uint8Array | null;
    static hasPrefix<S extends string>(str: Base58LikeInput<S, 1>, prefix: Uint8Array, prefixIsNonZero?: boolean): boolean;
}

export declare type Base64<S extends string = string, Min extends number = number, Max extends number | undefined = undefined> = Tagged<Encoded & ValidBase64<S, Min, Max>, "Base64Encoded">;

declare type Base64Char = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "+" | "/" | "=";

export declare type Base64Ish = Base64 | Base64Url;

declare type _Base64OK<S extends string, Min extends number, Max extends number | undefined> = ValidString<S, Base64Char, Min, Max>;

export declare type Base64Url<S extends string = string, Min extends number = number, Max extends number | undefined = undefined> = Tagged<Encoded & ValidBase64Url<S, Min, Max>, "Base64UrlEncoded">;

declare type Base64UrlChar = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "-" | "_";

declare type _Base64UrlOK<S extends string, Min extends number, Max extends number | undefined> = ValidString<S, Base64UrlChar, Min, Max>;

export declare interface BaseCalendarEntry {
    id: CalendarEntryId;
    name: string;
    date: Date;
    allDay?: boolean;
    notes?: string;
    type: CalendarEntryType;
}

declare interface BaseInstructionEvent extends CanonicalOperation {
    type: "instruction";
    parsed: ParsedInstruction;
    decoded: IsEncodable | null;
}

declare interface BaseLog extends OrderedOperation {
    log: string;
    type: SolanaLogType;
    subType?: SolanaLogSubType;
    data?: Uint8Array | string | null;
}

export declare type BaseParsedLog = InvokeLog | SuccessLog | FailedLog | ConsumedLog | ReturnLog | ProgramLog | DataLog | TruncatedLog;

export declare type BaseSolanaLogType = "invoke" | "success" | "failed" | "consumed" | "return" | "log" | "data";

declare type BaseTypeMap = {
    [SchemaFieldType.Address]: PublicKeyLike;
    [SchemaFieldType.U8]: U8;
    [SchemaFieldType.U16]: U16;
    [SchemaFieldType.U24]: U24;
    [SchemaFieldType.U32]: U32;
    [SchemaFieldType.BU8]: BU8;
    [SchemaFieldType.BU16]: BU16;
    [SchemaFieldType.BU24]: BU24;
    [SchemaFieldType.BU32]: BU32;
    [SchemaFieldType.U64]: BU64;
    [SchemaFieldType.U128]: BU128;
    [SchemaFieldType.I8]: I8;
    [SchemaFieldType.I16]: I16;
    [SchemaFieldType.I24]: I24;
    [SchemaFieldType.I32]: I32;
    [SchemaFieldType.BI8]: BI8;
    [SchemaFieldType.BI16]: BI16;
    [SchemaFieldType.BI24]: BI24;
    [SchemaFieldType.BI32]: BI32;
    [SchemaFieldType.I64]: BI64;
    [SchemaFieldType.I128]: BI128;
    [SchemaFieldType.Boolean]: boolean;
    [SchemaFieldType.FixedBytes]: Uint8Array;
    [SchemaFieldType.String]: string;
    [SchemaFieldType.Bytes]: Uint8Array;
};

export declare type BBytesBE = "bu8-be" | "bu16-be" | "bu24-be" | "bu32-be" | "bu64-be" | "bu128-be" | "bu256-be" | "bi8-be" | "bi16-be" | "bi24-be" | "bi32-be" | "bi64-be" | "bi128-be" | "bi256-be";

export declare type BBytesLE = "bu8" | "bu16" | "bu24" | "bu32" | "bu64" | "bu128" | "bu256" | "bi8" | "bi16" | "bi24" | "bi32" | "bi64" | "bi128" | "bi256";

export declare type BcryptHash = Tagged<Hash, "BcryptHash">;

export declare type BearerToken<S extends string = string> = Tagged<ApiKey<S>, "BearerToken">;

export { before }

export { beforeEach }

/** Between Min and Max (inclusive).
 *  If Max is **undefined** or the wide type **number**, we treat it as “no upper bound”. */
declare type Between<L extends number, Min extends number, Max extends number | undefined = undefined> = Gte<L, Min> extends true ? Max extends number ? number extends Max ? true : Lte<L, Extract<Max, number>> : true : false;

export declare type BI128<bint128 extends bigint = bigint> = Tagged<bint128, "bint128">;

export declare type BI128Bytes = Bytes<"bi128">;

export declare type BI16<bint16 extends bigint = bigint> = Tagged<bint16, "bint16">;

export declare type BI16Bytes = Bytes<"bi16">;

export declare type BI24<bint24 extends bigint = bigint> = Tagged<bint24, "bint24">;

export declare type BI24Bytes = Bytes<"bi24">;

export declare type BI256<bint256 extends bigint = bigint> = Tagged<bint256, "bint256">;

export declare type BI256Bytes = Bytes<"bi256">;

export declare type BI32<bint32 extends bigint = bigint> = Tagged<bint32, "bint32">;

export declare type BI32Bytes = Bytes<"bi32">;

export declare type BI64<bint64 extends bigint = bigint> = Tagged<bint64, "bint64">;

export declare type BI64Bytes = Bytes<"bi64">;

export declare type BI8<bint8 extends bigint = bigint> = Tagged<bint8, "bint8">;

export declare type BI8Bytes = Bytes<"bi8">;

export declare const BigDecimalBrand: unique symbol;

declare type BigSignedScalarMap = {
    "8": BI8;
    "16": BI16;
    "24": BI24;
    "32": BI32;
    "64": BI64;
    "128": BI128;
    "256": BI256;
};

declare type BigSignedWidth = "8" | "16" | "24" | "32" | "64" | "128" | "256";

declare type BigUnsignedScalarMap = {
    "8": BU8;
    "16": BU16;
    "24": BU24;
    "32": BU32;
    "64": BU64;
    "128": BU128;
    "256": BU256;
};

declare type BigUnsignedWidth = "8" | "16" | "24" | "32" | "64" | "128" | "256";

export declare const BINANCE: Binance;

export declare type Binance = ExchangeName<"Binance">;

export declare const BINANCE_INVERSE_FUTURES_API_URL: Url<"https://dapi.binance.com">;

export declare const BINANCE_LINEAR_FUTURES_API_URL: Url<"https://fapi.binance.com">;

export declare const BINANCE_SPOT_DATA_API_URL: Url<"https://data-api.binance.vision">;

export declare type BinanceId = ExchangeId<Binance>;

/**
 * Binary transaction format version
 */
export declare const BINARY_TRANSACTION_FORMAT_VERSION = 1;

/**
 * Fixed header size in bytes (version 1)
 */
export declare const BINARY_TRANSACTION_HEADER_SIZE = 28;

/**
 * Binary address table lookup format
 * [account_key_index:1][writable_count:1][readonly_count:1][writable_indices:variable][readonly_indices:variable]
 */
export declare interface BinaryAddressTableLookup {
    accountKey: SolanaAddress;
    writableIndexes: number[];
    readonlyIndexes: number[];
}

/**
 * Binary canonical event (union type)
 */
export declare type BinaryCanonicalEvent = BinaryInstructionEvent | BinaryLogEvent;

/**
 * Binary inner instruction format
 * [index:2][instruction_count:2][instructions:variable]
 */
export declare interface BinaryInnerInstruction {
    index: number;
    instructions: BinaryInstruction[];
}

/**
 * Binary instruction format
 * [program_id_index:1][account_count:2][data_length:2][accounts:variable][data:variable]
 */
export declare interface BinaryInstruction {
    programIdIndex: number;
    accountIndices: number[];
    data: Uint8Array;
}

/**
 * Binary instruction event structure
 */
export declare interface BinaryInstructionEvent {
    type: CanonicalEventType.INSTRUCTION;
    ordering: CanonicalEventOrdering;
    programId: Uint8Array;
    accountCount: number;
    accounts: Uint8Array[];
    data: Uint8Array;
    hasDecoded: boolean;
    decodedType?: string;
}

/**
 * Binary log event structure
 */
export declare interface BinaryLogEvent {
    type: CanonicalEventType.LOG;
    ordering: CanonicalEventOrdering;
    logType: CanonicalLogType;
    programId: Uint8Array | null;
    logString: string;
    data: Uint8Array | string | null;
    hasDecoded: boolean;
    decodedType?: string;
}

/**
 * Binary token balance format
 * [account_index:2][mint:variable][owner:variable][amount:variable][decimals:1][ui_amount_string:variable]
 */
export declare interface BinaryTokenBalance {
    accountIndex: number;
    mint: SolanaAddress;
    owner: SolanaAddress | null | undefined;
    uiTokenAmount: {
        amount: string;
        decimals: number;
        uiAmountString: string | null | undefined;
    };
}

/**
 * Binary transaction structure
 */
export declare interface BinaryTransaction {
    header: BinaryTransactionHeader;
    signatures: Uint8Array[];
    accountKeys: Array<{
        bytes: Uint8Array;
        base58: SolanaAddress;
    }>;
    recentBlockhash: Uint8Array | null;
    instructions: BinaryInstruction[];
    addressTableLookups: BinaryAddressTableLookup[];
    meta: BinaryTransactionMeta | null;
}

/**
 * Binary transaction header structure
 */
export declare interface BinaryTransactionHeader {
    formatVersion: number;
    headerSize: number;
    txVersion: number;
    signatureCount: number;
    accountKeysCount: number;
    instructionsCount: number;
    addressTableLookupsCount: number;
    hasMeta: number;
    blockTime: number | null;
    slot: number | null;
    bodySize: number;
}

/**
 * Binary transaction meta format
 * [has_err:1][fee:8][pre_balances_count:2][post_balances_count:2][inner_instructions_count:2][log_messages_count:2][pre_token_balances_count:2][post_token_balances_count:2][has_compute_units:1][loaded_addresses_count:1][...variable data...]
 */
export declare interface BinaryTransactionMeta {
    err: unknown | null;
    fee: number | null;
    preBalances: number[];
    postBalances: number[];
    innerInstructions: BinaryInnerInstruction[];
    logMessages: string[];
    preTokenBalances: BinaryTokenBalance[];
    postTokenBalances: BinaryTokenBalance[];
    computeUnitsConsumed: number | null;
    loadedAddresses: {
        readonly: SolanaAddress[];
        writable: SolanaAddress[];
    } | null;
}

export declare const BINGX: BingX;

export declare type BingX = ExchangeName<"BingX">;

export declare type BingXId = ExchangeId<BingX>;

export declare const BITCOIN: Bitcoin;

export declare type Bitcoin = AssetId<"bitcoin">;

export declare const BITCOIN_NAME: BitcoinName;

export declare const BITCOIN_SYMBOL: BitcoinSymbol;

export declare type BitcoinName = AssetName<"Bitcoin">;

export declare type BitcoinSymbol = AssetSymbol<"BTC">;

export declare const BITFINEX: Bitfinex;

export declare type Bitfinex = ExchangeName<"Bitfinex">;

export declare type BitfinexId = ExchangeId<Bitfinex>;

export declare const BITGET: Bitget;

export declare type Bitget = ExchangeName<"Bitget">;

export declare type BitgetId = ExchangeId<Bitget>;

export declare const BITMEX: BitMEX;

export declare type BitMEX = ExchangeName<"BitMEX">;

export declare type BitMEXId = ExchangeId<BitMEX>;

declare type BitWidthFromCore<Core extends string> = Core extends `u${infer W}` ? W : Core extends `i${infer W}` ? W : Core extends `bu${infer W}` ? W : Core extends `bi${infer W}` ? W : never;

export declare type BivariantSetter<T> = {
    bivarianceHack(value: T): void;
}["bivarianceHack"];

export declare type BlockchainProtocolFeatureName = BlockchainProtocolName<string> & FeatureName;

export declare type BlockchainProtocolId<T extends string> = Tagged<ProtocolId<T>, "BlockchainProtocolId">;

export declare type BlockchainProtocolName<T extends string = string> = Tagged<ProtocolName<T>, "BlockchainProtocolName">;

/**
 * Blockhash length in bytes (32 bytes)
 */
export declare const BLOCKHASH_BYTE_LENGTH = 32;

export declare interface BlockSubscribeConfig {
    commitment?: SolanaCommitmentLevel;
    encoding?: "base58" | "base64" | "base64+zstd" | "json" | "jsonParsed";
    transactionDetails?: "full" | "signatures" | "accounts" | "none";
    maxSupportedTransactionVersion?: number;
    showRewards?: boolean;
}

export declare type BlockSubscribeFilter = "all" | {
    mentionsAccountOrProgram: SolanaAddress;
};

/** Block Time */
export declare type BlockTimeSpan = ["blockTime", LabelValueRole];

export declare type BNB = AssetId<"bnb">;

export declare const BNB_ASSET: BNB;

export declare const BNB_NAME: BNBName;

export declare const BNB_SYMBOL: BNBSymbol;

export declare type BNBName = AssetName<"BNB">;

export declare type BNBSymbol = AssetSymbol<"BNB">;

export declare const BOXING_DAY: BoxingDay;

export declare type BoxingDay = Holiday<"Boxing Day">;

export declare type BrandedBigDecimal = BrandedInterface<typeof BigDecimalBrand>;

/**
 * Generic "brand-marker" that adds a unique-symbol key.
 *  – `B` **must** be a `unique symbol`.
 *  – The resulting type has a readonly property whose key is that symbol,
 *    with the default value `true`.
 * Allows for compile-time only branding of objects, classes and interfaces.
 * Where `T` is supplied we are able to extract this to infer `T`.
 */
export declare type BrandedInterface<B extends symbol, T extends any = true> = {
    readonly [K in B]: T;
};

export declare type BrandedPublicKey = BrandedInterface<typeof PublicKeyBrand>;

export declare type BrandedRelationEntity<EntityClass> = BrandedInterface<typeof RelationEntityBrand, EntityClass>;

export declare type BrandedRelationInterface<EntityInterface> = BrandedInterface<typeof RelationInterfaceBrand, EntityInterface>;

export declare type BrandedRepository = BrandedInterface<typeof EntityRepositoryBrand>;

export declare type BrandedSerialized<T> = BrandedInterface<typeof SerialzedBrand, T>;

export declare type BrandedTransformable = BrandedInterface<typeof TransformableBrand>;

export declare const BRAVE: Browser<"Brave">;

/** Explicit line break (renderer decides how to represent). */
export declare type BreakOp = 4;

export declare type Browser<S extends string = string> = Tagged<S, "Browser">;

export declare type BU128<buint32 extends bigint = bigint> = Tagged<buint32, "buint32">;

export declare type BU128Bytes = Bytes<"bu128">;

export declare type BU16<buint16 extends bigint = bigint> = Tagged<buint16, "buint16">;

export declare type BU16Bytes = Bytes<"bu16">;

export declare type BU24<buint24 extends bigint = bigint> = Tagged<buint24, "buint24">;

export declare type BU24Bytes = Bytes<"bu24">;

export declare type BU256<buint256 extends bigint = bigint> = Tagged<buint256, "buint256">;

export declare type BU256Bytes = Bytes<"bu256">;

export declare type BU32<buint32 extends bigint = bigint> = Tagged<buint32, "buint32">;

export declare type BU32Bytes = Bytes<"bu32">;

export declare type BU64<buint64 extends bigint = bigint> = Tagged<buint64, "buint64">;

export declare type BU64Bytes = Bytes<"bu64">;

export declare type BU8<buint8 extends bigint = bigint> = Tagged<buint8, "buint8">;

export declare type BU8Bytes = Bytes<"bu8">;

/**
 * Error thrown when conversion between formats fails
 */
export declare class BufferConversionError extends BufferError {
    readonly fromFormat?: string | undefined;
    readonly toFormat?: string | undefined;
    readonly type = "CONVERSION_ERROR";
    constructor(message: string, fromFormat?: string | undefined, toFormat?: string | undefined, originalError?: unknown);
}

/**
 * Error thrown when encoding/decoding fails
 */
export declare class BufferEncodingError extends BufferError {
    readonly encoding: string;
    readonly inputValue?: unknown | undefined;
    readonly type = "ENCODING_ERROR";
    constructor(message: string, encoding: string, inputValue?: unknown | undefined, originalError?: unknown);
}

/**
 * Error thrown when environment doesn't support required operations
 */
export declare class BufferEnvironmentError extends BufferError {
    readonly missingFeature?: string | undefined;
    readonly type = "ENVIRONMENT_ERROR";
    constructor(message: string, missingFeature?: string | undefined, originalError?: unknown);
}

/**
 * Base class for BufferUtil-related errors
 */
export declare abstract class BufferError extends SolomonLabsError {
    readonly category = "BUFFER_UTIL";
}

/**
 * Error thrown when invalid input is provided
 */
export declare class BufferInputError extends BufferError {
    readonly providedValue?: unknown | undefined;
    readonly type = "INVALID_INPUT";
    constructor(message: string, providedValue?: unknown | undefined, originalError?: unknown);
}

export declare type BufferLikeInput<S extends string = string, Min extends number = number, Max extends number = Min> = HexLower<S, Min, Max> | Base58<S, Min, Max> | LiteralHexLower<S, Min, Max> | LiteralBase58<S, Min, Max> | UTF8 | Uint8Array | Buffer | NumericTupleBetween<number, Min, Max> | NumericTupleBetween<number, Min, Max> | ReadonlyUint8Array;

export declare class BufferUtil {
    private static textEncoder;
    private static textDecoder;
    static readonly hasBuffer: boolean;
    static readonly hasAtob: boolean;
    static readonly hasTextCoders: boolean;
    static readonly hasToBase64: boolean;
    private static getTextEncoder;
    private static getTextDecoder;
    /**
     * Convert string to Uint8Array (replaces Buffer.from(string, encoding))
     */
    static fromString<S extends string>(str: Base64 | LiteralBase64<S, 1>, encoding: "base64"): Uint8Array;
    static fromString<S extends string>(str: Base64Url | LiteralBase64Url<S, 1>, encoding: "base64url"): Uint8Array;
    static fromString(str: Base64Ish, encoding: "base64ish"): Uint8Array;
    static fromString(str: UTF8 | string, encoding: "utf8"): Uint8Array;
    static fromString<S extends string>(str: HexLower | LiteralHexLower<S, 2>, encoding: "hex"): Uint8Array;
    static fromString(str: UTF8 | string, encoding?: "utf8"): Uint8Array;
    /**
     * Convert Uint8Array to string (replaces buffer.toString(encoding))
     */
    static toString(bytes: Uint8Array, encoding: "base64"): Base64;
    static toString(bytes: Uint8Array, encoding: "base64url"): Base64Url;
    static toString(bytes: Uint8Array, encoding: "utf8"): UTF8;
    static toString(bytes: Uint8Array, encoding: "hex"): HexLower;
    static toString(bytes: Uint8Array, encoding?: "utf8"): UTF8;
    static toString(bytes: Uint8Array, encoding: "base64" | "base64url" | "utf8" | "hex"): string;
    /**
     * Convert one string format to another directly
     * Can also operate directly on a preloaded Uint8Array
     */
    static convertString<S extends string>(str: Base64Url | LiteralBase64Url<S, 1> | Uint8Array, sourceEncoding: "base64url", destEncoding: "utf8"): UTF8;
    static convertString<S extends string>(str: Base64Url | LiteralBase64Url<S, 1> | Uint8Array, sourceEncoding: "base64url", destEncoding: "base64"): Base64;
    static convertString<S extends string>(str: Base64Url | LiteralBase64Url<S, 1> | Uint8Array, sourceEncoding: "base64url", destEncoding: "hex"): HexLower;
    static convertString<S extends string>(str: Base64 | LiteralBase64<S, 1> | Uint8Array, sourceEncoding: "base64", destEncoding: "utf8"): UTF8;
    static convertString<S extends string>(str: Base64 | LiteralBase64<S, 1> | Uint8Array, sourceEncoding: "base64", destEncoding: "base64url"): Base64Url;
    static convertString<S extends string>(str: Base64 | LiteralBase64<S, 1> | Uint8Array, sourceEncoding: "base64", destEncoding: "hex"): HexLower;
    static convertString(str: Base64Ish | Uint8Array, sourceEncoding: "base64ish", destEncoding: "utf8"): UTF8;
    static convertString(str: Base64Ish | Uint8Array, sourceEncoding: "base64ish", destEncoding: "base64"): Base64;
    static convertString(str: Base64Ish | Uint8Array, sourceEncoding: "base64ish", destEncoding: "base64url"): Base64Url;
    static convertString(str: Base64Ish | Uint8Array, sourceEncoding: "base64ish", destEncoding: "hex"): HexLower;
    static convertString<S extends string>(str: HexLower | LiteralHexLower<S, 2> | Uint8Array, sourceEncoding: "hex", destEncoding: "utf8"): UTF8;
    static convertString<S extends string>(str: HexLower | LiteralHexLower<S, 2> | Uint8Array, sourceEncoding: "hex", destEncoding: "base64url"): Base64Url;
    static convertString<S extends string>(str: HexLower | LiteralHexLower<S, 2> | Uint8Array, sourceEncoding: "hex", destEncoding: "base64"): Base64;
    static convertString(str: UTF8, sourceEncoding: "utf8", destEncoding: "base64"): Base64;
    static convertString(str: UTF8, sourceEncoding: "utf8", destEncoding: "base64url"): Base64Url;
    static convertString(str: UTF8, sourceEncoding: "utf8", destEncoding: "hex"): HexLower;
    /**
     * Concatenate multiple byte arrays (replaces Buffer.concat())
     */
    static concat(arrays: (Uint8Array | Buffer | ArrayBuffer)[]): Uint8Array;
    /**
     * Check if value is a Buffer-like object
     */
    static isBuffer(value: unknown): value is Buffer | Uint8Array;
    /**
     * Convert any buffer-like input to Uint8Array
     */
    static toUint8Array<S extends string>(input: BufferLikeInput<S> | readonly number[] | string, encoding?: "base64" | "base64url" | "utf8" | "hex"): Uint8Array;
    /**
     * Trim trailing 0x00 padding from a Uint8Array view.
     */
    static rtrimZeros(view: Uint8Array | Buffer): Uint8Array;
    /**
     * Compare two buffer-like objects for equality
     */
    static equals(a: Uint8Array | Buffer, b: Uint8Array | Buffer): boolean;
    private static looksLikeBase64;
    private static looksLikeHex;
}

declare type Build<N extends number, R extends any[] = []> = R["length"] extends N ? R : Build<N, [any, ...R]>;

export declare type BuildTuple<T, L extends number, Acc extends readonly unknown[] = []> = Acc["length"] extends L ? Acc : BuildTuple<T, L, [...Acc, T]>;

/** 15 = BurnChecked { amount: u64, decimals: u8 } */
export declare class BurnCheckedInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    mint: SolanaAddressLike;
    owner: SolanaAddressLike;
    amount: bigint | number | string;
    decimals: number;
    constructor(props: EncodableProps<BurnCheckedInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): BurnCheckedInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/** 8 = Burn { amount: u64 } */
export declare class BurnInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    mint: SolanaAddressLike;
    owner: SolanaAddressLike;
    amount: bigint | number | string;
    constructor(props: EncodableProps<BurnInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): BurnInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const BYBIT: ByBit;

export declare type ByBit = ExchangeName<"ByBit">;

export declare type ByBitId = ExchangeId<ByBit>;

export declare type ByteBuffer<ByteEncoding extends BytesType = BytesType> = Tagged<ArrayBuffer, ByteEncoding>;

/**
 * Given a BytesType encoding, compute the fixed byte length.
 */
export declare type ByteLengthForEncoding<Encoding extends BytesType> = ByteLengthFromBitWidth<BitWidthFromCore<StripEndian_2<Encoding>>>;

declare type ByteLengthFromBitWidth<W extends string> = W extends "8" ? 1 : W extends "16" ? 2 : W extends "24" ? 3 : W extends "32" ? 4 : W extends "64" ? 8 : W extends "128" ? 16 : W extends "256" ? 32 : never;

export declare type Bytes<ByteEncoding extends BytesType = BytesType> = Uint8Array<ByteBuffer<ByteEncoding>>;

export declare type Bytes32 = Tuple<number, 32>;

export declare type BytesBE = "u8-be" | "u16-be" | "u24-be" | "u32-be" | "i8-be" | "i16-be" | "i24-be" | "i32-be";

export declare type BytesEncodingFromScalar<ScalarType> = ScalarTagName<ScalarType> extends keyof ScalarTagToBytesEncoding ? ScalarTagToBytesEncoding[ScalarTagName<ScalarType>] : never;

export declare type BytesForScalar<ScalarType> = Bytes<BytesEncodingFromScalar<ScalarType>>;

export declare type BytesLE = "u8" | "u16" | "u24" | "u32" | "i8" | "i16" | "i24" | "i32";

export declare type BytesType = BytesLE | BytesBE | BBytesLE | BBytesBE;

export declare type BytesView<ByteEncoding extends BytesType = BytesType> = DataView<ByteBuffer<ByteEncoding>>;

export declare const CAC40: CAC40Name;

export declare type CAC40Name = EquityIndexName<"CAC 40">;

export declare type CAC40Symbol = EquityIndexSymbol<"PX1">;

export declare const CACHE_MAX_AGE: "max-age";

export declare const CACHE_MUST_REVALIDATE: "must-revalidate";

export declare const CACHE_NO_CACHE: "no-cache";

export declare const CACHE_NO_STORE: "no-store";

export declare const CACHE_PRIVATE: "private";

export declare const CACHE_PUBLIC: "public";

export declare type CacheControlDirective = "no-cache" | "no-store" | "must-revalidate" | "public" | "private" | "max-age";

export declare type CalendarEntry = HolidayEntry | EconomicEventEntry | QuarterEndEntry | OptionsExpiryEntry | MarketSessionEntry;

export declare type CalendarEntryId<S extends string = string> = Tagged<S, "CalendarEntryId">;

export declare type CalendarEntryType = "Holiday" | "Economic" | "QuarterEnd" | "OptionsExpiry" | "Session";

export declare type CalendarEventImpact = Exclude<EventImpact, "None" | "Very Low" | "Very High" | "Extreme" | "Full">;

export declare type CallbackName = Tagged<string, "CallbackName">;

export declare type CamelCase<T extends string = string> = Tagged<ValidCamelCasePattern<T>, "CamelCase">;

/**
 * Canonical event format version
 */
export declare const CANONICAL_EVENT_FORMAT_VERSION = 1;

/**
 * Fixed header size in bytes (version 1)
 */
export declare const CANONICAL_EVENT_HEADER_SIZE = 8;

/**
 * Binary canonical event header
 */
export declare interface CanonicalEventHeader {
    formatVersion: number;
    eventCount: number;
}

/**
 * Canonical event ordering information
 */
export declare interface CanonicalEventOrdering {
    ordinal: number;
    stackIndex: number;
    index: number;
    parentIndex: number;
    parentOrdinal: number;
    inferred: boolean;
}

/**
 * Event type discriminator
 */
export declare enum CanonicalEventType {
    INSTRUCTION = 0,
    LOG = 1
}

/**
 * Log type discriminator
 */
export declare enum CanonicalLogType {
    INVOKE = 0,
    SUCCESS = 1,
    FAILED = 2,
    CONSUMED = 3,
    RETURN = 4,
    LOG = 5,
    DATA = 6,
    ANCHOR_EVENT = 7,
    TRUNCATED = 8,
    ORPHAN = 9
}

export declare interface CanonicalOperation extends OrderedOperation {
    parentOrdinal: number;
}

export declare function capitalizeFirst(str: string): string;

export declare const CBOE: CBOE_2;

declare type CBOE_2 = ExchangeName<"CBOE">;

export declare type CBOEId = ExchangeId<CBOE_2>;

export declare type ChaCha20Key = Tagged<EncryptionKey, "ChaCha20Key">;

declare type Chars<S extends string, A extends any[] = []> = S extends `${infer _Head}${infer Tail}` ? Chars<Tail, [any, ...A]> : A;

/**
 * Adds a test that checks for hanging handles/requests after all tests complete.
 * Call this in your test files to detect timers, sockets, file descriptors, or other resources
 * that might prevent the process from exiting cleanly.
 */
export declare function checkForHangingHandles(): void;

export declare function checkForUnexpectedObjects(): void;

export declare const CHICAGO_TIMEZONE: ChicagoTimezone;

export declare type ChicagoTimezone = IANATimeZone<"America/Chicago">;

export declare const CHRISTMAS_DAY: ChristmasDay;

export declare type ChristmasDay = Holiday<"Christmas Day">;

export declare const CHROME: Browser<"Chrome">;

export declare const CHROME_OS: OperatingSystem<"Chrome OS">;

export declare const CIRCUIT_BREAKER_HALT_SESSION: CircuitBreakerHaltSession;

export declare type CircuitBreakerHaltSession = MarketSession<"Circuit Breaker Halt">;

declare type CleanChar<C extends string> = C extends " " ? "-" : Lowercase<C> extends LowercaseAlphaNumeric ? Lowercase<C> : "";

export declare function cleanupAfterTests(testStream: Readable, options?: CleanupOptions): Promise<void>;

export declare interface CleanupOptions {
    readonly resumeStream?: boolean;
    readonly cleanupStream?: boolean;
    readonly cleanupDelay?: number;
    readonly verbose?: boolean;
    readonly killChildProcesses?: boolean;
    readonly unrefHandles?: boolean;
}

export declare const CLIENT_FEATURE: FeatureName<"Client">;

export declare const CLOSE_WINDOW_MS: DurationMs<5000>;

/** 9 = CloseAccount {} */
export declare class CloseAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    destination: SolanaAddressLike;
    owner: SolanaAddressLike;
    constructor(props: EncodableProps<CloseAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): CloseAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const CLOSING_AUCTION_SESSION: ClosingAuctionSession;

export declare type ClosingAuctionSession = MarketSession<"Closing Auction">;

export declare const CLOUDFLARE_TRACE_ENDPOINT: Url<"https://cloudflare.com/cdn-cgi/trace">;

export declare const CME: CME_2;

declare type CME_2 = ExchangeName<"CME">;

export declare type CMEId = ExchangeId<CME_2>;

export declare const COINBASE: Coinbase;

export declare type Coinbase = ExchangeName<"Coinbase">;

export declare type CoinbaseId = ExchangeId<Coinbase>;

export declare const COLUMBUS_DAY: ColumbusDay;

export declare type ColumbusDay = Holiday<"Columbus Day">;

/** Common global reusable spans */
export declare type CommonGlobalSpans = [
EqualsSpan,
TextSpan,
MetaSpan,
ProgramSpan,
MintSpan,
AtaSpan,
OwnerSpan,
AddressSpan,
SlotSpan,
BlockTimeSpan,
SignatureSpan,
KindSpan,
FrameStartSpan,
FrameEndSpan,
BadgeStartSpan,
BadgeEndSpan,
DepthSpan,
IndexSpan,
TypeSpan,
SubtypeSpan
];

export declare interface CompatibleMessageArgs {
    /** The message header, identifying signed and read-only `accountKeys` */
    header: TransactionMessageHeader;
    /** All the account keys used by this transaction */
    accountKeys: string[] | PublicKeyLike[];
    /** The hash of a recent ledger block */
    recentBlockhash: SolanaBlockhash;
    /** Instructions that will be executed in sequence and committed in one atomic transaction if all succeed. */
    instructions: CompiledInstruction[];
}

export declare interface CompatibleMessageCompiledInstruction {
    /** Index into the transaction keys array indicating the program account that executes this instruction */
    programIdIndex: number;
    /** Ordered indices into the transaction keys array indicating which accounts to pass to the program */
    accountKeyIndexes: number[];
    /** The program input data */
    data?: Uint8Array | Base58 | null | undefined;
}

export declare interface CompatibleMessageV0Args {
    /** The message header, identifying signed and read-only `accountKeys` */
    header: TransactionMessageHeader;
    /** The static account keys used by this transaction */
    staticAccountKeys: string[] | PublicKeyLike[];
    /** The hash of a recent ledger block */
    recentBlockhash: SolanaBlockhash;
    /** Instructions that will be executed in sequence and committed in one atomic transaction if all succeed. */
    compiledInstructions: CompatibleMessageCompiledInstruction[];
    /** Instructions that will be executed in sequence and committed in one atomic transaction if all succeed. */
    addressTableLookups: AddressTableLookup[];
}

export declare interface CompiledInstruction {
    programIdIndex: number;
    accounts: number[];
    data?: Base58 | null | undefined;
    /** present on RPC JSON and inner instructions; optional on wire */
    stackHeight?: number | null;
}

export declare const COMPUTE_BUDGET_PROGRAM_ID_STRING: ComputeBudgetProgram;

export declare type ComputeBudgetProgram = SolanaAddressInfo<"ComputeBudget111111111111111111111111111111", {
    systemProgram: true;
    name: "Compute Budget Program";
    description: "Computes budget for a transaction";
}>;

export declare interface ConfirmedSignatureInfo {
    signature: SolanaTransactionSignature;
    slot: SolanaSlot;
    err: unknown | null;
    memo: string | null;
    blockTime: TimestampS | null;
    confirmationStatus?: SolanaCommitmentLevel | null;
}

export declare const CONNECTION_CLOSE: "close";

export declare const CONNECTION_KEEP_ALIVE: "keep-alive";

export declare const CONNECTION_MESSAGE: "connection";

export declare const CONNECTION_REFUSED_FULL_MESSAGE: "connection refused";

export declare const CONNECTION_REFUSED_MESSAGE: "econnrefused";

export declare const CONNECTION_RESET_MESSAGE: "econnreset";

export declare const CONNECTION_TIMEOUT_MESSAGE: "connection timeout";

export declare const CONNECTION_UPGRADE: "upgrade";

declare interface ConnectionManager {
    getAccountInfo: (address: SolanaAddressLike, options?: ConnectionManagerOptions) => Promise<AccountDataResult>;
    getMultipleAccounts: (addresses: SolanaAddressLike[], options?: ConnectionManagerOptions) => Promise<AccountDataResult[]>;
}

export declare class ConnectionManagerFactory {
    private static connectionManager;
    private constructor();
    static setConnectionManager(connectionManager: ConnectionManager): void;
    static getConnectionManager(): ConnectionManager;
}

declare interface ConnectionManagerOptions {
    signal?: AbortSignal;
    timeout?: number;
    cacheDuration?: DurationMs;
    disableBatching?: boolean;
    disableDeduping?: boolean;
    isUnifiedBatch?: boolean;
}

export declare type Constructor<T, A = unknown> = new (...args: A[]) => T;

declare type ConstructTradingPair<Base extends string, Quote extends string> = `${Base}-${Quote}`;

export declare interface ConsumedLog extends BaseLog {
    type: "consumed";
    programId: PublicKey_2;
    unitsConsumed: number;
    meterPrev: number;
}

export declare type ContentEncoding = "gzip" | "deflate" | "br" | "identity";

export declare const CORE_PROGRAM_BYTES: {
    TOKEN_PROGRAM: Uint8Array | null;
    TOKEN_2022_PROGRAM: Uint8Array | null;
    ASSOCIATED_TOKEN_ACCOUNT_PROGRAM: Uint8Array | null;
    SYSTEM_PROGRAM: Uint8Array | null;
    COMPUTE_BUDGET_PROGRAM: Uint8Array | null;
    MEMO_PROGRAM: Uint8Array | null;
};

export declare interface CostBreakdownItem {
    type: "base_fee" | "priority_fee" | "ata_creation" | "pda_creation" | "rent";
    description: string;
    costLamports: number;
    accountAddress?: string;
}

/**
 * Thrown when transaction cost estimation fails
 */
export declare class CostEstimationError extends TransactionError {
    readonly reason?: string | undefined;
    readonly type = "COST_ESTIMATION";
    constructor(message?: string, reason?: string | undefined, originalError?: unknown);
}

export declare type CountryCode<Code extends string = string> = Tagged<Code, "CountryCode">;

export declare type CountryCode3<Code extends string = string> = Tagged<CountryCode<Code>, "CountryCode3">;

export declare const CPI_RELEASE: CPIReleaseEvent;

export declare type CPIReleaseEvent = EconomicEvent<"CPIRelease">;

export declare type Crawler<S extends string = string> = Tagged<Browser<S>, "Crawler">;

/**
 * 0 = CreateAccount { lamports: u64, space: u64, owner: Pubkey }
 */
export declare class CreateAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    from: SolanaAddressLike;
    newAccount: SolanaAddressLike;
    lamports: bigint | number | string;
    space: bigint | number | string;
    owner: PublicKeyLike;
    constructor(props: EncodableProps<CreateAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): CreateAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/**
 * 3 = CreateAccountWithSeed { base: Pubkey, seed: String, lamports: u64, space: u64, owner: Pubkey }
 */
export declare class CreateAccountWithSeedInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    from: SolanaAddressLike;
    newAccount: SolanaAddressLike;
    base: PublicKeyLike;
    seed: string;
    lamports: bigint | number | string;
    space: bigint | number | string;
    owner: PublicKeyLike;
    constructor(props: EncodableProps<CreateAccountWithSeedInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): CreateAccountWithSeedInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/**
 * Create Associated Token Account instruction
 */
export declare class CreateAssociatedTokenAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    payer: SolanaAddressLike;
    associatedToken: SolanaAddressLike;
    owner: SolanaAddressLike;
    mint: SolanaAddressLike;
    systemProgram: SolanaAddressLike & EncodableDefault;
    tokenProgram: SolanaAddressLike & EncodableDefault;
    constructor(props: EncodableProps<CreateAssociatedTokenAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): CreateAssociatedTokenAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare function createATAKey(owner: string, mint: string, isToken2022?: boolean): string;

export declare function createDeferredInstanceProxy<T>(label: string): {
    proxy: T;
    set(value: T): void;
};

/**
 * Create Idempotent Associated Token Account instruction
 */
export declare class CreateIdempotentAssociatedTokenAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    payer: SolanaAddressLike;
    associatedToken: SolanaAddressLike;
    owner: SolanaAddressLike;
    mint: SolanaAddressLike;
    systemProgram: SolanaAddressLike & EncodableDefault;
    tokenProgram: SolanaAddressLike & EncodableDefault;
    constructor(props: EncodableProps<CreateIdempotentAssociatedTokenAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): CreateIdempotentAssociatedTokenAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare function createProgramAddressKey(seeds: (Buffer | Uint8Array)[], programIdBase58: string): string;

export declare function createReplyMock(): ReplyMock;

export declare function createSocketTracker(): {
    readonly activeSockets: ReadonlySet<any>;
    readonly destroyAllSockets: () => void;
    readonly install: () => void;
    readonly uninstall: () => void;
};

export declare function createTimerTracker(): {
    readonly activeTimers: ReadonlySet<ReturnType<typeof setTimeout>>;
    readonly clearAllTrackedTimers: () => void;
    readonly install: () => void;
    readonly uninstall: () => void;
};

export declare const CRYPTO_COM: CryptoDotCom;

export declare type CryptoDotCom = ExchangeName<"Crypto.com">;

export declare type CryptoDotComId = ExchangeId<CryptoDotCom>;

/**
 * Error thrown when SHA-256 implementation is not available
 */
export declare class CryptoUnavailableError extends CryptoUtilError {
    readonly type = "CRYPTO_UNAVAILABLE";
    constructor(message?: string, originalError?: unknown);
}

export declare class CryptoUtil {
    private static cryptoCache;
    private static cryptoPromise;
    private static getCrypto;
    static sha256Sync(data: Uint8Array): Uint8Array;
    private static rotateRight32;
    static sha256(data: Uint8Array): Promise<Uint8Array>;
    /**
     * Keccak-256 (the Ethereum / Solana variant - original Keccak `pad10*1` with
     * the 0x01 domain byte, NOT NIST SHA3-256 which uses 0x06). Synchronous and
     * dependency-free: there is no host primitive for the legacy keccak padding
     * (Node's "sha3-256" produces a different digest), and merkle-tree builders
     * hash nodes in tight loops where a synchronous call is required. Returns a
     * fresh 32-byte digest; the input is never mutated.
     */
    static keccak256(data: Uint8Array): Uint8Array;
    /**
     * Keccak `pad10*1`: write the 0x01 domain byte after the message, then set the
     * high bit (0x80) of the final byte of the last rate block. When only a single
     * pad byte remains, both land in the same byte and merge to 0x81. (SHA-3 would
     * use a 0x06 domain byte instead of 0x01.)
     */
    private static keccakPad;
    /** 64-bit left-rotate within a keccak lane. */
    private static keccakRotl;
    /** The Keccak-f[1600] permutation, applied in place to the 25-lane state. */
    private static keccakF;
    static signSecp256k1MessageHash(messageHash: Uint8Array, privateKey: Uint8Array): Secp256k1RecoverableSignature;
    static deriveSecp256k1PublicKey(privateKey: Uint8Array): Uint8Array;
    static recoverSecp256k1PublicKey(messageHash: Uint8Array, signature: Uint8Array, recoveryId: number): Uint8Array;
    private static assertSecp256k1MessageHash;
    private static secp256k1PrivateKeyToScalar;
    private static secp256k1DeterministicNonce;
    private static secp256k1NextNonce;
    private static hmacSha256Sync;
    private static concatBytes;
    private static bytesToBigIntBE;
    private static bigIntToBytesBE;
    private static secp256k1AffinePointToBytes;
    private static secp256k1PointFromX;
    private static secp256k1IsZero;
    private static secp256k1Add;
    private static secp256k1Double;
    private static secp256k1Multiply;
    private static secp256k1ToAffine;
    private static secp256k1Mod;
    private static secp256k1ModPow;
    private static secp256k1ModInverse;
    static hmacSha256(data: Uint8Array, key: Uint8Array): Promise<Uint8Array>;
    static hmacSha512(data: Uint8Array, key: Uint8Array): Promise<Uint8Array>;
    /**
     * Validates if a 32-byte array represents a valid Ed25519 curve point
     */
    static isValidEd25519Point(bytes: Uint8Array, zip215?: boolean): boolean;
    private static uvRatio;
    /**
     * Computes modular inverse using extended Euclidean algorithm
     */
    private static modInverse;
    /**
     * Square root modulo p for p ≡ 5 (mod 8) using the formula from RFC 8032
     */
    private static modSqrt;
    /**
     * Checks if a number is a quadratic residue mod p using Legendre symbol
     */
    private static isQuadraticResidue;
    /**
     * Modular exponentiation
     */
    private static modPow;
    /**
     * Verifies an Ed25519 signature
     */
    static verifyEd25519(signature: Uint8Array, message: Uint8Array, publicKey: Uint8Array): Promise<boolean>;
    /**
     * Converts a raw 32-byte Ed25519 public key to DER format for Node.js
     */
    private static ed25519RawToDer;
    static signEd25519(message: Uint8Array, privateKey: ValidEd25519PrivateKey): Promise<Uint8Array>;
    /** Build minimal RFC 8410 PKCS#8 for Ed25519 from a 32-byte seed */
    private static ed25519SeedToPkcs8Der;
    static pbkdf2(password: string, salt: Uint8Array, iterations: number, keylen: number): Promise<Uint8Array>;
    static randomBytes(size: number): Promise<Uint8Array>;
    static timingSafeEqual(a: Uint8Array, b: Uint8Array): boolean;
    static hashPassword(password: string): Promise<HashedPassword>;
    static verifyPassword(password: string, stored: string): Promise<boolean>;
    static encryptAES256GCM(plaintext: Uint8Array, privateKey: Uint8Array, aad?: Uint8Array): Promise<Uint8Array>;
    static decryptAES256GCM(ciphertextCombined: Uint8Array, privateKey: Uint8Array, aad?: Uint8Array): Promise<Uint8Array>;
    static signRS256(message: Uint8Array, privateKey: ValidRsaPrivateKey): Promise<Uint8Array>;
    static verifyRS256(signature: Uint8Array, message: Uint8Array, publicKey: ValidRsaPublicKey): Promise<boolean>;
    static signES256(message: Uint8Array, privateKey: ValidEcPrivateKey): Promise<Uint8Array>;
    static verifyES256(signature: Uint8Array, message: Uint8Array, publicKey: ValidEcPublicKey): Promise<boolean>;
    /** DER ECDSA signature → IEEE P1363 (r‖s, each `coordLen` bytes) */
    private static ecDerToP1363;
    /** IEEE P1363 (r‖s, 64 bytes) → DER ECDSA signature */
    private static ecP1363ToDer;
    static deriveAesKeyFromX25519(senderPublicSpkiDer: Uint8Array, recipientPrivatePkcs8Der: Uint8Array, salt: Uint8Array, // random per message
    info: Uint8Array): Promise<Uint8Array>;
}

/**
 * Base class for CryptoUtil-related errors
 */
export declare abstract class CryptoUtilError extends SolomonLabsError {
    readonly category = "CRYPTO";
}

export declare const CSI300: CSI300Name;

export declare type CSI300Name = EquityIndexName<"CSI 300">;

export declare type CSI300Symbol = EquityIndexSymbol<"CSI300">;

/**
 * Error thrown when curve validation fails
 */
export declare class CurveValidationError extends PublicKeyError {
    readonly type = "CURVE_VALIDATION";
    constructor(message?: string, originalError?: unknown);
}

export declare type CustomMetaColumnType = Tagged<string, "CustomMetaColumnType">;

export declare const DAILY_MAINTENANCE_SESSION: DailyMaintenanceSession;

export declare type DailyMaintenanceSession = MarketSession<"Daily Maintenance">;

/**
 * Error thrown when data being decoded is null, undefined, zero length or empty length
 * when something was expected.
 */
export declare class DataEmptyDecoderError extends DecoderError {
    readonly type = "EMPTY_DATA";
    constructor(message?: string, originalError?: unknown);
}

/**
 * Error thrown when data being encoded is null, undefined, zero length, empty length
 * or otherwise missing.
 */
export declare class DataEmptyEncoderError extends EncoderError {
    readonly type = "EMPTY_DATA";
    constructor(message?: string, originalError?: unknown);
}

export declare interface DataLog extends BaseLog {
    type: "data";
    data: Base64;
}

export declare type DateDDMMYY<S extends string = string> = Tagged<DateString & DateDDMMYYPattern<S>, "DateDDMMYY">;

export declare type DateDDMMYYPattern<T extends string> = T extends `${infer D}-${infer M}-${infer Y}` ? Len<D> extends 2 ? IsAllDigits<D> extends true ? Len<M> extends 2 ? IsAllDigits<M> extends true ? Len<Y> extends 2 ? IsAllDigits<Y> extends true ? T : never : never : never : never : never : never : never;

export declare type DateLike = string | number | Date;

/**
 * Represents different kinds of date-like values that can be converted to Date instances
 */
export declare const DateLikeKind: {
    /** A JavaScript {@link Date} instance */
    readonly Date: "Date";
    /** Unix timestamp in milliseconds as {@link TimestampMs} (e.g., {Date.now}) */
    readonly TimestampMs: "TimestampMs";
    /** Unix timestamp in seconds as {@link TimestampS} (e.g., Math.floor({Date.now} / 1000)) */
    readonly TimestampS: "TimestampS";
    /** ISO 8601 string or timezone string (e.g., "2024-01-01T00:00:00Z") */
    readonly ISOOrTZString: "ISOOrTZString";
    /** Human-readable date string (e.g., "2024-01-01" or "01-01-2024") */
    readonly StructuredString: "StructuredString";
};

export declare type DateLikeKind = (typeof DateLikeKind)[keyof typeof DateLikeKind];

export declare type DateLikeString<S extends string> = IsoDate<S> | IsoTime<S> | IsoDateTime<S> | DateDDMMYY<S> | DateMMDDYYYY<S> | DateYYYYMMDD<S>;

export declare type DateLikeStringPattern<S extends string> = IsoDatePattern<S> | IsoTimePattern<S> | IsoDateTimePattern<S> | DateDDMMYYPattern<S> | DateMMDDYYYYPattern<S> | DateYYYYMMDDPattern<S>;

export declare type DateMMDDYYYY<S extends string = string> = Tagged<DateString & DateMMDDYYYYPattern<S>, "DateMMDDYYYY">;

export declare type DateMMDDYYYYPattern<T extends string> = T extends `${infer M}/${infer D}/${infer Y}` ? Len<M> extends 2 ? IsAllDigits<M> extends true ? Len<D> extends 2 ? IsAllDigits<D> extends true ? Len<Y> extends 4 ? IsAllDigits<Y> extends true ? T : never : never : never : never : never : never : never;

export declare type DateOrTime = Tagged<string, "DateOrTime">;

export declare type DatePeriod = Exclude<Frequency, "Minutely" | "TwoMinutely" | "FiveMinutely" | "TenMinutely" | "FifteenMinutely" | "ThirtyMinutely" | "FortyMinutely" | "FiftyMinutely" | "Hourly" | "TwoHourly" | "ThreeHourly" | "FourHourly" | "SixHourly" | "EightHourly" | "TwelveHourly">;

export declare type DatePredicate = (date: Date) => boolean;

export declare type DateString = Tagged<DateOrTime, "DateString">;

export declare type DateTime = Tagged<DateString & Time, "DateTime">;

export declare type DateYYYYMMDD<S extends string = string> = Tagged<DateString & DateYYYYMMDDPattern<S>, "DateYYYYMMDD">;

export declare type DateYYYYMMDDPattern<T extends string> = T extends `${infer Y}/${infer M}/${infer D}` ? Len<Y> extends 4 ? IsAllDigits<Y> extends true ? Len<M> extends 2 ? IsAllDigits<M> extends true ? Len<D> extends 2 ? IsAllDigits<D> extends true ? T : never : never : never : never : never : never : never;

export declare const DAX: DAXName;

export declare type DAXName = EquityIndexName<"DAX">;

export declare type DAXSymbol = EquityIndexSymbol<"DAX">;

export declare const DEAULT_RPC_BACKOFF_MULTIPLIER = 2;

export declare const DEAULT_RPC_BASE_DELAY_MS: DurationMs;

export declare const DEAULT_RPC_BURST_LIMIT = 20;

export declare const DEAULT_RPC_MAX_DELAY_MS: DurationMs;

export declare const DEAULT_RPC_MAX_RETRIES = 3;

export declare const DEAULT_RPC_OPTIMISTIC_VALIDATION = true;

export declare const DEAULT_RPC_REQUESTS_PER_SECOND = 10;

export declare type deBridge = SolanaProtocolName<"deBridge">;

export declare type deBridgeId = SolanaProtocolId<deBridge>;

export declare type Dec<N extends number> = BuildTuple<unknown, N> extends readonly [unknown, ...infer R] ? R["length"] : 0;

declare type DecodableFields<T> = Pick<T, Exclude<DecodableNonFnKeys<T>, DecodableReadonlyKeys<T>>>;

export declare type DecodableInstruction = EncodedInstruction | LegacyTransactionInstruction | IsEncodable;

declare type DecodableIsReadonly<T, K extends keyof T> = {
    readonly [P in K]: T[K];
} extends {
    [P in K]: T[K];
} ? false : true;

/** Make defaulted keys optional, required keys required, and allow optional keys and programId as optional. Excludes functions and readonly properties. */
declare type DecodableNonFnKeys<T> = {
    [K in keyof T]: T[K] extends (...args: any) => any ? never : K;
}[keyof T];

/** Drops functions and readonly members; `address` becomes optional when present. */
export declare type DecodableProps<T> = MakeOptional<DecodableFields<T>, Extract<keyof DecodableFields<T>, "address">>;

declare type DecodableReadonlyKeys<T> = {
    [K in keyof T]: DecodableIsReadonly<T, K> extends true ? K : never;
}[keyof T];

export declare type DecodeDataPolicy = "error" | "ignore";

export declare class Decoder {
    private static getCacheKey;
    /**
     * Decode length as compact-u16 (Solana varuint)
     */
    static decodeLength(bytes: Uint8Array): [number, number];
    static decode<DecodedClass extends IsDecodableStatic>(dataOrInstruction: string | Uint8Array | Buffer | null | undefined, ctor: DecodedClass, address?: SolanaAddressLike): InstanceType<DecodedClass>;
    static decode<InstructionClass extends IsEncodableStatic>(dataOrInstruction: DecodableInstruction, ctor: InstructionClass, address?: SolanaAddressLike): InstanceType<InstructionClass>;
    static decode<LogClass extends IsLogDecodableStatic>(dataOrInstruction: string | Base64 | Uint8Array | Buffer | null, ctor: LogClass, address?: SolanaAddressLike): InstanceType<LogClass>;
    static getAccount<DecodedClass extends IsDecodableStatic>(address: SolanaAddressLike, decoder: DecodedClass): Promise<InstanceType<DecodedClass>>;
    /**
     * Decode multiple objects from bytes using a decoder with length
     */
    private static decodeObjectsFromSchema;
    /** Read a coder's fields sequentially from `offset`, returning an instance. */
    private static decodeStructFields;
    /**
     * Decode a Rust data enum: one tag byte selects the variant coder, whose
     * fields follow immediately. Length is variable, so the variant is read in
     * place rather than from a fixed-size slice.
     */
    private static decodeBorshEnum;
    private static decodeField;
}

/**
 * Base class for Decoder-related errors
 */
export declare abstract class DecoderError extends SolomonLabsError {
    readonly category = "DECODER";
}

export declare type DeepMutable<T> = T extends (...args: infer A) => infer R ? (...args: A) => R : T extends ReadonlyArray<infer U> ? DeepMutable<U>[] : T extends object ? {
    -readonly [K in keyof T as undefined extends T[K] ? never : K]: IsPrimitiveLike<NonNil<T[K]>> extends true ? T[K] : IsKept<NonNil<T[K]>, DefaultKeep | ExtraKeep> extends true ? T[K] : IsBrandedType<NonNil<T[K]>> extends true ? T[K] : DeepMutable<T[K]>;
} & {
    -readonly [K in keyof T as undefined extends T[K] ? K : never]?: IsPrimitiveLike<NonNil<T[K]>> extends true ? T[K] : IsKept<NonNil<T[K]>, DefaultKeep | ExtraKeep> extends true ? T[K] : IsBrandedType<NonNil<T[K]>> extends true ? T[K] : DeepMutable<T[K]>;
} : T;

export declare type DeepPartial<T> = T | (T extends Array<infer U> ? DeepPartial<U>[] : T extends Map<infer K, infer V> ? Map<DeepPartial<K>, DeepPartial<V>> : T extends Set<infer M> ? Set<DeepPartial<M>> : T extends object ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : T);

export declare type DeepSerializableType<T> = T extends null | undefined ? T : T extends readonly (infer U)[] ? readonly DeepSerializedType<U>[] : T extends (infer U)[] ? DeepSerializedType<U>[] : T extends Function ? T : IsTagged<T> extends true ? T : IsPlainObject<T> extends true ? {
    [K in keyof T]: DeepSerializedType<T[K]>;
} : T extends object ? HasToString : T;

export declare type DeepSerializedType<T> = T extends null | undefined ? T : T extends readonly (infer U)[] ? readonly DeepSerializedType<U>[] : T extends (infer U)[] ? DeepSerializedType<U>[] : T extends Function ? T : IsTagged<T> extends true ? T : IsPlainObject<T> extends true ? {
    [K in keyof T]: DeepSerializedType<T[K]>;
} : T extends object ? string : T;

export declare type DeepWritable<T> = T extends (...args: infer A) => infer R ? (...args: A) => R : T extends ReadonlyArray<infer U> ? DeepWritable<U>[] : T extends object ? {
    -readonly [K in keyof T]: DeepWritable<T[K]>;
} : T;

export declare const DEFAULT_HTTP_CLIENT_HEADERS: {
    readonly "user-agent": `${Scope<"SolomonLabs">}/${Version<"1.0.0">} (+${Url<"https://solomonlabs.org/">}${Slug<"rest-client">}; ${Identity<"dev">}@${Domain<"solomonlabs.org">})`;
    readonly accept: "application/json";
    readonly "accept-language": "en-US,en;q=0.8";
    readonly connection: "keep-alive";
    readonly dnt: "1";
    readonly "cache-control": "no-cache";
    readonly pragma: "no-cache";
    readonly "sec-fetch-dest": "empty";
    readonly "sec-fetch-mode": "cors";
    readonly "sec-fetch-site": "cross-site";
};

export declare const DEFAULT_MEMCACHED_PORT: Port<11211>;

export declare const DEFAULT_REDIS_PORT: Port<6379>;

export declare const DEFAULT_RESULT_LIMIT: 50;

export declare const DEFAULT_SORT_ORDER: "ASC";

export declare const DEFAULT_USER_AGENT: `${Scope<"SolomonLabs">}/${Version<"1.0.0">} (+${Url<"https://solomonlabs.org/">}${Slug<"rest-client">}; ${Identity<"dev">}@${Domain<"solomonlabs.org">})`;

declare type DefaultedKeys<T> = {
    [K in FilteredKeys<T>]: T[K] extends EncodableDefault ? K : never;
}[FilteredKeys<T>];

declare type DefaultKeep = Date | RegExp | Error | URL | URLSearchParams | Promise<unknown> | Map<unknown, unknown> | Set<unknown> | WeakMap<object, unknown> | WeakSet<object> | ArrayBuffer | SharedArrayBuffer | DataView | Int8Array | Uint8Array | Uint8ClampedArray | Int16Array | Uint16Array | Int32Array | Uint32Array | Float32Array | Float64Array | BigInt64Array | BigUint64Array;

export declare type Dehydrated<T> = Tagged<T, "Dehydrated">;

export declare type DeliveryTradingPair<TBase extends string = string, TQuote extends string = string> = Tagged<FuturesTradingPair<TBase, TQuote>, "DeliveryTradingPair">;

/**
 * Thrown when attempting to use a deprecated method
 */
export declare class DeprecatedTransactionMethodError extends TransactionError {
    readonly method?: string | undefined;
    readonly type = "DEPRECATED";
    constructor(message?: string, method?: string | undefined, originalError?: unknown);
}

/** Depth span, i.e. stack depth beginning at zero and goes up */
export declare type DepthSpan = ["depth"];

export declare const DERIBIT: Deribit;

export declare type Deribit = ExchangeName<"Deribit">;

export declare type DeribitId = ExchangeId<Deribit>;

/**
 * Account discriminator: sha256("account:<AccountName>")[0..8]
 */
export declare function deriveAnchorAccountDiscriminator(name: string): Promise<Uint8Array>;

/**
 * Generic Anchor-style discriminator: first 8 bytes of sha256("<namespace>:<name>").
 * This matches Anchor's account / instruction / event discriminators when the correct
 * namespace is used.
 */
export declare function deriveAnchorDiscriminator(namespace: AnchorDiscriminatorNamespace, name: string): Promise<Uint8Array>;

/**
 * Event discriminator: sha256("event:<EventName>")[0..8]
 */
export declare function deriveAnchorEventDiscriminator(name: string): Promise<Uint8Array>;

/**
 * Global instruction discriminator: sha256("global:<method_name>")[0..8]
 * This is what Anchor uses for normal instructions from the IDL "instructions" array.
 */
export declare function deriveAnchorInstructionDiscriminator(name: string): Promise<Uint8Array>;

/**
 * Optional helper for state methods (if you ever need it):
 * sha256("state:<MethodName>")[0..8]
 */
export declare function deriveAnchorStateDiscriminator(name: string): Promise<Uint8Array>;

export { describe }

export declare type Description = Tagged<Text, "Description">;

/**
 * Convert a discriminator to a lower-case hex string for logging / debugging.
 */
export declare function discriminatorToHex(discriminator: Uint8Array): string;

/**
 * Display: an ordered list of domain-constrained opcodes.
 */
export declare type Display<Domain extends DisplayDomain> = DisplayOpcode<Domain>[];

/**
 * Display base: an ordered list of unconstrained opcodes.
 */
export declare type DisplayBase = DisplayOpcodeBase[];

/**
 * Display Domain
 *
 * @typeParam Groups - Union of group kinds
 * @typeParam IconSpans - Array of [type] or [type, variant] for icon spans
 * @typeParam GlobalSpans - Array of [tag] or [tag, role] for unconstrained spans
 * @typeParam GroupedSpans - Array of [group, ...spans] where spans are [tag] or [tag, role]
 */
export declare interface DisplayDomain<Groups extends string = string, Icons extends IconDef[] = IconDef[], GlobalSpans extends SpanDef[] = SpanDef[], GroupedSpans extends GroupSpansDef[] = GroupSpansDef[]> {
    readonly __groups?: Groups;
    readonly __icons?: Icons;
    readonly __globalSpans?: GlobalSpans;
    readonly __groupedSpans?: GroupedSpans;
}

export declare type DisplayOp = SpanOp | IconOp | SpaceOp | MarkerOp | BreakOp | GroupStartOp | GroupStartEnterOp | GroupEnterOp | GroupExitOp | GroupExitEndOp | GroupEndOp;

/**
 * A single display opcode constrained by a domain.
 */
export declare type DisplayOpcode<Domain extends DisplayDomain> = OpSpan<Domain> | OpIcon<Domain> | OpGroupStart<Domain> | OpGroupStartEnter<Domain> | OpGroupEnter | OpGroupExit | OpGroupExitEnd<Domain> | OpGroupEnd<Domain> | OpBreak | OpMarker | OpSpace;

/**
 * A single display opcode without any domain constraints.
 */
export declare type DisplayOpcodeBase = OpSpanBase | OpIconBase | OpGroupStartBase | OpGroupStartEnterBase | OpGroupEnter | OpGroupExit | OpGroupExitEndBase | OpGroupEndBase | OpBreak | OpMarker | OpSpace;

export declare const DNT_ENABLED: "1";

export declare type Domain<S extends string = string> = Tagged<S, "Domain">;

/** Extract group kinds union */
export declare type DomainGroups<Domain extends DisplayDomain> = Domain extends DisplayDomain<infer Group, any, any, any> ? Group : string;

/** Extract icon types union */
export declare type DomainIconTypes<Domain extends DisplayDomain> = Domain extends DisplayDomain<any, infer Icons, any, any> ? IconType<Icons[number]> : string;

/** Icon variant allowed for a type */
export declare type DomainIconVariant<Domain extends DisplayDomain, Type extends string> = Domain extends DisplayDomain<any, infer Icons, any, any> ? IconVariant<Extract<Icons[number], [Type, ...any[]]>, Type> : string | undefined;

/** Role allowed for a tag */
export declare type DomainSpanRole<Domain extends DisplayDomain, Tag extends string> = Domain extends DisplayDomain<any, any, infer Global, infer Grouped> ? SpanRole<Extract<Global[number], [Tag, ...any[]]>, Tag> | SpanRole<Extract<GroupedSpansSpans<Grouped[number]>[number], [Tag, ...any[]]>, Tag> : string | undefined;

/** All span tags in domain (global + all grouped) */
export declare type DomainSpanTags<Domain extends DisplayDomain> = Domain extends DisplayDomain<any, any, infer Global, infer Grouped> ? SpanTag<Global[number]> | SpanTag<GroupedSpansSpans<Grouped[number]>[number]> : string;

/** Span tags allowed in a specific group (global + grouped to that group) */
export declare type DomainSpanTagsInGroup<Domain extends DisplayDomain, Kind extends string> = Domain extends DisplayDomain<any, any, infer Global, infer Grouped> ? SpanTag<Global[number]> | {
    [Key in keyof Grouped]: Kind extends GroupedSpansGroup<Grouped[Key]> ? SpanTag<GroupedSpansSpans<Grouped[Key]>[number]> : never;
}[number] : string;

declare interface DOMStringList {
    readonly length: number;
    contains(string: string): boolean;
    item(index: number): string | null;
}

export declare const DOW_JONES_INDUSTRIAL_AVERAGE: DowJonesIndustrialAverageName;

export declare type DowJonesIndustrialAverage = EquityIndexId<"dow-jones-industrial-average">;

export declare type DowJonesIndustrialAverageName = EquityIndexName<"Dow Jones Industrial Average">;

export declare type DowJonesIndustrialAverageSymbol = EquityIndexSymbol<"DJIA">;

export declare const DRIFT: Drift;

export declare type Drift = SolanaProtocolName<"Drift">;

export declare const DRIFT_URL: Url<"https://www.drift.trade/">;

export declare type DriftId = SolanaProtocolId<Drift>;

export declare type Duration<T extends number = number> = Tagged<T, "Duration">;

export declare type DurationMs<T extends number = number> = Tagged<Duration<T> & WholeNumberPattern<T>, "DurationMs">;

export declare type DurationNs<T extends number = number> = Tagged<Duration<T> & WholeNumberPattern<T>, "DurationNs">;

export declare type DurationS<T extends number = number> = Tagged<Duration<T> & WholeNumberPattern<T>, "DurationS">;

export declare type DurationUs<T extends number = number> = Tagged<Duration<T> & WholeNumberPattern<T>, "DurationUs">;

export declare const EACCES_CODE: "EACCES";

export declare const EADDRINUSE_CODE: "EADDRINUSE";

export declare const EASTER_MONDAY: EasterMonday;

export declare const EASTER_SATURDAY: EasterSaturday;

export declare const EASTER_SUNDAY: EasterSunday;

export declare type EasterMonday = Holiday<"Easter Monday">;

export declare type EasterSaturday = Holiday<"Easter Saturday">;

export declare type EasterSunday = ObservedDate<"Easter Sunday">;

export declare const ECONNREFUSED_CODE: "ECONNREFUSED";

export declare const ECONNRESET_CODE: "ECONNRESET";

export declare type EconomicEvent<EconomicEventName extends string = string> = ObservedEvent<EconomicEventName>;

export declare interface EconomicEventEntry extends BaseCalendarEntry {
    type: "Economic";
    time?: string;
    impact: CalendarEventImpact;
    country: CountryCode;
    description?: string;
}

export declare type EcPrivateKey = Tagged<PrivateKey, "EcPrivateKey">;

export declare const ED25519_PROGRAM_ID = "Ed25519SigVerify111111111111111111111111111";

export declare type Ed25519PrivateKey = Tagged<PrivateKey, "Ed25519PrivateKey">;

export declare const EDGE: Browser<"Edge">;

export declare const EEXIST_CODE: "EEXIST";

export declare const EIGHT_HOURS_MS: DurationMs<28800000>;

export declare const EIGHT_HOURS_S: DurationS<28800>;

export declare type Email = Tagged<Identity<string>, "Email">;

/** Mixin you intersect with a property type to say “this field is defaulted”. */
export declare type EncodableDefault = {
    readonly [encodableDefaultBrand]?: true;
};

declare const encodableDefaultBrand: unique symbol;

/** Make defaulted keys optional, required keys required, and allow optional keys and programId as optional. Excludes functions and readonly properties. */
export declare type EncodableProps<T> = Partial<Pick<T, DefaultedKeys<T>>> & Required<Pick<T, RequiredKeys<T>>> & Partial<Pick<T, OptionalProvidedKeys<T>>>;

export declare interface EncodeAccountSchema {
    signer?: boolean;
    writable?: boolean;
    pda?: ProgramDerivedAccountDefinition;
    relations?: readonly string[];
    xRelations?: readonly AccountRelationMapping[];
    /** Account data size in bytes (for rent calculation) */
    accountSize?: number;
    /** Whether this account will be created by the instruction */
    willCreate?: boolean;
}

export declare type EncodeAccountsSchema = Record<string, EncodeAccountSchema>;

export declare type EncodeAccountsSchemaOrNull = EncodeAccountsSchema | null;

export declare type Encoded = Tagged<string, "Encoded">;

export declare interface EncodedInstruction {
    programId: PublicKeyLike;
    accounts: InstructionAccount[];
    /** Binary‐encoded instruction data */
    data: Uint8Array;
}

export declare class Encoder {
    static isEncodableInstruction(instruction: unknown): instruction is IsEncodable;
    /**
     * Encode length as compact-u16 (Solana varuint)
     */
    static encodeLength(n: number): Uint8Array;
    static isLegacyTransactionInstruction(instruction: unknown): instruction is LegacyTransactionInstruction;
    static instruction<T extends IsEncodable, I extends {
        discriminator: Uint8Array;
        getSchema(): Schema;
        getAccountsSchema(): EncodeAccountsSchemaOrNull;
    }>(programId: SolanaAddressLike, instance: T, instructionType: I): EncodedInstruction;
    static accounts<T extends IsEncodable, I extends {
        discriminator: Uint8Array;
        getAccountsSchema(): EncodeAccountsSchemaOrNull;
    }>(instance: T, instructionType: I): InstructionAccount[];
    /**
     * Get account creation info for an instruction
     */
    static getAccountCreationInfo<_T extends IsEncodable, I extends {
        discriminator: Uint8Array;
        getAccountsSchema(): EncodeAccountsSchemaOrNull;
    }>(instructionType: I): Array<{
        accountName: string;
        accountSize: number;
        willCreate: boolean;
    }>;
    static encode<T extends IsCodable, I extends {
        discriminator?: Uint8Array;
        getSchema(): Schema;
    }>(instance: T, instructionType: I): Uint8Array;
    /**
     * Encode multiple objects from schema – flattens parts and concatenates once.
     */
    private static encodeObjectsFromSchema;
    /**
     * Encode a Rust data enum: the leading tag byte followed by that variant's
     * fields. The variant is identified by matching the value against its coder,
     * the same way TaggedUnion resolves its variants.
     */
    private static encodeBorshEnum;
    /** Primitive encoder – mirrors Decoder's switch-style logic */
    private static encodeField;
}

/**
 * Base class for Encoder-related errors
 */
export declare abstract class EncoderError extends SolomonLabsError {
    readonly category = "ENCODER";
}

export declare const ENCODING_BROTLI: "br";

export declare const ENCODING_DEFLATE: "deflate";

export declare const ENCODING_GZIP: "gzip";

export declare const ENCODING_IDENTITY: "identity";

export declare type EncryptionKey = Tagged<string | Uint8Array, "EncryptionKey">;

export declare function endsWith(name: string, suff: string): boolean;

export declare type EnhancedSortOrder = "asc" | "desc";

/** How much transaction data Helius/Alchemy returns per notification. */
export declare type EnhancedTransactionDetails = "full" | "signatures" | "accounts" | "none";

export declare const ENOENT_CODE: "ENOENT";

export declare const ENOTFOUND_CODE: "ENOTFOUND";

export declare const ENTITY_SINGLETON_ID: "singleton";

export declare const EntityRepositoryBrand: unique symbol;

export declare type EnumerableObject = Record<PropertyKey, any>;

declare type Enumerate<N extends number, Acc extends number[] = []> = Acc["length"] extends N ? Acc[number] : Enumerate<N, [...Acc, Acc["length"]]>;

export declare const EPIPE_CODE: "EPIPE";

export declare type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2 ? true : false;

export declare type EqualIgnoreNullable<A, B> = Equal<A | undefined | null, B | undefined | null>;

export declare type EqualIgnoreReadonly<A, B> = Equal<DeepWritable<A>, DeepWritable<B>>;

export declare type EqualIgnoreReadonlyShallow<A, B> = Equal<ShallowWritable<A>, ShallowWritable<B>>;

/** i.e. "=" */
export declare type EqualsSpan = ["equals"];

export declare type EquityIndexId<S extends string = string> = Tagged<S, "EquityIndexId">;

export declare type EquityIndexName<S extends string = string> = Tagged<S, "EquityIndexName">;

export declare type EquityIndexSymbol<S extends string = string> = Tagged<S, "EquityIndexSymbol">;

export declare interface EstimatorConfig {
    baseFee?: number;
    priorityFee?: number;
    checkExistingAccounts?: boolean;
    lamportsPerByteYear?: number;
    exemptionThreshold?: number;
}

export declare const ETHENA_USDE: EthenaUsde;

export declare const ETHENA_USDE_NAME: EthenaUsdeName;

export declare const ETHENA_USDE_SYMBOL: EthenaUsdeSymbol;

export declare type EthenaUsde = TokenId<"ethena-usde">;

export declare type EthenaUsdeName = TokenName<"Ethena USDv">;

export declare type EthenaUsdeSolana = SolanaAddressInfo<"DEkqHyPN7GMRJ5cArtQFAWefqbZb33Hyf6s5iCwjEonT", {
    tokenId: SolanaToken<EthenaUsde>;
    decimals: 9;
    program: "Token";
}>;

export declare type EthenaUsdeSymbol = TokenSymbol<"USDe">;

export declare const ETHEREUM: Ethereum;

export declare type Ethereum = AssetId<"ethereum">;

export declare const ETHEREUM_NAME: EthereumName;

export declare const ETHEREUM_SYMBOL: EthereumSymbol;

export declare type EthereumName = AssetName<"Ethereum">;

export declare type EthereumSymbol = AssetSymbol<"ETH">;

export declare const ETIMEDOUT_CODE: "ETIMEDOUT";

/**
 * Public key length in bytes (32 bytes)
 */
export declare const EVENT_PUBLIC_KEY_BYTE_LENGTH = 32;

export declare type EventFrequency = Exclude<Frequency, "Minutely" | "TwoMinutely" | "FiveMinutely" | "TenMinutely" | "FifteenMinutely" | "ThirtyMinutely" | "FortyMinutely" | "FiftyMinutely" | "Hourly" | "TwoHourly" | "ThreeHourly" | "FourHourly" | "SixHourly" | "EightHourly" | "TwelveHourly">;

export declare type EventImpact = Impact;

export declare class EventLog {
    /**
     * Process transaction into ordered events.
     *
     * Merging strategy:
     * - Instructions form the canonical ordering (execution order)
     * - Logs interleave: instruction first, then invoke log, then nested content
     * - Index counts UP per stack depth, not globally
     *
     * NOTE:
     * I checked the rust source code for Solana and verified the order in which
     * instructions occur and then logs are emitted, so the current ordering is
     * canonically what does happen onchain.
     */
    static transactionEvents(params: {
        transaction: Transaction;
        parsedInstructions?: ParsedInstruction[];
        parsedLogs?: ParsedLog[];
        shouldProcessTokenInstructions?: boolean;
        shouldParseInstructions?: boolean;
        shouldParseLogs?: boolean;
    }): TransactionEvent[];
    private static incrementTokenAccountTouchCount;
    private static getTokenAccountTouchCount;
    private static publicKeysEqual;
    /**
     * Process token instruction events with balance details.
     * Mutates in-place.
     */
    static processTokenInstructionsEvents(transaction: Transaction, tokenInstructionEvents: TokenInstructionNoDetailsEvent[], tokenInstructionEventsCount?: number): void;
    static isDecodedLogEvent(logEvent: LogEvent): logEvent is LogEvent & {
        decoded: IsLogDecodable;
    };
    static isLogEventInstance<LogCtor extends IsLogDecodableStatic>(logEvent: LogEvent, logClass: LogCtor): logEvent is LogEvent & {
        decoded: InstanceType<LogCtor>;
    };
    static isDecodedInstructionEvent(instructionEvent: InstructionEvent): instructionEvent is InstructionEvent & {
        decoded: IsEncodable;
    };
    static isAnchorEventCpiInstructionEvent(instructionEvent: InstructionEvent): instructionEvent is AnchorEventCpiInstructionEvent;
    static isTokenInstructionEvent(instructionEvent: InstructionEvent): instructionEvent is TokenInstructionEvent | TokenInstructionNoDetailsEvent;
    static isTokenInstructionDetailedEvent(instructionEvent: TokenInstructionEvent | TokenInstructionNoDetailsEvent): instructionEvent is TokenInstructionEvent;
    static isTokenInstructionDetailedEventInstance<InstructionCtor extends IsEncodableStatic>(instructionEvent: TokenInstructionEvent | TokenInstructionNoDetailsEvent, instructionClass: InstructionCtor): instructionEvent is TokenInstructionEvent & {
        decoded: InstanceType<InstructionCtor>;
    };
    static isAnchorSelfCpiParsedLog(log: ParsedLog): log is AnchorCpiLog;
}

export declare type EventType = "instruction" | "log";

export declare type ExactKeys<T> = readonly (keyof T)[] & {
    length: UnionToTuple<keyof T>["length"];
};

export declare type ExchangeId<T extends TaggedFlat> = Tagged<MarketId<TagTypeStringToId<T>>, "ExchangeId">;

export declare type ExchangeName<T extends string = string> = Tagged<MarketName<T>, "ExchangeName">;

declare type ExcludedKeys<T> = ReadonlyKeys<T>;

export declare type Expect<T extends true> = T;

export declare type Explanation = Tagged<Text, "Explanation">;

export declare const EXPONENT: Exponent;

export declare type Exponent = SolanaProtocolName<"Exponent">;

export declare const EXPONENT_URL: Url<"https://exponential.fi/">;

export declare type ExponentId = SolanaProtocolId<Exponent>;

export declare type ExtractBaseType<T> = T extends {
    readonly [tagType]: infer U;
} ? U : T;

export declare type ExtractBrandedSerializedType<T> = ExtractBrandedType<T, typeof SerialzedBrand>;

export declare type ExtractBrandedType<T, B extends symbol> = T extends BrandedInterface<B, infer U> ? U : never;

export declare type ExtractDehydratedType<T> = T extends Dehydrated<infer U> ? U : never;

declare type ExtractEncoding<BytesArrayType> = BytesArrayType extends Bytes<infer Encoding> ? Encoding : never;

/**
 * Extracts nested string property with optional fallback
 */
export declare function extractNestedString<T extends Record<string, unknown>>(obj: T, key: keyof T, nestedKey: string): string | undefined;

export declare function extractNestedString<T extends Record<string, unknown>, F extends string>(obj: T, key: keyof T, nestedKey: string, fallback: F): string | F;

export declare function extractNestedString(obj: unknown, key: string, nestedKey: string): string | undefined;

export declare function extractNestedString<F extends string>(obj: unknown, key: string, nestedKey: string, fallback: F): string | F;

export declare type ExtractSerializedType<T> = T extends BrandedSerialized<infer U> ? U : never;

export declare type ExtractSerializedValueType<T> = T extends Serialized<any, infer S> ? S : never;

export declare type ExtractStringFromTagged<T> = T extends Tagged<infer U, any, any> ? (U extends Tagged<any, any, any> ? ExtractStringFromTagged<U> : U) : T;

export declare type ExtractTuple<T> = T extends readonly unknown[] ? T : T extends unknown[] ? readonly [...T] : never;

export declare function extractValue(data: unknown, path: string | string[]): unknown;

declare type ExtraKeep = DeepMutableKeep[keyof DeepMutableKeep];

export declare interface FailedLog extends BaseLog {
    type: "failed";
    programId: PublicKey_2;
    details: string;
}

export declare type FeatureEvent<EventName extends string = string> = Tagged<EventName, "FeatureEvent">;

export declare type FeatureId<S extends string = string> = Tagged<S, "FeatureId">;

export declare type FeatureName<S extends string = string> = Tagged<S, "FeatureName">;

export declare const FED_CHAIR_PRESS_CONFERENCE: FedChairPressConferenceEvent;

export declare type FedChairPressConferenceEvent = EconomicEvent<"FedChairPressConference">;

export declare const FETCH_FAILED_MESSAGE: "fetch failed";

export declare interface FieldMapping<T = unknown> {
    path: string | string[];
    transform?: (value: unknown) => T | undefined;
    required?: boolean;
}

export declare const FIFTEEN_MINUTES_MS: DurationMs<900000>;

export declare const FIFTEEN_MINUTES_S: DurationS<900>;

export declare function filterAsciiPrintable(input: string, maxLen?: number): string;

declare type FilteredKeys<T> = Exclude<NonFnKeys<T>, ExcludedKeys<T>>;

export declare interface FilterParameters {
    page?: number;
    limit?: number;
    offset?: number;
    sortBy?: string;
    sortOrder?: SortOrder;
}

export declare const FIREFOX: Browser<"Firefox">;

export declare const FIRST_DIGITAL_USD: FirstDigitalUsd;

export declare const FIRST_DIGITAL_USD_NAME: FirstDigitalUsdName;

export declare const FIRST_DIGITAL_USD_SYMBOL: FirstDigitalUsdSymbol;

export declare type FirstDigitalUsd = TokenId<"first-digital-usd">;

export declare type FirstDigitalUsdName = TokenName<"First Digital USD">;

export declare type FirstDigitalUsdSolana = SolanaAddressInfo<"USDSwr9ApdHk5bvJKMjzff41FfuX8bSxdKcR81vTwcA", {
    tokenId: SolanaToken<FirstDigitalUsd>;
    decimals: 6;
    program: "Token";
}>;

export declare type FirstDigitalUsdSymbol = TokenSymbol<"FDUSD">;

export declare type FirstName = Tagged<Identity<string>, "FirstName">;

export declare const FIVE_MINUTES_MS: DurationMs<300000>;

export declare const FIVE_MINUTES_S: DurationS<300>;

export declare const FIVE_SECOND_MS: DurationMs<5000>;

export declare function fnv1aSubstringHash(str: string, sample?: number): string;

export declare const FOMC_DECISION: FomcDecisionEvent;

export declare const FOMC_MINUTES_RELEASE: FomcMinutesReleaseEvent;

export declare type FomcDecisionEvent = EconomicEvent<"FomcDecision">;

export declare type FomcMinutesReleaseEvent = EconomicEvent<"FomcMinutesRelease">;

export declare const FORBIDDEN_MESSAGE: "forbidden";

export declare const FOUR_HOURS_MS: DurationMs<14400000>;

export declare const FOUR_HOURS_S: DurationS<14400>;

export declare const FOUR_MINUTES_MS: DurationMs<240000>;

export declare const FOUR_MINUTES_S: DurationS<240>;

/** End of frame, i.e. "]", "}" */
export declare type FrameEndSpan = ["frame", "end"];

/** Start of frame, i.e. "[", "{" */
export declare type FrameStartSpan = ["frame", "start"];

export declare const FRANKFURT_TIMEZONE: FrankfurtTimezone;

export declare type FrankfurtTimezone = IANATimeZone<"Europe/Berlin">;

/** 10 = FreezeAccount {} */
export declare class FreezeAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    mint: SolanaAddressLike;
    authority: SolanaAddressLike;
    constructor(props: EncodableProps<FreezeAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): FreezeAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare type Frequency = "Minutely" | "TwoMinutely" | "FiveMinutely" | "TenMinutely" | "FifteenMinutely" | "ThirtyMinutely" | "FortyMinutely" | "FiftyMinutely" | "Hourly" | "TwoHourly" | "ThreeHourly" | "FourHourly" | "SixHourly" | "EightHourly" | "TwelveHourly" | "Daily" | "Weekly" | "Monthly" | "Quarterly" | "Yearly";

export declare const FREQUENCY_LIMIT_MESSAGE: "frequency limit";

export declare const FTSE100: FTSE100Name;

export declare type FTSE100Name = EquityIndexName<"FTSE 100">;

export declare type FTSE100Symbol = EquityIndexSymbol<"UKX">;

export declare const FTSEMIB: FTSEMIBName;

export declare type FTSEMIBName = EquityIndexName<"FTSE MIB">;

export declare type FTSEMIBSymbol = EquityIndexSymbol<"FTSEMIB">;

export declare type FullName = Tagged<Identity<string>, "FullName">;

export declare type FunctionProperties<T> = Pick<T, FunctionPropertyNames<T>>;

export declare type FunctionPropertyNames<T> = {
    [K in keyof T]: T[K] extends Function ? K : never;
}[keyof T];

export declare type FuturesTradingPair<TBase extends string = string, TQuote extends string = string> = Tagged<TradingPair<TBase, TQuote>, "FuturesTradingPair">;

export declare const GATEIO: GateIo;

export declare type GateIo = ExchangeName<"Gate.io">;

export declare type GateIoId = ExchangeId<GateIo>;

export declare const GDP_RELEASE: GDPReleaseEvent;

export declare type GDPReleaseEvent = EconomicEvent<"GDPRelease">;

export declare type GetAccountInfoResponse = AccountResponse<AccountInfo | null>;

export declare type GetBalanceResponse = AccountResponse<number>;

export declare function getFunctionName(fn: Function): string;

export declare interface GetLatestBlockhashAndContextResponse {
    value: GetLatestBlockhashResponse;
    context: {
        slot: SolanaSlot;
    };
}

export declare interface GetLatestBlockhashResponse {
    blockhash: SolanaBlockhash;
    lastValidBlockHeight: number;
}

export declare function getMockCalls(fn: {
    mock: {
        calls: unknown;
    };
}): MockCall[];

export declare type GetMultipleAccountsResponse = AccountResponse<(AccountInfo | null)[]>;

export declare type GetProgramAccountsResponse = {
    account: AccountInfo;
    pubkey: string;
}[];

export declare function getRandomElements<T>(arr: T[], count: 1): T;

export declare function getRandomElements<T>(arr: T[], count: number): T[];

export declare interface GetSignaturesForAddressConfig {
    limit?: number;
    before?: string;
    until?: string;
    commitment?: SolanaCommitmentLevel;
    minContextSlot?: number;
}

export declare type GetTokenAccountBalanceResponse = AccountResponse<TokenAccountBalance>;

export declare type GetTokenAccountsByOwnerResponse = AccountResponse<TokenAccountResponse[]>;

declare interface GetTokenAccountsConfig {
    mint?: SolanaAddressLike;
    owner?: SolanaAddressLike;
    page?: number;
    limit?: number;
    cursor?: string;
    before?: string;
    after?: string;
    options?: {
        showZeroBalance?: boolean;
    };
}

declare interface GetTokenAccountsEntry {
    address: SolanaAddress;
    mint: SolanaAddress;
    owner: SolanaAddress;
    amount: string | number;
    delegated_amount?: string | number;
    frozen?: boolean;
    burnt?: unknown;
}

declare interface GetTokenAccountsResponse {
    last_indexed_slot?: SolanaSlot;
    total?: number;
    limit?: number;
    cursor?: string | null;
    token_accounts: GetTokenAccountsEntry[];
}

export declare interface GetTransactionConfig {
    encoding?: SolanaRpcEncoding;
    commitment?: SolanaCommitmentLevel;
    maxSupportedTransactionVersion?: number;
}

export declare type GetTransactionResponse = TransactionData | null;

export declare interface GetTransactionsForAddressBlockTimeFilter {
    gte?: TimestampS;
    gt?: TimestampS;
    lte?: TimestampS;
    lt?: TimestampS;
    eq?: TimestampS;
}

export declare interface GetTransactionsForAddressConfig {
    transactionDetails?: EnhancedTransactionDetails;
    sortOrder?: EnhancedSortOrder;
    limit?: number;
    paginationToken?: string;
    commitment?: SolanaCommitmentLevel;
    filters?: GetTransactionsForAddressFilters;
    encoding?: SolanaRpcEncoding;
    maxSupportedTransactionVersion?: number;
    minContextSlot?: number;
}

export declare interface GetTransactionsForAddressFilters {
    slot?: GetTransactionsForAddressSlotFilter;
    blockTime?: GetTransactionsForAddressBlockTimeFilter;
    signature?: GetTransactionsForAddressSignatureFilter;
    status?: "succeeded" | "failed" | "any";
    tokenAccounts?: GetTransactionsForAddressTokenAccountsFilter;
    tokenTransfer?: GetTransactionsForAddressTokenTransferFilter;
}

export declare type GetTransactionsForAddressFullResponse = GetTransactionsForAddressResponseBase<TransactionResponse>;

export declare type GetTransactionsForAddressResponse = GetTransactionsForAddressSignaturesResponse | GetTransactionsForAddressFullResponse;

export declare interface GetTransactionsForAddressResponseBase<TEntry> {
    data: TEntry[];
    paginationToken?: string | null;
}

export declare interface GetTransactionsForAddressSignatureFilter {
    gte?: SolanaTransactionSignature;
    gt?: SolanaTransactionSignature;
    lte?: SolanaTransactionSignature;
    lt?: SolanaTransactionSignature;
}

export declare type GetTransactionsForAddressSignaturesResponse = GetTransactionsForAddressResponseBase<ConfirmedSignatureInfo>;

export declare interface GetTransactionsForAddressSlotFilter {
    gte?: SolanaSlot;
    gt?: SolanaSlot;
    lte?: SolanaSlot;
    lt?: SolanaSlot;
}

declare type GetTransactionsForAddressTokenAccountsFilter = "none" | "balanceChanged" | "all";

declare interface GetTransactionsForAddressTokenTransferAmountFilter {
    gte?: string | number;
    gt?: string | number;
    lte?: string | number;
    lt?: string | number;
}

declare type GetTransactionsForAddressTokenTransferDirection = "in" | "out" | "any";

declare interface GetTransactionsForAddressTokenTransferFilter {
    with?: SolanaAddressLike;
    direction?: GetTransactionsForAddressTokenTransferDirection;
    mint?: SolanaAddressLike;
    amount?: GetTransactionsForAddressTokenTransferAmountFilter;
}

export declare const GOOD_FRIDAY: GoodFriday;

export declare type GoodFriday = Holiday<"Good Friday">;

/** Extract group from grouped spans def */
declare type GroupedSpansGroup<GroupedSpans> = GroupedSpans extends [infer Group, ...any[]] ? Group : never;

/** Extract spans from grouped spans def */
declare type GroupedSpansSpans<GroupedSpans> = GroupedSpans extends [any, ...infer Spans] ? Spans : [];

/** Ends a group (pops a group frame). */
export declare type GroupEndOp = 10;

/** Enters the current group frame (renderer may increase indentation here). */
export declare type GroupEnterOp = 7;

/** Exit marker emitted at the end of a group's final line (renderer-defined semantics). */
export declare type GroupExitEndOp = 9;

/** Exits the current group frame (renderer may decrease indentation here). */
export declare type GroupExitOp = 8;

/** A group's constrained spans: [group, ...spans] */
declare type GroupSpansDef = [string, ...SpanDef[]];

/** Starts a group and immediately enters it (start + enter). */
export declare type GroupStartEnterOp = 6;

/** Starts a group (pushes a group frame). */
export declare type GroupStartOp = 5;

declare type Gte<L extends number, Min extends number> = Build<L> extends [...Build<Min>, ...any[]] ? true : false;

export declare const HANG_SENG_INDEX: HangSengIndexName;

export declare type HangSengIndex = EquityIndexId<"hang-seng-index">;

export declare type HangSengIndexName = EquityIndexName<"Hang Seng Index">;

export declare type HangSengIndexSymbol = EquityIndexSymbol<"HSI">;

export declare type Hash = Tagged<string, "Hash">;

export declare type HashedPassword = Tagged<Password & (Sha256Hash | Sha512Hash | BcryptHash), "HashedPassword">;

export declare function hasNonNullishProperty<T extends object, K extends keyof T>(obj: T | null | undefined, key: K): obj is T & {
    [P in K]-?: NonNullable<T[P]>;
};

export declare function hasNonNullishProperty<T extends object, K extends keyof T>(obj: T, key: K): obj is T & {
    [P in K]-?: NonNullable<T[P]>;
};

export declare function hasNonNullishProperty<K extends string | number | symbol>(obj: unknown, key: K): obj is Record<K, NonNullable<unknown>> & object;

export declare function hasProperties<T extends object, K extends StrArray>(obj: T, keys: K): obj is T & Record<K[number], unknown>;

export declare function hasProperty<T extends object, K extends string>(obj: T, key: K): obj is T & Record<K, unknown>;

export declare function hasSingleProperty(obj: unknown): obj is AnySingleProperty;

export declare type HasToString = string | {
    toString(): string;
};

export declare const HEADER_ACCEPT: "accept";

export declare const HEADER_ACCEPT_ENCODING: "accept-encoding";

export declare const HEADER_ACCEPT_LANGUAGE: "accept-language";

export declare const HEADER_ACCESS_CONTROL_ALLOW_CREDENTIALS: "access-control-allow-credentials";

export declare const HEADER_ACCESS_CONTROL_ALLOW_HEADERS: "access-control-allow-headers";

export declare const HEADER_ACCESS_CONTROL_ALLOW_METHODS: "access-control-allow-methods";

export declare const HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: "access-control-allow-origin";

export declare const HEADER_ACCESS_CONTROL_EXPOSE_HEADERS: "access-control-expose-headers";

export declare const HEADER_ACCESS_CONTROL_MAX_AGE: "access-control-max-age";

export declare const HEADER_AUTHORIZATION: "authorization";

export declare const HEADER_CACHE_CONTROL: "cache-control";

export declare const HEADER_CONNECTION: "connection";

export declare const HEADER_CONTENT_ENCODING: "content-encoding";

export declare const HEADER_CONTENT_LENGTH: "content-length";

export declare const HEADER_CONTENT_SECURITY_POLICY: "content-security-policy";

export declare const HEADER_CONTENT_TYPE: "content-type";

export declare const HEADER_COOKIE: "cookie";

export declare const HEADER_DATE: "date";

export declare const HEADER_DNT: "dnt";

export declare const HEADER_ETAG: "etag";

export declare const HEADER_EXPIRES: "expires";

export declare const HEADER_HOST: "host";

export declare const HEADER_IF_MODIFIED_SINCE: "if-modified-since";

export declare const HEADER_IF_NONE_MATCH: "if-none-match";

export declare const HEADER_LAST_MODIFIED: "last-modified";

export declare const HEADER_LOCATION: "location";

export declare const HEADER_ORIGIN: "origin";

export declare const HEADER_PRAGMA: "pragma";

export declare const HEADER_REFERER: "referer";

export declare const HEADER_RETRY_AFTER: "retry-after";

export declare const HEADER_SEC_CH_UA: "sec-ch-ua";

export declare const HEADER_SEC_CH_UA_MOBILE: "sec-ch-ua-mobile";

export declare const HEADER_SEC_CH_UA_PLATFORM: "sec-ch-ua-platform";

export declare const HEADER_SEC_FETCH_DEST: "sec-fetch-dest";

export declare const HEADER_SEC_FETCH_MODE: "sec-fetch-mode";

export declare const HEADER_SEC_FETCH_SITE: "sec-fetch-site";

export declare const HEADER_SEC_FETCH_USER: "sec-fetch-user";

export declare const HEADER_SERVER: "server";

export declare const HEADER_SET_COOKIE: "set-cookie";

export declare const HEADER_STRICT_TRANSPORT_SECURITY: "strict-transport-security";

export declare const HEADER_UPGRADE_INSECURE_REQUESTS: "upgrade-insecure-requests";

export declare const HEADER_USER_AGENT: "user-agent";

export declare const HEADER_VARY: "vary";

export declare const HEADER_X_CONTENT_TYPE_OPTIONS: "x-content-type-options";

export declare const HEADER_X_FORWARDED_FOR: "x-forwarded-for";

export declare const HEADER_X_FRAME_OPTIONS: "x-frame-options";

export declare const HEADER_X_MBX_USED_WEIGHT_1M: "x-mbx-used-weight-1m";

export declare const HEADER_X_RATE_LIMIT_LIMIT: "x-ratelimit-limit";

export declare const HEADER_X_RATE_LIMIT_METHOD_LIMIT: "x-ratelimit-method-limit";

export declare const HEADER_X_RATE_LIMIT_METHOD_REMAINING: "x-ratelimit-method-remaining";

export declare const HEADER_X_RATE_LIMIT_REMAINING: "x-ratelimit-remaining";

export declare const HEADER_X_RATE_LIMIT_RESET: "x-ratelimit-reset";

export declare const HEADER_X_RATE_LIMIT_RPS_LIMIT: "x-ratelimit-rps-limit";

export declare const HEADER_X_RATE_LIMIT_RPS_REMAINING: "x-ratelimit-rps-remaining";

export declare const HEADER_X_REAL_IP: "x-real-ip";

export declare const HEADER_X_XSS_PROTECTION: "x-xss-protection";

export declare type HeadersInit = StrTupleArray | StringMap | Headers;

export declare const HELIUS_DEVNET_ENDPOINT: Url<"https://devnet.helius-rpc.com/">;

export declare const HELIUS_MAINNET_ENDPOINT: Url<"https://mainnet.helius-rpc.com/">;

/**
 * Inline transaction payload when transactionDetails = "accounts" or "full".
 * accountKeys reuses ParsedAccountKey (same shape: pubkey, signer, source, writable).
 * meta reuses TransactionMeta.
 * Base structure extends TransactionResponse for version/slot/blockTime fields.
 */
export declare type HeliusInlineTransaction = TransactionResponse & {
    transaction: {
        message: {
            accountKeys: ParsedAccountKey[];
        };
        signatures: SolanaTransactionSignature[];
    };
    meta: TransactionMeta | null;
};

/**
 * Helius-enhanced WebSocket client.
 *
 * Extends {@link RpcWebsocketClient} with `transactionSubscribe` /
 * `transactionUnsubscribe` - Helius-specific methods that are not part of
 * the standard Solana RPC websocket spec.
 *
 * Requires a Helius endpoint:
 *   wss://atlas-mainnet.helius-rpc.com/?api-key=YOUR_KEY
 *
 * @example
 * ```typescript
 * const ws = new HeliusRpcWebsocketClient({
 *     url: "wss://atlas-mainnet.helius-rpc.com/?api-key=abc123"
 * });
 *
 * await ws.connect();
 *
 * const subId = await ws.transactionSubscribe(
 *     { accountInclude: ["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"] },
 *     (notification) => console.log(notification.value.signature),
 *     { commitment: "finalized", transactionDetails: "signatures", failed: false }
 * );
 *
 * await ws.transactionUnsubscribe(subId);
 * ws.disconnect();
 * ```
 */
export declare class HeliusRpcWebsocketClient extends RpcWebsocketClient {
    /**
     * Subscribe to transactions matching the given account filter.
     *
     * @param filter   - accountInclude / accountExclude / accountRequired
     * @param callback - Called for each matching transaction notification
     * @param config   - Commitment, encoding, detail level, failed flag, etc.
     * @returns Subscription ID for unsubscribing
     */
    transactionSubscribe(filter: HeliusTransactionFilter, callback: (notification: HeliusTransactionNotification) => void, config?: HeliusTransactionSubscribeConfig): Promise<SubscriptionId>;
    /**
     * Unsubscribe from Helius transaction updates.
     */
    transactionUnsubscribe(subscriptionId: SubscriptionId): Promise<boolean>;
}

/**
 * Helius transactionSubscribe account filter.
 * All fields are ANDed when multiple are specified.
 */
export declare type HeliusTransactionFilter = {
    accountInclude?: SolanaAddress[];
    accountExclude?: SolanaAddress[];
    accountRequired?: SolanaAddress[];
};

export declare type HeliusTransactionNotification = {
    slot: SolanaSlot;
    signature: SolanaTransactionSignature;
    /** Present when transactionDetails != "none" and no error */
    transaction?: HeliusInlineTransaction;
    /** Present when the RPC could not return the transaction (e.g. UnsupportedTransactionVersion) */
    error?: unknown;
    transactionIndex?: number;
};

export declare type HeliusTransactionSubscribeConfig = {
    commitment?: SolanaCommitmentLevel;
    encoding?: SolanaRpcEncoding;
    transactionDetails?: EnhancedTransactionDetails;
    showRewards?: boolean;
    maxSupportedTransactionVersion?: number;
    /** Include failed transactions (default: false) */
    failed?: boolean;
};

export declare type Hex<S extends string = string, Min extends number = number, Max extends number | undefined = undefined> = HexLower<S, Min, Max> | HexUpper<S, Min, Max>;

export declare type HexLower<S extends string = string, Min extends number = number, Max extends number | undefined = undefined> = Tagged<Encoded & ValidHexLower<S, Min, Max>, "HexLower">;

declare type HexLowerDigit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "a" | "b" | "c" | "d" | "e" | "f";

declare type _HexLowerOK<S extends string, Min extends number, Max extends number | undefined> = ValidString<S, HexLowerDigit, Min, Max, true>;

/**
 * Parse a hex string (with or without 0x prefix) into a discriminator-sized Uint8Array.
 * If the length is not exactly 8 bytes, this will throw.
 */
export declare function hexToDiscriminator(hex: string): Uint8Array;

export declare type HexUpper<S extends string = string, Min extends number = number, Max extends number | undefined = undefined> = Tagged<Encoded & ValidHexUpper<S, Min, Max>, "HexUpper">;

declare type HexUpperDigit = Uppercase<HexLowerDigit>;

declare type _HexUpperOK<S extends string, Min extends number, Max extends number | undefined> = ValidString<S, HexUpperDigit, Min, Max, true>;

export declare const HKEX: HKEX_2;

declare type HKEX_2 = ExchangeName<"HKEX">;

export declare type HKEXId = ExchangeId<HKEX_2>;

export declare type HmacSecret<S extends string = string> = Tagged<ApiSecret<S>, "HmacSecret">;

export declare type Holiday<HolidayName extends string = string> = ObservedDate<HolidayName>;

export declare interface HolidayEntry extends BaseCalendarEntry {
    type: "Holiday";
    bankHoliday?: boolean;
    tradingHoliday?: boolean;
    settlementHoliday?: boolean;
}

export declare const HONG_KONG_TIMEZONE: HongKongTimezone;

export declare type HongKongTimezone = IANATimeZone<"Asia/Hong_Kong">;

export declare type HostName<S extends string = string> = Tagged<S, "HostName">;

export declare const HTTP_ACCEPTED: 202;

export declare const HTTP_BAD_GATEWAY: 502;

export declare const HTTP_BAD_REQUEST: 400;

export declare const HTTP_CONFLICT: 409;

export declare const HTTP_CONNECT: "CONNECT";

export declare const HTTP_CONTINUE: 100;

export declare const HTTP_CREATED: 201;

export declare const HTTP_DELETE: "DELETE";

export declare const HTTP_FORBIDDEN: 403;

export declare const HTTP_FOUND: 302;

export declare const HTTP_GATEWAY_TIMEOUT: 504;

export declare const HTTP_GET: "GET";

export declare const HTTP_GONE: 410;

export declare const HTTP_HEAD: "HEAD";

export declare const HTTP_I_AM_A_TEAPOT: 418;

export declare const HTTP_INTERNAL_SERVER_ERROR: 500;

export declare const HTTP_METHOD_NOT_ALLOWED: 405;

export declare const HTTP_MOVED_PERMANENTLY: 301;

export declare const HTTP_NO_CONTENT: 204;

export declare const HTTP_NOT_ACCEPTABLE: 406;

export declare const HTTP_NOT_FOUND: 404;

export declare const HTTP_NOT_IMPLEMENTED: 501;

export declare const HTTP_NOT_MODIFIED: 304;

export declare const HTTP_OK: 200;

export declare const HTTP_OPTIONS: "OPTIONS";

export declare const HTTP_PATCH: "PATCH";

export declare const HTTP_PAYLOAD_TOO_LARGE: 413;

export declare const HTTP_PERMANENT_REDIRECT: 308;

export declare const HTTP_POST: "POST";

export declare const HTTP_PRECONDITION_FAILED: 412;

export declare const HTTP_PUT: "PUT";

export declare const HTTP_REQUEST_TIMEOUT: 408;

export declare const HTTP_SCHEME: "http";

export declare const HTTP_SERVICE_UNAVAILABLE: 503;

export declare const HTTP_SWITCHING_PROTOCOLS: 101;

export declare const HTTP_TEMPORARY_REDIRECT: 307;

export declare const HTTP_TOO_MANY_REQUESTS: 429;

export declare const HTTP_TRACE: "TRACE";

export declare const HTTP_UNAUTHORIZED: 401;

export declare const HTTP_UNPROCESSABLE_ENTITY: 422;

export declare const HTTP_UNSUPPORTED_MEDIA_TYPE: 415;

export declare type HttpHeaderName = string;

export declare type HttpHeaderValue = string;

export declare type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD" | "OPTIONS" | "TRACE" | "CONNECT";

export declare type HttpMethodLowercase = Lowercase<HttpMethod>;

export declare interface HttpReply {
    code(statusCode: HttpStatusCode): HttpReply;
    header(key: string, value: string): HttpReply;
    send(payload: unknown): HttpReply;
    type(contentType: string): HttpReply;
}

export declare const HTTPS_SCHEME: "https";

export declare type HttpScheme = "http" | "https";

export declare type HttpStatusCode = 100 | 101 | 200 | 201 | 202 | 204 | 301 | 302 | 304 | 307 | 308 | 400 | 401 | 403 | 404 | 405 | 406 | 408 | 409 | 410 | 412 | 413 | 415 | 418 | 422 | 429 | 500 | 501 | 502 | 503 | 504;

export declare const HTX: HTX_2;

declare type HTX_2 = ExchangeName<"HTX">;

export declare type HTXId = ExchangeId<HTX_2>;

export declare const HYPERLIQUID: Hyperliquid;

export declare type Hyperliquid = ExchangeName<"Hyperliquid"> & BlockchainProtocolName<"Hyperliquid">;

export declare type HyperliquidId = ExchangeId<Hyperliquid> & BlockchainProtocolId<Hyperliquid>;

export declare type I16<int16 extends number = number> = Tagged<WholeNumberPattern<int16>, "int16">;

export declare type I16Bytes = Bytes<"i16">;

export declare type I24<int24 extends number = number> = Tagged<WholeNumberPattern<int24>, "int24">;

export declare type I24Bytes = Bytes<"i24">;

export declare type I32<int32 extends number = number> = Tagged<WholeNumberPattern<int32>, "int32">;

export declare type I32Bytes = Bytes<"i32">;

export declare type I8<int8 extends number = number> = Tagged<WholeNumberPattern<int8>, "int8">;

export declare type I8Bytes = Bytes<"i8">;

export declare type IANATimeZone<TimeZone extends string = string> = Tagged<TimeZone, "IANATimeZone">;

export declare const IBEX35: IBEX35Name;

export declare type IBEX35Name = EquityIndexName<"IBEX 35">;

export declare type IBEX35Symbol = EquityIndexSymbol<"IBEX">;

export declare const ICE: ICE_2;

declare type ICE_2 = ExchangeName<"ICE">;

export declare type ICEId = ExchangeId<ICE_2>;

/** An icon definition: [type] or [type, variant] or [type, undefined] */
declare type IconDef = [string] | [string, string] | [string, undefined];

/** An icon, could be an image could be ANSI. */
export declare type IconOp = 1;

/** Extract type from icon def */
declare type IconType<Icon> = Icon extends [infer IconTypeValue, ...any[]] ? IconTypeValue : never;

/**
 * Extract variant from icon def:
 * - ["type"] (length 1) → string (any variant allowed)
 * - ["type", Variant] where Variant is string → Variant (only that variant)
 * - ["type", undefined] → undefined (no variant allowed)
 */
declare type IconVariant<Icon, Type extends string> = Icon extends [Type, infer Variant] ? Variant : Icon extends [Type] ? string : never;

export declare type Id<T extends number | string = number> = Tagged<Identity<T>, "Id">;

declare interface IDBDatabase {
    name: string;
    version: number;
    objectStoreNames: DOMStringList;
    transaction(storeNames: string | string[], mode?: IDBTransactionMode): IDBTransaction;
    createObjectStore(name: string, options?: IDBObjectStoreParameters): IDBObjectStore;
    close(): void;
}

export declare interface IDBFactory {
    open(name: string, version?: number): IDBOpenDBRequest;
}

declare interface IDBObjectStore {
    get(key: any): IDBRequest;
    put(value: any, key?: any): IDBRequest;
    delete(key: any): IDBRequest;
    getAllKeys(): IDBRequest<any[]>;
    clear(): IDBRequest;
}

declare interface IDBObjectStoreParameters {
    keyPath?: string | string[];
    autoIncrement?: boolean;
}

declare interface IDBOpenDBRequest extends IDBRequest<IDBDatabase> {
    onupgradeneeded: ((event: Event) => void) | null;
}

declare interface IDBRequest<T = any> {
    result: T;
    error: DOMException | null;
    onsuccess: ((event: Event) => void) | null;
    onerror: ((event: Event) => void) | null;
}

declare interface IDBTransaction {
    objectStore(name: string): IDBObjectStore;
    abort(): void;
}

declare type IDBTransactionMode = "readonly" | "readwrite" | "versionchange";

export declare type Identity<T extends number | string = number> = Tagged<T, "Identity">;

export declare const IE: Browser<"IE">;

declare interface ILBPair {
    address: PublicKeyLike;
    parameters: PublicKeyLike;
    vParameters: PublicKeyLike;
    bumpSeed: Uint8Array;
    binStep: number;
    pairType: number;
    activeId: number;
    binStepSeed: Uint8Array;
    tokenXMint: PublicKeyLike;
    tokenYMint: PublicKeyLike;
    reserveX: PublicKeyLike;
    reserveY: PublicKeyLike;
    protocolFee: unknown;
    feeOwner: PublicKeyLike;
    rewardInfos: unknown[];
    oracle: PublicKeyLike;
    binArrayBitmap: Uint8Array[];
}

export declare type ImageMimeType = "image/png" | "image/jpeg" | "image/gif" | "image/svg+xml" | "image/webp" | "image/x-icon";

export declare type Impact = "None" | "Very Low" | "Low" | "Medium" | "High" | "Very High" | "Extreme" | "Full";

export declare const INDEPENDENCE_DAY: IndependenceDay;

export declare type IndependenceDay = Holiday<"Independence Day">;

export declare const INDEXED_DB_AVAILABLE: boolean;

/** Index span, i.e. current stack index beginning at zero and goes up */
export declare type IndexSpan = ["index"];

/** Information typing, i.e. invoke for logs, virtual for virtualized items */
export declare type InfoType = "info" | "virtual";

export declare const INITIAL_JOBLESS_CLAIMS: InitialJoblessClaimsEvent;

/** 18 = InitializeAccount3 { owner: Pubkey } */
export declare class InitializeAccount3Instruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    mint: SolanaAddressLike;
    owner: PublicKeyLike;
    constructor(props: EncodableProps<InitializeAccount3Instruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): InitializeAccount3Instruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/** 22 = InitializeImmutableOwner { } */
export declare class InitializeImmutableOwnerInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    constructor(props: EncodableProps<InitializeImmutableOwnerInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): InitializeImmutableOwnerInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/** 20 = InitializeMint2 { decimals: u8, mintAuthority: Pubkey, freezeAuthority: COption<Pubkey> } */
export declare class InitializeMint2Instruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    mint: SolanaAddressLike;
    decimals: number;
    mintAuthority: PublicKeyLike;
    freezeAuthority?: PublicKeyLike | null;
    constructor(props: EncodableProps<InitializeMint2Instruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): InitializeMint2Instruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/**
 * 6 = InitializeNonceAccount { authorized: Pubkey }
 */
export declare class InitializeNonceAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    nonceAccount: SolanaAddressLike;
    recentBlockhashSysvar: SolanaAddressLike;
    rentSysvar: SolanaAddressLike;
    authorized: PublicKeyLike;
    constructor(props: EncodableProps<InitializeNonceAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): InitializeNonceAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare function initializeStaticAddresses(): void;

export declare type InitialJoblessClaimsEvent = EconomicEvent<"InitialJoblessClaims">;

export declare function installConnectTrace(filterHost: string, filterPort: number): () => void;

export declare type InstanceOf<T> = T extends Constructor<infer R> ? R : T;

export declare interface InstructionAccount {
    pubkey: PublicKeyLike;
    isSigner: boolean;
    isWritable: boolean;
}

export declare type InstructionEvent = BaseInstructionEvent | AnchorEventCpiInstructionEvent | TokenInstructionNoDetailsEvent | TokenInstructionEvent;

export declare interface InstructionMatcher<Instruction extends IsEncodableStatic = IsEncodableStatic> {
    programId: PublicKey;
    discriminator: Uint8Array | null;
    instructionClass: Instruction;
}

export declare class InstructionParser {
    private static readonly registered;
    private static decodeAnchorEventCpiInstructions;
    private static readonly EVENT_CPI_DISCRIMINATOR;
    private static readonly defaultIgnoredProgramIds;
    private static ignoredProgramIds;
    private static decodeIgnoredPrograms;
    private static readonly EMPTY_BYTES;
    private static readonly decodedInstructionDataCache;
    private static sortMatchers;
    /**
     * Register instruction matchers
     * Stores per programId, maintains insertion order within equal discriminator length,
     * and sorts by discriminator length (desc).
     */
    static register(...matchers: InstructionMatcher[]): void;
    /**
     * Clear all registered matchers
     */
    static clearAll(): void;
    static setDecodeAnchorEventCpiInstructions(enable: boolean): void;
    static setDecodeIgnoredPrograms(enable: boolean): void;
    static addIgnoredProgramId(programId: PublicKey): void;
    static clearIgnoredProgramIdsToDefault(): void;
    static clearAllIgnoredProgramIds(): void;
    static isIgnoredProgramId(programId: PublicKey): boolean;
    /**
     * Get all registered instruction matchers
     */
    static getRegistered(): Map<string, InstructionMatcher[]>;
    /**
     * Get all registered instruction matchers for a given program
     */
    static getRegisteredForProgram(programId: PublicKey): InstructionMatcher[];
    /**
     * First pass: Parse transaction instructions into ParsedInstruction objects
     */
    static parse(transaction: Transaction): ParsedInstruction[];
    static isAlreadyEncodable(instruction: unknown): instruction is IsEncodable;
    static isEncodedInstruction(instruction: unknown): instruction is EncodedInstruction;
    /**
     * Second pass: Decode parsed instruction into concrete type
     */
    static decode(parsedInstruction: ParsedInstruction | MessageInstruction): IsEncodable | null;
    static registerInstructions(programId: PublicKey, classes: IsEncodableStatic[]): void;
    static registerSplTokenInstructions(): void;
}

/**
 * Error thrown when data being decoded isn't long enough for the expected field size.
 */
export declare class InsufficientDataDecoderError extends DecoderError {
    readonly type = "INSUFFICIENT_DATA";
    constructor(message?: string, originalError?: unknown);
}

/**
 * Error thrown when data being encoded isn't long enough for the expected field size.
 */
export declare class InsufficientDataEncoderError extends EncoderError {
    readonly type = "INSUFFICIENT_DATA";
    constructor(message?: string, originalError?: unknown);
}

/**
 * Thrown when attempting to sign with insufficient signers
 */
export declare class InsufficientSignersError extends TransactionError {
    readonly requiredSigners?: number | undefined;
    readonly providedSigners?: number | undefined;
    readonly missingSignerAddresses?: SolanaAddress[] | undefined;
    readonly type = "INSUFFICIENT_SIGNERS";
    constructor(message?: string, requiredSigners?: number | undefined, providedSigners?: number | undefined, missingSignerAddresses?: SolanaAddress[] | undefined, originalError?: unknown);
}

export declare function interpolateTemplate(input: string, params: Record<string, unknown>): string;

export declare type IntersectArrayElements<T> = (T extends {
    [n: number]: infer E;
} ? (x: E) => void : never) extends (x: infer I) => void ? I : never;

export declare const INVALID_CREDENTIALS_MESSAGE: "invalid credentials";

export declare const INVALID_FORMAT_MESSAGE: "invalid format";

export declare const INVALID_INPUT_MESSAGE: "invalid input";

export declare const INVALID_TOKEN_MESSAGE: "invalid token";

/**
 * Thrown when instruction account meta is invalid
 */
export declare class InvalidAccountMetaError extends TransactionError {
    readonly accountAddress?: SolanaAddress | undefined;
    readonly reason?: string | undefined;
    readonly type = "INVALID_ACCOUNT_META";
    constructor(message?: string, accountAddress?: SolanaAddress | undefined, reason?: string | undefined, originalError?: unknown);
}

/**
 * Error thrown when invalid base58 characters are encountered
 */
export declare class InvalidBase58Error extends Base58Error {
    readonly invalidCharacter?: string | undefined;
    readonly originalError?: unknown | undefined;
    readonly type = "INVALID_BASE58";
    constructor(message?: string, invalidCharacter?: string | undefined, originalError?: unknown | undefined);
}

/**
 * Thrown when recent blockhash is invalid or expired
 */
export declare class InvalidBlockhashError extends TransactionError {
    readonly blockhash?: Base58 | undefined;
    readonly type = "INVALID_BLOCKHASH";
    constructor(message?: string, blockhash?: Base58 | undefined, originalError?: unknown);
}

/**
 * Thrown when program ID is invalid, i.e. it is on-curve
 */
export declare class InvalidProgramIdError extends PublicKeyError {
    readonly programId?: SolanaAddressLike | undefined;
    readonly reason?: string | undefined;
    readonly type = "INVALID_PROGRAM_ID";
    constructor(message?: string, programId?: SolanaAddressLike | undefined, reason?: string | undefined, originalError?: unknown);
}

/**
 * Error thrown when invalid input is provided to PublicKey constructor
 */
export declare class InvalidPublicKeyInputError extends PublicKeyError {
    readonly providedValue?: unknown | undefined;
    readonly type = "INVALID_INPUT";
    constructor(message?: string, providedValue?: unknown | undefined, originalError?: unknown);
}

/**
 * Error thrown when public key has invalid length
 */
export declare class InvalidPublicKeyLengthError extends PublicKeyError {
    readonly actualLength: number;
    readonly expectedLength: number;
    readonly type = "INVALID_LENGTH";
    constructor(actualLength: number, expectedLength?: number, originalError?: unknown);
}

/**
 * Thrown when transaction signature verification fails
 */
export declare class InvalidSignatureError extends TransactionError {
    readonly signerAddress?: SolanaAddress | undefined;
    readonly type = "INVALID_SIGNATURE";
    constructor(message?: string, signerAddress?: SolanaAddress | undefined, originalError?: unknown);
}

export declare type InverseTradingPair<TBase extends string = string, TQuote extends string = string> = Tagged<PerpetualTradingPair<TBase, TQuote> | DeliveryTradingPair<TBase, TQuote>, "InverseTradingPair">;

export declare interface InvokeLog extends BaseLog {
    type: "invoke";
    programId: PublicKey_2;
    depth: number;
}

export declare const IOS: OperatingSystem<"iOS">;

export declare const IP_EMPTY: IpAddress<"0.0.0.0">;

export declare const IP_LOOPBACK: IpAddress<"127.0.0.1">;

export declare type IpAddress<S extends string = string> = Tagged<S, "IpAddress">;

export declare function ipInCidr(ip: IpAddress, cidr: string): boolean;

export declare const IPV6_EMPTY: IpAddress<"::">;

export declare const IPV6_LOOPBACK: IpAddress<"::1">;

export declare const IS_NODE: boolean;

declare type IsAllDigits<S extends string> = S extends `${infer H}${infer Rest}` ? H extends "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ? IsAllDigits<Rest> : false : true;

export declare type IsArrayish<T> = T extends string ? false : T extends ArrayBufferView ? false : T extends {
    [n: number]: unknown;
} ? true : false;

export declare function isAsciiPrintable(input: string, maxLen?: number, minLen?: number): boolean;

export declare function isAsyncFunction(fn: unknown): fn is AsyncFunction;

export declare function isAsyncGeneratorFunction(fn: unknown): fn is AsyncGeneratorFunction;

export declare type IsBrandedType<T> = T extends BrandedInterface<infer _B, any> ? true : false;

export declare function isCamelCase(value: string): value is CamelCase;

export declare interface IsCodable {
}

export declare interface IsCodableStatic {
    new (...args: any[]): IsCodable;
    getSchema(): Schema;
}

export declare function isCoreProgram(address: string): boolean;

export declare interface IsDecodable {
}

export declare interface IsDecodableStatic {
    new (...args: any[]): IsDecodable;
    getSchema(): Schema;
    decode?(data: string | Uint8Array | Buffer | null | undefined, address?: SolanaAddressLike): IsDecodable;
    readonly programDerivedAccounts?: ProgramDerivedAccountSchema;
    readonly accountRelations?: AccountRelationsSchema;
    readonly decodePolicy?: DecodeDataPolicy;
}

export declare function isDuration(value: unknown): value is Duration;

export declare function isDurationMs(value: unknown): value is DurationMs;

export declare function isDurationNs(value: unknown): value is DurationNs;

export declare function isDurationS(value: unknown): value is DurationS;

export declare interface IsEncodable {
    programId: PublicKeyLike;
    encode(): Uint8Array;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
}

export declare interface IsEncodableStatic {
    readonly name: string & EncodableDefault;
    new (...args: any[]): IsEncodable;
    getSchema(): Schema;
    getAccountsSchema(): EncodeAccountsSchemaOrNull;
    decode?(instruction: DecodableInstruction): IsEncodable;
    readonly programDerivedAccounts?: ProgramDerivedAccountSchema;
    readonly accountRelations?: AccountRelationsSchema;
    /** If true, InstructionParser calls class.decode(data, accounts) directly */
    readonly customDecode?: boolean;
    /** Bytes to skip in Decoder (instead of discriminator.length) */
    readonly nativeDiscriminatorLength?: number;
}

/** True when the string length is even (multiple of 2). */
declare type IsEvenLength<S extends string> = S extends `${infer _A}${infer _B}${infer Rest}` ? IsEvenLength<Rest> : S extends "" ? true : false;

export declare function isFunction(fn: unknown): fn is Function;

export declare function isGeneratorFunction(fn: unknown): fn is GeneratorFunction;

export declare function isIterable<T>(obj: unknown): obj is Iterable<T>;

export declare function isKebabCase(value: string): value is KebabCase;

declare type IsKept<T, K> = [T] extends [K] ? true : false;

export declare function isKnownProgramAddress(address: string): boolean;

export declare function isLegacyJsonTransaction(tx?: unknown | null): tx is LegacyJsonTransactionData;

export declare function isLegacyTransaction(tx?: unknown | null): tx is LegacyJsonTransactionData | (RawTransactionData & {
    version: "legacy";
});

export declare interface IsLogDecodable {
}

export declare interface IsLogDecodableStatic {
    new (...args: any[]): IsLogDecodable;
    readonly discriminator: Uint8Array;
    getSchema(): Schema;
    decode(data: string | Uint8Array | Buffer | null | undefined, address?: SolanaAddressLike): IsLogDecodable;
    /**
     * Optional: Prefix that appears after "Program log: " for this log type.
     * Example: "ray_log: " for Raydium AMM logs.
     * If set, LogParser will match logs with this prefix.
     */
    readonly logPrefix?: string;
    /**
     * Optional: Encoding of the payload after the prefix.
     * Defaults to "base64" if not specified.
     */
    readonly logEncoding?: "base64" | "base58" | "hex";
    /**
     * Optional: Number of bytes to skip before reading schema fields.
     * If set, Decoder uses this instead of discriminator.length.
     *
     * Example: Raydium AMM uses 1-byte native discriminator, so set to 1.
     * The 8-byte `discriminator` is still used for registry lookup.
     */
    readonly nativeDiscriminatorLength?: number;
    readonly decodePolicy?: DecodeDataPolicy;
}

export declare function isMainnetEndpoint(endpoint: string): boolean;

export declare type IsNever<T> = [T] extends [never] ? true : false;

export declare function isNumberArray(arg: unknown): arg is number[];

export declare function isObject(obj: unknown): obj is Metadata;

export declare function isObject<T extends object>(obj: T | null | undefined): obj is T;

export declare function isObjectWithProperties(obj: unknown): obj is {
    properties: Metadata;
    required?: StrArray;
};

export declare type IsoDate<S extends string = string> = Tagged<DateString & IsoDatePattern<S>, "IsoDate">;

export declare type IsoDatePattern<T extends string> = T extends `${infer Y}-${infer M}-${infer D}` ? Len<Y> extends 4 ? IsAllDigits<Y> extends true ? Len<M> extends 2 ? IsAllDigits<M> extends true ? Len<D> extends 2 ? IsAllDigits<D> extends true ? T : never : never : never : never : never : never : never;

export declare type IsoDateTime<S extends string = string> = Tagged<DateTime & IsoDateTimePattern<S>, "IsoDateTime">;

export declare type IsoDateTimePattern<T extends string> = T extends `${infer Y}-${infer M}-${infer D}T${infer H}:${infer N}:${infer S}.${infer Ms}Z` ? Len<Y> extends 4 ? IsAllDigits<Y> extends true ? Len<M> extends 2 ? IsAllDigits<M> extends true ? Len<D> extends 2 ? IsAllDigits<D> extends true ? Len<H> extends 2 ? IsAllDigits<H> extends true ? Len<N> extends 2 ? IsAllDigits<N> extends true ? Len<S> extends 2 ? IsAllDigits<S> extends true ? Len<Ms> extends 3 ? IsAllDigits<Ms> extends true ? T : never : never : never : never : never : never : never : never : never : never : never : never : never : never : never;

declare type IsOptional<T, K extends keyof T> = undefined extends T[K] ? true : false;

export declare type IsoTime<S extends string = string> = Tagged<Time & IsoTimePattern<S>, "IsoTime">;

export declare type IsoTimePattern<T extends string> = T extends `${infer H}:${infer N}:${infer S}` ? Len<H> extends 2 ? IsAllDigits<H> extends true ? Len<N> extends 2 ? IsAllDigits<N> extends true ? Len<S> extends 2 ? IsAllDigits<S> extends true ? T : never : never : never : never : never : never : never;

export declare function isParsedTransaction(tx?: unknown | null): tx is ParsedTransactionData;

export declare function isPascalCase(value: string): value is PascalCase;

export declare type IsPlainObject<T> = T extends object ? T extends any[] ? false : T extends Function ? false : T extends Date | RegExp | Map<any, any> | Set<any> | WeakMap<any, any> | WeakSet<any> ? false : T extends Record<string, any> ? keyof T extends string ? true : false : false : false;

export declare function isPlainObject(obj: unknown): obj is Record<string, unknown>;

export declare type IsPrimitiveLike<T> = [T] extends [Primitive] ? true : T extends Date ? true : T extends Uint8Array ? true : false;

export declare function isPublicIp(ip: IpAddress): boolean;

export declare function isRawTransaction(tx?: unknown | null): tx is RawTransactionData;

declare type IsReadonly<T, K extends keyof T> = {
    readonly [P in K]: T[K];
} extends {
    [P in K]: T[K];
} ? false : true;

export declare function isSchemaObject(obj: unknown): obj is SchemaObject;

export declare function isScreamingSnakeCase(value: string): value is ScreamingSnakeCase;

export declare function isSnakeCase(value: string): value is SnakeCase;

export declare function isSyncFunction(fn: unknown): fn is SyncFunction;

export declare type IsTagged<T> = T extends Tagged<infer _U, any, any> ? true : false;

export declare function isTimestamp(value: unknown): value is Timestamp;

export declare function isTimestampMs(value: unknown): value is TimestampMs;

export declare function isTimestampNs(value: unknown): value is TimestampNs;

export declare function isTimestampS(value: unknown): value is TimestampS;

export declare function isTitleCase(value: string): value is TitleCase;

export declare function isTokenMovementInstruction(instruction: IsEncodable): instruction is TokenMovementInstruction;

export declare function isTrustedProxy(connectionIp: IpAddress, trusted: string[]): boolean;

/** Tuple vs array discriminator */
export declare type IsTuple<T extends readonly unknown[]> = number extends T["length"] ? false : true;

export declare type IsUnion<T, U = T> = T extends any ? ([U] extends [T] ? false : true) : false;

export declare function isUuid(str: string): str is Uuid;

export declare function isValidEmail(email: string): email is Email;

export declare function isValidIp(ip: string): ip is IpAddress;

export declare function isValidIpv4(ip: string): ip is IpAddress;

export declare function isValidIpv6(ip: string): ip is IpAddress;

export declare function isValidLogLevel(value: string): value is LogLevel;

/**
 * Minimum twelve characters; must include at least one uppercase, one lowercase,
 * one digit, and one special from: @$!%*?&
 * Only ASCII letters, digits, and the listed specials are allowed.
 */
export declare function isValidPassword(password: string): password is RawPassword<12>;

export declare function isValidPort(value: number): value is Port;

export declare function isValidServerUrl(value: string): value is Url;

/**
 * Basic IANA timezone validation (checks format, not exhaustive list)
 */
export declare function isValidTimeZone(value: string): boolean;

export declare function isValidUrl(value: string): value is Url;

export declare function isValidUsernameOrEmail(input: string): boolean;

export declare function isVersionedJsonTransaction(tx?: unknown | null): tx is VersionedJsonTransactionData;

export declare function isVersionedTransaction(tx?: unknown | null): tx is VersionedJsonTransactionData | (RawTransactionData & {
    version: number;
});

export declare function isWellKnownToken(address: string): boolean;

export declare function isWhitespaceCodeUnitFull(code: number): boolean;

export { it }

export declare type ItemName = Tagged<Text, "ItemName">;

export declare const JACKSON_HOLE_SYMPOSIUM: JacksonHoleSymposium;

export declare type JacksonHoleSymposium = EconomicEvent<"JacksonHoleSymposium">;

/**
 * Default header name for Jito API key
 */
export declare const JITO_DEFAULT_API_KEY_HEADER = "x-api-key";

/**
 * Jito block engine endpoints by region
 */
export declare const JITO_ENDPOINTS: Record<JitoRegion, Url>;

/**
 * Maximum number of transactions in a Jito bundle
 */
export declare const JITO_MAX_BUNDLE_SIZE = 5;

/**
 * Minimum tip amount for Jito bundles (in lamports)
 */
export declare const JITO_MINIMUM_TIP_LAMPORTS = 1000;

/**
 * Jito public bundle statuses base endpoint (public, non-regional)
 */
export declare const JITO_PUBLIC_BUNDLE_STATUSES_BASE_URL: Url;

/**
 * Jito-related HTTP route constants (server contexts).
 *
 * Keep these in @solomon-labs/constants so any server/app package can reference
 * the same canonical paths without duplicating strings.
 */
export declare const JITO_PUBLIC_ROUTES: {
    readonly sendBundle: "/solana/jito/bundles/send";
    readonly getBundleStatus: "/solana/jito/bundles/status";
};

/**
 * Jito block engine hosts by region
 */
export declare const JITO_REGION_HOSTS: {
    readonly mainnet: "https://mainnet.block-engine.jito.wtf";
    readonly testnet: "https://dallas.testnet.block-engine.jito.wtf";
    readonly amsterdam: "https://amsterdam.mainnet.block-engine.jito.wtf";
    readonly frankfurt: "https://frankfurt.mainnet.block-engine.jito.wtf";
    readonly ny: "https://ny.mainnet.block-engine.jito.wtf";
    readonly tokyo: "https://tokyo.mainnet.block-engine.jito.wtf";
};

/**
 * Supported Jito regions
 */
export declare const JITO_REGIONS: readonly JitoRegion[];

/**
 * Jito tip accounts (from getTipAccounts RPC - these are static)
 */
export declare const JITO_TIP_ACCOUNTS_MAINNET: readonly SolanaAddress[];

export declare const JITO_TIP_ACCOUNTS_TESTNET: readonly SolanaAddress[];

export declare class JitoBundle {
    private readonly transactions;
    private readonly config;
    constructor(config?: JitoBundleConfig);
    addTransaction(transaction: Transaction): JitoBundle;
    addTransactions(transactions: Transaction[]): JitoBundle;
    getTransactions(): readonly Transaction[];
    /**
     * Validate the bundle
     * 1. Checks if the bundle is empty
     * 2. Checks if the bundle has > JITO_MAX_BUNDLE_SIZE transactions
     * 3. Checks if any transaction is missing signatures
     * 4. Checks if the tip is missing in the last transaction
     * 5. Checks if any transaction is > PACKET_DATA_SIZE
     * 6. Checks if any transaction failed to serialize
     * 7. Checks if the tip amount is < JITO_MINIMUM_TIP_LAMPORTS
     * 8. Checks if the tip amount is missing in the last transaction
     * @returns The validation result.
     */
    validate(): JitoBundleValidationResult;
    estimateCost(): Promise<JitoBundleCostEstimate>;
    getTipInstruction(payer: PublicKeyLike): SystemTransferInstruction;
    hasTipInstruction(): boolean;
    serialize(encoding?: JitoBundleEncoding): string[];
    toJSON(encoding?: JitoBundleEncoding): JitoBundleJSON;
    static getTipAccounts(region?: JitoRegion): SolanaAddress[];
    static fromTransactions(transactions: Transaction[], config?: JitoBundleConfig): JitoBundle;
    private extractTipAmount;
    private formatValidationErrors;
}

/**
 * Jito bundle configuration options
 */
export declare interface JitoBundleConfig {
    /** Maximum number of transactions allowed in the bundle (default: 5) */
    maxTransactions?: number;
    /** Custom tip amount in lamports (default: 1000 minimum) */
    tipLamports?: number;
    /** Index of the Jito tip account to use (0-7) */
    tipAccountIndex?: number;
    /** Routing preference */
    region?: JitoRegion;
}

/**
 * Jito bundle cost estimate
 */
export declare interface JitoBundleCostEstimate {
    /** Individual transaction cost estimates */
    transactionCosts: TransactionCostEstimate[];
    /** Tip amount in lamports */
    tipAmount: number;
    /** Total cost in lamports (including all fees and tip) */
    totalCostLamports: number;
    /** Total cost in SOL */
    totalCostSol: number;
}

/**
 * Jito bundle transaction encoding
 */
export declare type JitoBundleEncoding = "base64";

/**
 * Jito bundle error codes
 */
export declare const JitoBundleErrorCode: {
    readonly EMPTY_BUNDLE: "EMPTY_BUNDLE";
    readonly TOO_MANY_TRANSACTIONS: "TOO_MANY_TRANSACTIONS";
    readonly UNSIGNED_TRANSACTION: "UNSIGNED_TRANSACTION";
    readonly TRANSACTION_TOO_LARGE: "TRANSACTION_TOO_LARGE";
    readonly SERIALIZATION_FAILED: "SERIALIZATION_FAILED";
    readonly MISSING_TIP: "MISSING_TIP";
    readonly MINIMUM_TIP_AMOUNT: "MINIMUM_TIP_AMOUNT";
};

export declare type JitoBundleErrorCodeType = (typeof JitoBundleErrorCode)[keyof typeof JitoBundleErrorCode];

/**
 * JSON representation of a Jito bundle
 */
export declare interface JitoBundleJSON {
    /** Base64 encoded transactions */
    transactions: string[];
    /** Encoding used for transactions (default: base64) */
    encoding?: JitoBundleEncoding;
}

/**
 * Jito bundle status entry
 */
export declare interface JitoBundleStatus {
    bundleId: string;
    slot: number;
    validator: string;
    tippers: string[];
    landedTipLamports: number;
    landedCu: number;
    blockIndex: number;
    timestamp: string;
    txSignatures: string[];
}

/**
 * Thrown when Jito bundle status lookup fails
 */
export declare class JitoBundleStatusError extends TransactionError {
    readonly statusCode?: number | undefined;
    readonly response?: Metadata | undefined;
    readonly type = "JITO_BUNDLE_STATUS";
    constructor(message?: string, statusCode?: number | undefined, response?: Metadata | undefined, originalError?: unknown);
}

export declare type JitoBundleStatusesApiResponse = JitoBundleStatus[] | JitoBundleStatusesErrorResponse;

/**
 * Jito bundle status API response
 */
export declare interface JitoBundleStatusesErrorResponse {
    error: string;
}

/**
 * Jito bundle statuses result
 */
export declare type JitoBundleStatusesResult = JitoBundleStatus[];

/**
 * Thrown when a Jito bundle submission fails
 */
export declare class JitoBundleSubmissionError extends TransactionError {
    readonly statusCode?: number | undefined;
    readonly response?: Metadata | undefined;
    readonly type = "JITO_BUNDLE_SUBMISSION";
    constructor(message?: string, statusCode?: number | undefined, response?: Metadata | undefined, originalError?: unknown);
}

/**
 * Jito bundle submission result
 */
export declare interface JitoBundleSubmissionResult {
    bundleId: string;
    raw?: Metadata;
}

/**
 * Jito bundle submission API response
 */
export declare interface JitoBundleSubmitApiResponse {
    jsonrpc?: string;
    id?: number | string;
    result?: string;
    error?: JitoBundleSubmitError;
}

/**
 * Jito bundle submission error response
 */
export declare interface JitoBundleSubmitError {
    code?: number;
    message?: string;
    data?: Metadata;
}

/**
 * Jito bundle validation error
 */
export declare interface JitoBundleValidationError {
    code: JitoBundleErrorCodeType;
    message: string;
    transactionIndex?: number;
}

/**
 * Jito bundle validation result
 */
export declare interface JitoBundleValidationResult {
    valid: boolean;
    errors: JitoBundleValidationError[];
    warnings: JitoBundleValidationWarning[];
}

/**
 * Jito bundle validation warning
 */
export declare interface JitoBundleValidationWarning {
    code: JitoBundleWarningCodeType;
    message: string;
    transactionIndex?: number;
}

/**
 * Jito bundle warning codes
 */
export declare const JitoBundleWarningCode: {};

export declare type JitoBundleWarningCodeType = (typeof JitoBundleWarningCode)[keyof typeof JitoBundleWarningCode];

export declare type JitoPublicRoute = (typeof JITO_PUBLIC_ROUTES)[keyof typeof JITO_PUBLIC_ROUTES];

/**
 * Jito region routing preference
 */
export declare type JitoRegion = "mainnet" | "testnet" | "amsterdam" | "frankfurt" | "ny" | "tokyo";

export declare class JitoRestClient extends BaseRestClient {
    private static bundleStatusesClient;
    private readonly apiKeyHeader?;
    private readonly apiKey?;
    private requestId;
    private readonly region;
    constructor(config?: JitoRestClientConfig);
    protected getAuthHeaders(): Promise<Headers | null>;
    private getNextRequestId;
    private static getBundleStatusesClient;
    submitBundle(bundle: JitoBundleJSON, options?: {
        skipValidation?: boolean;
    }): Promise<JitoBundleSubmissionResult>;
    private validateBundle;
    private formatValidationErrors;
    private parseSubmissionResponse;
    getBundleStatuses(bundleId: string): Promise<JitoBundleStatusesResult>;
    private makeBundleStatusesRequest;
    private parseBundleStatusesResponse;
}

/**
 * Jito REST client configuration
 */
export declare interface JitoRestClientConfig extends RestClientConfig {
    region?: JitoRegion;
    apiKeyHeader?: string;
}

export declare const JPX: JPX_2;

declare type JPX_2 = ExchangeName<"JPX">;

export declare type JPXId = ExchangeId<JPX_2>;

export declare const JSONRPC_INTERNAL_ERROR_CODE: -32603;

export declare const JSONRPC_INVALID_PARAMS_CODE: -32602;

export declare const JSONRPC_INVALID_REQUEST_CODE: -32600;

export declare const JSONRPC_METHOD_NOT_FOUND_CODE: -32601;

export declare const JSONRPC_PARSE_ERROR_CODE: -32700;

export declare const JSONRPC_RATE_LIMIT_ERROR_CODE: -32005;

export declare const JSONRPC_SERVER_ERROR_END_CODE: -32099;

export declare const JSONRPC_SERVER_ERROR_START_CODE: -32000;

export declare interface JsonRpcNotification<T = unknown> {
    jsonrpc: "2.0";
    method: string;
    params: {
        subscription: number;
        result: T;
    };
}

export declare interface JsonRpcRequest {
    jsonrpc: "2.0";
    id: number;
    method: string;
    params?: unknown[];
}

export declare interface JsonRpcResponse<T = unknown> {
    jsonrpc: "2.0";
    id?: number;
    result?: T;
    error?: {
        code: number;
        message: string;
        data?: unknown;
    };
}

export declare type JsonSerializable = string | number | boolean | null | JsonSerializable[] | {
    [key: string]: JsonSerializable;
} | {
    toString(): string;
} | {
    toJSON(): JsonSerializable;
};

declare interface JsonSerializable_2 {
    toJSON(): SerializableValue_2;
    [key: string]: any;
}

export declare const JUNETEENTH: Juneteenth;

export declare type Juneteenth = Holiday<"Juneteenth">;

export declare const JUPITER: Jupiter;

export declare type Jupiter = SolanaProtocolName<"Jupiter">;

export declare const JUPITER_URL: Url<"https://jup.ag/">;

export declare type JupiterId = SolanaProtocolId<Jupiter>;

export declare type JwtSecret<S extends string = string> = Tagged<ApiSecret<S>, "JwtSecret">;

export declare const KAMINO: Kamino;

export declare type Kamino = SolanaProtocolName<"Kamino">;

export declare const KAMINO_URL: Url<"https://app.kamino.finance/">;

export declare type KaminoId = SolanaProtocolId<Kamino>;

export declare type KebabCase<T extends string = string> = Tagged<ValidKebabCasePattern<T>, "KebabCase">;

export declare type KeysWithBrand<Container, Brand> = {
    [PropertyKey in keyof Container]-?: Container[PropertyKey] extends Brand ? PropertyKey : never;
}[keyof Container] & string;

export declare type KeysWithBrandNullish<Container, B> = Extract<{
    [PropertyKey in keyof Container]-?: Exclude<Container[PropertyKey], null | undefined> extends B ? PropertyKey : never;
}[keyof Container], string>;

export declare type KeysWithRelation<Container> = {
    [PropertyKey in keyof Container]-?: Container[PropertyKey] extends Relation<any, any> ? PropertyKey : never;
}[keyof Container] & string;

/** A span kind i.e. LOG, INSTR, SWAP, ADD, etc */
export declare type KindSpan = ["kind"];

export declare const KINGS_BIRTHDAY: KingsBirthday;

export declare type KingsBirthday = Holiday<"Kings Birthday">;

export declare const KOSPI: KOSPIName;

export declare type KOSPIName = EquityIndexName<"KOSPI">;

export declare type KOSPISymbol = EquityIndexSymbol<"KOSPI">;

export declare const KRAKEN: Kraken;

export declare type Kraken = ExchangeName<"Kraken">;

export declare type KrakenId = ExchangeId<Kraken>;

export declare const KUCOIN: KuCoin;

export declare type KuCoin = ExchangeName<"KuCoin">;

export declare type KuCoinId = ExchangeId<KuCoin>;

/** Common role for key-value style spans */
export declare type LabelValueRole = "label" | "value";

export declare const LABOR_DAY: LaborDay;

export declare type LaborDay = Holiday<"Labor Day">;

export declare const LABOUR_DAY: LabourDay;

export declare type LabourDay = Holiday<"Labour Day">;

export declare type Latitude = Tagged<string, "Latitude">;

export declare interface LegacyJsonTransactionData extends RpcTransactionData {
    transaction: {
        message: LegacyTransactionMessage;
        signatures: SolanaTransactionSignature[];
    };
    version: "legacy";
}

export declare interface LegacyPublicKey {
    toBase58(): SolanaAddress;
    toString(): SolanaAddress;
    toBytes(): Uint8Array;
    toBuffer(): Buffer;
    findProgramAddressSync(...args: unknown[]): unknown;
    constructor: LegacyPublicKeyConstructor;
}

export declare type LegacyPublicKeyConstructor = new (value: LegacyPublicKeyInitData) => LegacyPublicKey;

declare type LegacyPublicKeyData = {
    /** @internal */
    _bn: unknown;
};

declare type LegacyPublicKeyInitData = number | SolanaAddress | SolanaKitAddress | Uint8Array | number[] | LegacyPublicKeyData;

export declare interface LegacyTransactionInstruction {
    keys: InstructionAccount[];
    programId: PublicKeyLike;
    data?: Uint8Array | Base58 | null | undefined;
    new (...args: any[]): LegacyTransactionInstruction;
    toJSON(): any;
}

export declare interface LegacyTransactionMessage {
    accountKeys: SolanaAddress[];
    header: TransactionMessageHeader;
    instructions: TransactionInstruction[];
    recentBlockhash: SolanaBlockhash;
}

declare type Len<S extends string> = Chars<S>["length"];

export declare type LinearTradingPair<TBase extends string = string, TQuote extends string = string> = Tagged<PerpetualTradingPair<TBase, TQuote> | DeliveryTradingPair<TBase, TQuote>, "LinearTradingPair">;

export declare const LINUX: OperatingSystem<"Linux">;

/** Literal Base58, length between Min–Max. */
export declare type LiteralBase58<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? never : _Base58OK<S, Min, Max> extends true ? S : never;

/** Literal Base64, length between Min–Max. */
export declare type LiteralBase64<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? never : _Base64OK<S, Min, Max> extends true ? S : never;

/** Literal Base64Url, length between Min–Max. */
export declare type LiteralBase64Url<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? never : _Base64OK<S, Min, Max> extends true ? S : never;

/** Literal lower-case hex, even length, between Min–Max. */
export declare type LiteralHexLower<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? never : _HexLowerOK<S, Min, Max> extends true ? S : never;

export declare const LOCALHOST: HostName<"localhost">;

export declare const LOCALSTORAGE_AVAILABLE: boolean;

export declare const LOCKEDUSDV_ADDRESS_STRINGS: SolanaAddress<"5zHAA3Gk8tG3Bze2o6S2bboFBCTYoJmDvk8oyjcFVAbz">;

export declare interface LogEvent extends CanonicalOperation {
    type: "log";
    parsed: ParsedLog;
    decoded: IsLogDecodable | null;
}

export declare class LogParser {
    private static readonly registered;
    private static readonly prefixMap;
    private static prefixEntries;
    private static readonly programIdCache;
    private static readonly invokeRegex;
    private static readonly successRegex;
    private static readonly failedRegex;
    private static readonly consumedRegex;
    private static readonly returnRegex;
    private static readonly dataRegex;
    private static readonly programLogRegex;
    private static readonly truncatedRegex;
    private static readonly instructionRegex;
    private static readonly anchorErrorRegex;
    private static readonly EVENT_CPI_DISCRIMINATOR;
    private static readonly STACK_KEY_MULTIPLIER;
    private static discriminatorKeyFromBytes;
    private static getProgramId;
    private static discriminatorKeyFromDiscriminator;
    static tryDecodeRegistered(data: Uint8Array): IsLogDecodable | null;
    private static tryDecodePrefixedLog;
    /**
     * Register log decoders
     */
    static register(...logTypes: IsLogDecodableStatic[]): void;
    /**
     * Clear all registered log types
     */
    static clearAll(): void;
    /**
     * Get all registered log types
     */
    static getRegistered(): Map<string, IsLogDecodableStatic>;
    /**
     * First pass: Parse raw log messages into structured ParsedLog objects
     *
     * Additionally, when given a Transaction and instructions, we synthesize
     * "Anchor self-CPI" event logs inline, as we see self-CPI invokes, so
     * downstream code sees the event in canonical order without a second pass.
     */
    static parse(logMessagesOrTransaction: Transaction | string[], parsedInstructions?: ParsedInstruction[] | null, includeAnchorSelfCpiSyntheticLogs?: boolean): ParsedLog[];
    /**
     * Second pass: Decode data logs into concrete types
     */
    static decode(parsedLog: ParsedLog): IsLogDecodable | null;
    /**
     * Detect Anchor self-CPI inner instructions whose `data` payloads look like
     * registered event types (by discriminator), and synthesize log entries
     * using that instruction data. These are attached to the parent instruction
     * frame so the event appears where the user code emitted it.
     */
    private static extractAnchorSelfCpiLog;
}

/**
 * Thrown when transaction log parsing fails
 */
export declare class LogParsingError extends TransactionError {
    readonly logIndex?: number | undefined;
    readonly reason?: string | undefined;
    readonly type = "LOG_PARSING";
    constructor(message?: string, logIndex?: number | undefined, reason?: string | undefined, originalError?: unknown);
}

export declare interface LogsNotification {
    context: {
        slot: SolanaSlot;
    };
    value: {
        signature: SolanaTransactionSignature;
        err: TransactionMeta["err"];
        logs: string[];
    };
}

export declare interface LogsSubscribeConfig {
    commitment?: SolanaCommitmentLevel;
}

export declare type LogsSubscribeFilter = "all" | "allWithVotes" | {
    mentions: [SolanaAddress];
};

export declare const LONDON_TIMEZONE: LondonTimezone;

export declare type LondonTimezone = IANATimeZone<"Europe/London">;

export declare type Longitude = Tagged<string, "Longitude">;

export declare const LOOPSCALE: Loopscale;

export declare type Loopscale = SolanaProtocolName<"Loopscale">;

export declare const LOOPSCALE_URL: Url<"https://loopscale.com/">;

export declare type LoopscaleId = SolanaProtocolId<Loopscale>;

declare type LowercaseAlphaNumeric = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

export declare const LSE: LSE_2;

declare type LSE_2 = ExchangeName<"LSE">;

export declare type LSEId = ExchangeId<LSE_2>;

declare type Lte<L extends number, Max extends number> = Build<Max> extends [...Build<L>, ...any[]] ? true : false;

export declare const LULO: Lulo;

export declare type Lulo = SolanaProtocolName<"Lulo">;

export declare const LULO_URL: Url<"https://lulo.fi/">;

export declare type LuloId = SolanaProtocolId<Lulo>;

export declare const LUNCH_BREAK_SESSION: LunchBreakSession;

export declare type LunchBreakSession = MarketSession<"Lunch Break">;

export declare const MACOS: OperatingSystem<"macOS">;

export declare const MAINNET_BETA_ENDPOINT: UrlInfo<{
    url: "https://api.mainnet-beta.solana.com";
    description: "Solana Mainnet RPC Endpoint";
}>;

export declare type MakeOptional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

declare type MakeOptionalMap<T, IsOptional extends boolean> = IsOptional extends true ? T | null : T;

export declare type MakeOptionalOrNull<T, K extends keyof T> = Omit<T, K> & {
    [P in K]?: T[P] | null;
};

export declare type MakeRequired<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

export declare const MALFORMED_MESSAGE: "malformed";

/**
 * Thrown when transaction instruction is malformed
 */
export declare class MalformedInstructionError extends TransactionError {
    readonly instructionIndex?: number | undefined;
    readonly reason?: string | undefined;
    readonly type = "MALFORMED_INSTRUCTION";
    constructor(message?: string, instructionIndex?: number | undefined, reason?: string | undefined, originalError?: unknown);
}

export declare interface MapCacheResult {
    id: PrimaryId;
    uuid: Uuid;
}

export declare interface MapCacheWithAddressResult {
    id: PrimaryId;
    uuid: Uuid;
    address: SolanaAddress;
}

export declare interface MapCacheWithSlugAndSymbolResult {
    id: PrimaryId;
    uuid: Uuid;
    slug: Slug;
    symbol: AssetSymbol;
}

export declare interface MapCacheWithSlugAndTradingPairResult {
    id: PrimaryId;
    uuid: Uuid;
    slug: Slug;
    symbol: TradingPair;
}

export declare interface MapCacheWithSlugResult {
    id: PrimaryId;
    uuid: Uuid;
    slug: Slug;
}

export declare function mapFields<T extends Metadata>(data: unknown, fieldMappings: Record<keyof T, FieldMapping>): Partial<T>;

export declare const MARGINFI: Marginfi;

export declare type Marginfi = SolanaProtocolName<"marginfi">;

export declare const MARGINFI_URL: Url<"https://www.marginfi.com/">;

export declare type MarginfiId = SolanaProtocolId<Marginfi>;

export declare const MARINADE: Marinade;

export declare type Marinade = SolanaProtocolName<"Marinade">;

export declare const MARINADE_URL: Url<"https://marinade.finance/">;

export declare type MarinadeId = SolanaProtocolId<Marinade>;

/** A zero-width marker/anchor with a tag. */
export declare type MarkerOp = 3;

export declare type MarketId<T extends string = string> = Tagged<T, "MarketId">;

export declare type MarketName<T extends string = string> = Tagged<T, "MarketName">;

export declare type MarketSession<SessionName extends string = string> = Tagged<Session, SessionName>;

export declare interface MarketSessionEntry extends BaseCalendarEntry {
    type: "Session";
    open: Date;
    close: Date;
    isPaused?: boolean;
    pauseReason?: string;
}

export declare interface MarketStatus {
    isOpen: boolean;
    currentSession?: MarketSessionEntry;
    nextOpen?: Date;
    nextClose?: Date;
    timeToClose?: number;
    timeToOpen?: number;
    sessions: MarketSessionEntry[];
}

export declare const MARTIN_LUTHER_KING_JR_DAY: MartinLutherKingJrDay;

export declare type MartinLutherKingJrDay = Holiday<"Martin Luther King Jr Day">;

/**
 * Check if data matches a discriminator
 */
export declare function matchesDiscriminator(data: Uint8Array, discriminator: Uint8Array): boolean;

export declare const MAXIMUM_ORM_NAME_LENGTH = 63;

export declare function maxLengthValidator<T extends {
    length: number;
}>(max: number): (value: T) => boolean;

export declare type Md5Hash = Tagged<Hash, "Md5Hash">;

export declare const MEMO_PROGRAM_ID_STRING: MemoProgram;

export declare type MemoProgram = SolanaAddressInfo<"MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr", {
    systemProgram: true;
    name: "Memo Program";
    description: "Verifies any accounts provided are signers of the transaction";
}>;

export declare const MEMORIAL_DAY: MemorialDay;

export declare type MemorialDay = Holiday<"Memorial Day">;

export declare class Message {
    readonly shared: MessageShared;
    readonly versioned: MessageVersioned | null;
    private readonly internals;
    constructor(initArgs?: MessageConstructor, options?: "constructed" | "rpc" | MessageDeserializeArgs);
    determineVersion(): ValidSolanaTransactionVersion;
    get version(): ValidSolanaTransactionVersion;
    getVersion(): ValidSolanaTransactionVersion;
    isVersioned(): boolean;
    isReadonly(): boolean;
    setIsReadonly(isReadonly: boolean): this;
    getFeePayer(): PublicKeyLike | null;
    attach(transaction: Transaction): this;
    assertMutable(methodName: string): void;
    private ensureAddressesOrLookups;
    ensureComputedLookups(): void;
    private getActiveLookupTables;
    private generateLookupTables;
    private generateLookupTableAddress;
    private computeLookupTables;
    aggregateAccountMetas(): AggregatedInstructionAccount[];
    get recentBlockhash(): SolanaBlockhash | null;
    get header(): TransactionMessageHeader;
    getRecentBlockhash(): SolanaBlockhash | null;
    getHeader(): TransactionMessageHeader;
    getInstructions(): MessageInstruction[];
    get instructions(): MessageInstruction[];
    hasRecentBlockhash(): boolean;
    setRecentBlockhash(blockhash: SolanaBlockhash): this;
    getStaticAccountKeys(): PublicKeyLike[];
    get staticAccountKeys(): PublicKeyLike[];
    get accountKeys(): PublicKeyLike[];
    serialize(): Uint8Array;
    applyAggregatesToStaticKeysAndHeader(): this;
    private isAddressTableLookupsNonEmpty;
    private assertValidProgramSlot;
    /** Convert mixed instructions to compiled w/ proper account indices for this message. */
    toCompiled(): CompiledInstruction[];
    private serializeLegacy;
    private serializeV0;
    get numAccountKeysFromLookups(): number;
    get addressTableLookups(): AddressTableLookup[];
    getAddressTableLookups(): AddressTableLookup[];
    addInstruction(...instructions: MessageInstruction[]): this;
    addInstructions(...instructions: MessageInstruction[]): this;
    add(...instructions: MessageInstruction[]): this;
    private ensureKeysForInstruction;
    getInternalType(): "constructed" | "rpc" | "wire";
    markDirty(): void;
    getAddressLookupTableAccounts(): AddressLookupTableAccount[];
    getLoadedAddresses(): ResolvedAddressTableLookups | null;
    enableLookupTables(enabled?: boolean): this;
    disableLookupTables(): this;
    addLookupTableAccounts(...accounts: AddressLookupTableAccount[]): this;
    setLookupTableMode(mode: AddressTableLookupMode): this;
    setLookupTableAccounts(accounts: AddressLookupTableAccount[]): this;
    flushLookupTables(): this;
    getAccountKeys(addressLookupTableAccounts?: AddressLookupTableAccount[]): PublicKeyLike[];
    private resolveAddressTableLookups;
    isAccountSigner(index: number): boolean;
    isAccountWritable(index: number, addressLookupTableAccounts?: AddressLookupTableAccount[]): boolean;
    isProgramId(index: number): boolean;
    programIds(): PublicKeyLike[];
    getUniqueProgramIds(): PublicKeyLike[];
    getUniqueInstructionAccounts(): PublicKeyLike[];
    nonProgramIds(): PublicKeyLike[];
    static deserializeMessageVersion(serializedMessage: Uint8Array): ValidSolanaTransactionVersion;
    static deserialize(serializedMessage: Uint8Array, args?: MessageDeserializeArgs): Message;
    static from(buffer: Buffer | Uint8Array | number[], args?: MessageDeserializeArgs): Message;
    static deserializeLegacy(buffer: Uint8Array, args?: MessageDeserializeArgs): Message;
    static deserializeV0(buffer: Uint8Array, args?: MessageDeserializeArgs): Message;
    clone(): Message;
    toJSON(): MessageJson;
}

/**
 * Thrown when message is already attached to a Transaction
 */
export declare class MessageAlreadyAttachedError extends MessageError {
    readonly type = "ALREADY_ATTACHED";
    constructor(message?: string, originalError?: unknown);
}

export declare type MessageConstructor = MessageWireArgs | MessageRpcArgs | CompatibleMessageV0Args | CompatibleMessageArgs | Buffer | Uint8Array | number[] | LegacyTransactionMessage | VersionedTransactionMessage | MessageConstructorArgs;

export declare interface MessageConstructorArgs {
    version?: ValidSolanaTransactionVersion;
    header: TransactionMessageHeader;
    accountKeys?: AddressTableLookup[] | SolanaAddress[] | PublicKeyLike[];
    staticAccountKeys?: AddressTableLookup[] | SolanaAddress[] | PublicKeyLike[];
    recentBlockhash?: SolanaBlockhash;
    instructions: MessageInstruction[];
    addressLookupTableAccounts?: AddressLookupTableAccount[];
    loadedAddresses?: ResolvedAddressTableLookups | null;
    addressTableLookups?: AddressTableLookup[];
}

export declare interface MessageDeserializeArgs {
    addressLookupTableAccounts?: AddressLookupTableAccount[];
    loadedAddresses?: ResolvedAddressTableLookups | null;
}

/**
 * Base class for Message-related errors
 */
export declare abstract class MessageError extends SolomonLabsError {
    readonly category = "MESSAGE";
}

export declare type MessageInstruction = CompiledInstruction | LegacyTransactionInstruction | CompatibleMessageCompiledInstruction | EncodedInstruction | IsEncodable;

export declare interface MessageInternals {
    addressLookupTableAccounts: AddressLookupTableAccount[];
    loadedAddresses: ResolvedAddressTableLookups | null;
    externalLookupTables: AddressLookupTableAccount[];
    lookupTableMode: AddressTableLookupMode;
    generatedLookupTables: AddressLookupTableAccount[];
    enableLookupTables: boolean;
    computedLookups: AddressTableLookup[];
    version: ValidSolanaTransactionVersion | null;
    isDirty: boolean;
    type: "constructed" | "wire" | "rpc";
    isReadonly: boolean;
    versionOverride?: ValidSolanaTransactionVersion | null;
    transaction: null | Transaction;
    cachedAccountKeys: PublicKeyLike[] | undefined;
    cachedCompiled: CompiledInstruction[] | undefined;
    cachedAggregatedMetas: AggregatedInstructionAccount[] | undefined;
}

export declare type MessageJson = MessageJsonLegacy | MessageJsonV0;

export declare interface MessageJsonBase {
    version: ValidSolanaTransactionVersion;
    header: TransactionMessageHeader;
    accountKeys: SolanaAddress[];
    recentBlockhash: SolanaBlockhash | null;
    instructions: MessageJsonInstruction[];
}

export declare interface MessageJsonInstruction {
    programIdIndex: number;
    accounts: number[];
    data: Base58 | null;
    stackHeight: number | null;
}

export declare interface MessageJsonLegacy extends MessageJsonBase {
    version: "legacy";
}

export declare interface MessageJsonV0 extends MessageJsonBase {
    version: 0;
    addressTableLookups: AddressTableLookup[];
}

export declare type MessageRpcArgs = Omit<MessageShared, "recentBlockhash"> & {
    recentBlockhash: SolanaBlockhash;
} & MessageVersioned;

export declare interface MessageShared {
    recentBlockhash?: SolanaBlockhash | null;
    accountKeys: SolanaAddress[];
    header: TransactionMessageHeader;
    instructions: MessageInstruction[];
}

export declare interface MessageVersioned {
    addressTableLookups?: AddressTableLookup[];
}

export declare interface MessageWireArgs {
    bytes: Uint8Array | number[] | Buffer;
    args?: MessageDeserializeArgs;
}

export declare type Metadata<T = Record<PropertyKey, any>> = T extends Record<PropertyKey, any> ? T : Record<PropertyKey, any>;

export declare const METAPLEX_METADATA_PROGRAM_ID = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

/** Meta associated with a group */
export declare type MetaSpan = ["meta"];

export declare const METEORA: Meteora;

export declare type Meteora = ExchangeName<"Meteora"> & SolanaProtocolName<"Meteora">;

export declare const METEORA_DLMM_USDV_USDC_POOL_ADDRESS_STRING: SolanaAddress<"DmXXwEcK2c7fuVoW6TBzF5UDByhuQBHZS1qHnwprvHFH">;

export declare const METEORA_URL: Url<"https://www.meteora.ag/">;

export declare type MeteoraId = ExchangeId<Meteora> & SolanaProtocolId<Meteora>;

export declare type MetricDistanceUnits = "Millimetre" | "Centimetre" | "Metre" | "Kilometre";

export declare type MetricWeightUnits = "Nanogram" | "Milligram" | "Gram" | "Kilogram";

export declare const MEXC: MEXC_2;

declare type MEXC_2 = ExchangeName<"MEXC">;

export declare type MEXCId = ExchangeId<MEXC_2>;

export declare const MIME_APPLICATION_FORM_URLENCODED: "application/x-www-form-urlencoded";

export declare const MIME_APPLICATION_GZIP: "application/gzip";

export declare const MIME_APPLICATION_JAVASCRIPT: "application/javascript";

export declare const MIME_APPLICATION_JSON: "application/json";

export declare const MIME_APPLICATION_OCTET_STREAM: "application/octet-stream";

export declare const MIME_APPLICATION_PDF: "application/pdf";

export declare const MIME_APPLICATION_XML: "application/xml";

export declare const MIME_APPLICATION_ZIP: "application/zip";

export declare const MIME_AUDIO_MP3: "audio/mpeg";

export declare const MIME_AUDIO_OGG: "audio/ogg";

export declare const MIME_AUDIO_WAV: "audio/wav";

export declare const MIME_IMAGE_GIF: "image/gif";

export declare const MIME_IMAGE_ICO: "image/x-icon";

export declare const MIME_IMAGE_JPEG: "image/jpeg";

export declare const MIME_IMAGE_PNG: "image/png";

export declare const MIME_IMAGE_SVG: "image/svg+xml";

export declare const MIME_IMAGE_WEBP: "image/webp";

export declare const MIME_MULTIPART_FORM_DATA: "multipart/form-data";

export declare const MIME_MULTIPART_MIXED: "multipart/mixed";

export declare const MIME_TEXT_CSS: "text/css";

export declare const MIME_TEXT_CSV: "text/csv";

export declare const MIME_TEXT_HTML: "text/html";

export declare const MIME_TEXT_JAVASCRIPT: "text/javascript";

export declare const MIME_TEXT_PLAIN: "text/plain";

export declare const MIME_TEXT_XML: "text/xml";

export declare const MIME_VIDEO_MP4: "video/mp4";

export declare const MIME_VIDEO_OGG: "video/ogg";

export declare const MIME_VIDEO_WEBM: "video/webm";

export declare type MimeType = ApplicationMimeType | TextMimeType | ImageMimeType | AudioMimeType | VideoMimeType | MultipartMimeType;

export declare function minLengthValidator<T extends {
    length: number;
}>(min: number): (value: T) => boolean;

export declare const MINT_ACCOUNT_LEN = 82;

export declare type MintAddress = Tagged<SolanaAddress, "MintAddress">;

export declare interface MintHolder {
    tokenAccount: SolanaAddress;
    /** Raw base-unit balance (exact u64). */
    amount: bigint;
}

/** Mint Address */
export declare type MintSpan = ["mint", LabelValueRole];

/** 14 = MintToChecked { amount: u64, decimals: u8 } */
export declare class MintToCheckedInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    mint: SolanaAddressLike;
    destination: SolanaAddressLike;
    authority: SolanaAddressLike;
    amount: bigint | number | string;
    decimals: number;
    constructor(props: EncodableProps<MintToCheckedInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): MintToCheckedInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/** 7 = MintTo { amount: u64 } */
export declare class MintToInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    mint: SolanaAddressLike;
    destination: SolanaAddressLike;
    authority: SolanaAddressLike;
    amount: bigint | number | string;
    constructor(props: EncodableProps<MintToInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): MintToInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const MISSING_PARAMETER_MESSAGE: "missing parameter";

/**
 * Error thrown when an expected account in the schema is missing from the instruction.
 */
export declare class MissingAccountEncoderError extends EncoderError {
    readonly type = "MISSING_ACCOUNT";
    constructor(message?: string, originalError?: unknown);
}

export { mock }

export declare type MockCall = {
    arguments: unknown[];
};

export declare class MockDataGenerator {
    static tokenAccountData(overrides?: {
        amount?: number;
    }): Uint8Array;
    static mintAccountData(overrides?: {
        decimals?: number;
        supply?: number | bigint;
        mintAuthority?: Uint8Array;
        freezeAuthority?: Uint8Array;
    }): Uint8Array;
    static whirlpoolData(overrides?: Partial<{
        liquidity: bigint;
        sqrtPrice: bigint;
        tickCurrentIndex: number;
        feeRate: number;
        protocolFeeRate: number;
        tickSpacing: number;
    }>): Uint8Array;
    static positionData(overrides?: Partial<{
        liquidity: bigint;
        tickLowerIndex: number;
        tickUpperIndex: number;
        feeOwedA: bigint;
        feeOwedB: bigint;
    }>): Uint8Array;
    static dlmmPairData(overrides?: Partial<{
        activeId: number;
        binStep: number;
        tokenXMint?: PublicKey_2;
        tokenYMint?: PublicKey_2;
    }>): Uint8Array;
    static createMockLBPair(overrides?: Partial<{
        address?: PublicKey_2;
        tokenXMint?: PublicKey_2;
        tokenYMint?: PublicKey_2;
        activeId?: number;
        binStep?: number;
    }>): ILBPair;
    static farmStateData(overrides?: Partial<{
        perSlotRewardA: bigint;
        perSlotRewardB: bigint;
        totalRewardA: bigint;
        totalRewardB: bigint;
    }>): Uint8Array;
    static cpmmPoolData(overrides?: Partial<{
        lpSupply: bigint;
        openTime: bigint;
    }>): Uint8Array;
    static binArrayData(overrides?: Partial<{
        index: bigint;
        version: number;
    }>): Uint8Array;
    static dlmmPositionData(overrides?: Partial<{
        lastUpdatedAt: bigint;
    }>): Uint8Array;
    static protocolFeeData(overrides?: Partial<{
        amountX: bigint;
        amountY: bigint;
    }>): Uint8Array;
    static dlmmRewardInfoData(overrides?: Partial<{
        rewardDuration: bigint;
        rewardRate: bigint;
    }>): Uint8Array;
    static binData(overrides?: Partial<{
        amountX: bigint;
        amountY: bigint;
        price: bigint;
        liquiditySupply: bigint;
    }>): Uint8Array;
    static liquidityParameterData(overrides?: Partial<{
        strategyType: number;
        amountX: bigint;
        amountY: bigint;
        activeId: number;
    }>): Uint8Array;
    static feeInfoData(overrides?: Partial<{
        feeXPerTokenComplete: bigint;
        feeYPerTokenComplete: bigint;
        feeXPending: bigint;
        feeYPending: bigint;
    }>): Uint8Array;
    static userRewardInfoData(overrides?: Partial<{
        rewardPerTokenComplete: bigint;
        rewardPending: bigint;
    }>): Uint8Array;
    static binArrayBitmapExtensionData(overrides?: Partial<{
        value: bigint;
    }>): Uint8Array;
    static dynamicPoolData(overrides?: Partial<{
        tokenAAmount: bigint;
        tokenBAmount: bigint;
        virtualPrice: bigint;
        tradeFeeNumerator: bigint;
        tradeFeeDenominator: bigint;
    }>): Uint8Array;
    static parameterLBPairData(overrides?: Partial<{
        baseFactor: number;
        filterPeriod: number;
        decayPeriod: number;
        reductionFactor: number;
        variableFeeControl: number;
        maxVolatilityAccumulator: number;
        minBinId: number;
        maxBinId: number;
        protocolShare: number;
    }>): Uint8Array;
    static variableParametersData(overrides?: Partial<{
        volatilityAccumulator: number;
        volatilityReference: number;
        indexReference: number;
        lastUpdateTimestamp: bigint;
    }>): Uint8Array;
    static raydiumAMMData(overrides?: Partial<{
        status: bigint;
        tradeFeeNumerator: bigint;
        tradeFeeDenominator: bigint;
        swapFeeNumerator: bigint;
        swapFeeDenominator: bigint;
    }>): Uint8Array;
    static raydiumCLMMPoolData(overrides?: Partial<{
        liquidity: bigint;
        sqrtPriceX64: bigint;
        tickCurrent: number;
        tickSpacing: number;
    }>): Uint8Array;
    static raydiumCLMMPositionData(overrides?: Partial<{
        liquidity: bigint;
        tickLowerIndex: number;
        tickUpperIndex: number;
        tokenFeesOwed0: bigint;
        tokenFeesOwed1: bigint;
    }>): Uint8Array;
    static raydiumCLMMPersonalPositionData(overrides?: Partial<{
        liquidity: bigint;
        tickLowerIndex: number;
        tickUpperIndex: number;
    }>): Uint8Array;
    static raydiumCLMMRewardInfoData(overrides?: Partial<{
        rewardGrowthInsideLastX64: bigint;
        amountOwed: bigint;
    }>): Uint8Array;
    static raydiumCPMMPoolData(overrides?: Partial<{
        lpSupply: bigint;
        openTime: bigint;
        status: number;
    }>): Uint8Array;
    static raydiumFarmStateData(overrides?: Partial<{
        state: bigint;
        totalRewardA: bigint;
        totalRewardB: bigint;
        perSlotRewardA: bigint;
        perSlotRewardB: bigint;
    }>): Uint8Array;
    static raydiumUserInfoData(overrides?: Partial<{
        state: bigint;
        lpTokenAmount: bigint;
        rewardDebtA: bigint;
        rewardDebtB: bigint;
    }>): Uint8Array;
    /**
     * Helper method to write U128 values in little endian format
     */
    private static writeBigUint128;
}

export declare type Mocked<T> = {
    [K in keyof T]: T[K] extends (...args: infer A) => infer R ? Mock<(...args: A) => R> : T[K] extends object ? Mocked<T[K]> : T[K];
};

export declare type MockedConstructor<T> = Mock<new (...args: any[]) => T>;

export declare class MockHelpers {
    /**
     * Creates a mock function that returns different values on subsequent calls
     */
    static createSequentialMock<T>(values: T[]): Mock<() => T>;
    /**
     * Creates a mock function that returns different promises on subsequent calls
     */
    static createSequentialAsyncMock<T>(values: T[]): Mock<() => Promise<T>>;
    static createSequentialAsyncFunctionMock<T>(values: T[]): Mock<() => Promise<T>>;
    /**
     * Creates a mock function that throws an error
     */
    static createErrorMock(error: Error): Mock<() => never>;
    /**
     * Creates a mock function that rejects with an error
     */
    static createAsyncErrorMock(error: Error): Mock<() => Promise<never>>;
    /**
     * Creates a mock function that returns a value based on call count and arguments
     */
    static createConditionalMock<T>(condition: (callCount: number, args: any[]) => T): Mock<(...args: any[]) => T>;
    /**
     * Creates a mock function that returns async values based on call count and arguments
     */
    static createConditionalAsyncMock<T>(condition: (callCount: number, args: any[]) => T): Mock<(...args: any[]) => Promise<T>>;
    /**
     * Creates a mock function that always returns the same value
     * Note: Use createConstantFunctionMock if you need to pass a function
     */
    static createConstantMock<T extends Exclude<any, (...args: any[]) => any>>(value: T): Mock<(...args: any[]) => T>;
    /**
     * Creates a mock function that always resolves to the same value
     * Note: Use createConstantAsyncFunctionMock if you need to pass a function
     */
    static createConstantAsyncMock<T extends Exclude<any, (...args: any[]) => any>>(value: T): Mock<() => Promise<T>>;
    /**
     * Creates a mock constructor that returns the same instance
     */
    static createConstructorMock<T>(instance: T): Mock<new (..._args: any[]) => T>;
    /**
     * Creates a mock class that can be used as a replacement for the original class
     */
    static createClassMock<T extends object>(mockInstance: Partial<T>): new (...args: any[]) => T;
    /**
     * Creates a mock class that wraps the original class
     */
    static createClassMockExtending<BaseCtor extends new (...args: any[]) => any>(Base: BaseCtor, mockInstance: Partial<InstanceType<BaseCtor>>): BaseCtor;
    /**
     * Creates a mock class that wraps some interface definition stub.
     * i.e. the concrete implementation is not present in the test.
     */
    static createMockConstructorReturning<T extends object>(instance: T): Mock<new (...args: any[]) => T>;
    /**
     * Creates a non-async mock function that always calls a function if its a function, else resolves to the same value
     */
    static createConstantFunctionMock<T>(value: T): Mock<(...args: any[]) => T>;
    /**
     * Creates a mock function that always calls a function if its a function, else resolves to the same value
     */
    static createConstantAsyncFunctionMock<T>(value: T): Mock<(...args: any[]) => Promise<T>>;
    /**
     * Creates a mock function that alternates between success and error
     */
    static createAlternatingMock<T>(successValue: T, error: Error): Mock<() => T>;
    /**
     * Creates a mock function that fails on specific call numbers
     */
    static createFailOnCallMock<T>(successValue: T, error: Error, failOnCalls: number[]): Mock<() => T>;
    /**
     * Creates a mock function with delay simulation
     */
    static createDelayedMock<T>(value: T, delayMs: number): Mock<() => Promise<T>>;
    /**
     * Creates a mock that tracks call arguments and returns different values based on them
     */
    static createArgumentBasedMock<T>(responseMap: Map<string, T>, defaultValue?: T): Mock<(...args: any[]) => T | undefined>;
    /**
     * Creates a mock that counts calls and has configurable behavior
     */
    static createCountingMock<T>(config: {
        returnValue?: T;
        throwAfterCalls?: number;
        error?: Error;
        maxCalls?: number;
    }): Mock<() => T | undefined>;
    /**
     * Utility to reset all mocks in an object
     */
    static resetAllMocks(obj: EnumerableObject): void;
    /**
     * Creates a spy that tracks calls without replacing implementation
     */
    static createSpy<T extends (...args: any[]) => any>(originalFn: T): Mock<T>;
    /**
     * Creates mock for protocol-specific responses
     */
    static createProtocolMocks(): {
        connectionManager: {
            getAccountInfo: Mock<() => Promise<ReturnType<typeof MockRpcResponses.getAccountInfo>>>;
            getTokenAccountsForOwner: Mock<() => Promise<ReturnType<typeof MockRpcResponses.getTokenAccountsByOwner>>>;
            getProgramAccounts: Mock<() => Promise<any[]>>;
            getMultipleAccounts: Mock<() => Promise<any[]>>;
            getNFTAccountsForOwner: Mock<() => Promise<any[]>>;
        };
        orca: {
            findWhirlpools: Mock<() => Promise<any[]>>;
            fetchPositions: Mock<() => Promise<any[]>>;
            calculateTokenAmounts: Mock<() => {
                tokenA: bigint;
                tokenB: bigint;
            }>;
        };
        meteora: {
            findDLMMPairs: Mock<() => Promise<any[]>>;
            findDynamicPools: Mock<() => Promise<any[]>>;
            findStablePools: Mock<() => Promise<any[]>>;
            getAllPositions: Mock<() => Promise<any[]>>;
            calculateLiquidity: Mock<() => {
                totalLiquidityX: bigint;
                totalLiquidityY: bigint;
            }>;
            getPoolTVL: Mock<() => {
                tvlUSD: number;
                tokenAValue: number;
                tokenBValue: number;
            }>;
        };
        raydium: {
            findAMMPools: Mock<() => Promise<any[]>>;
            findCLMMPools: Mock<() => Promise<any[]>>;
            findCPMMPools: Mock<() => Promise<any[]>>;
            getAllPositions: Mock<() => Promise<any[]>>;
            calculatePositionAmounts: Mock<() => {
                token0Amount: bigint;
                token1Amount: bigint;
            }>;
            getPoolTVL: Mock<() => Promise<{
                tvlUSD: number;
                token0Value: number;
                token1Value: number;
            }>>;
        };
    };
}

export declare class MockRpcResponses {
    static createSuccessResponse<T>(result: T, id?: number): any;
    static createErrorResponse(code: number, message: string, id?: number): any;
    static getAccountInfo(data?: string | null): any;
    static getTokenAccountsByOwner(accounts?: any[]): any;
    static getMultipleAccounts(accounts: Array<{
        data: string | null;
    }>): any;
}

export declare class MockTimer {
    private static enabled;
    private static currentTime;
    private static startTime;
    private static readonly originalSetTimeout;
    private static readonly originalClearTimeout;
    private static readonly originalDateNow;
    private static readonly originalPerformanceNow;
    static enable(): void;
    static disable(): void;
    static isEnabled(): boolean;
    static getCurrentTime(): number;
    static reset(): void;
}

export declare type MockWrapObject<T> = {
    [K in keyof T]: T[K] extends (...args: any[]) => any ? Mock<T[K]> : T[K] extends ((...args: any[]) => any) | undefined ? Mock<NonNullable<T[K]>> | undefined : T[K];
};

export declare type MultipartMimeType = "multipart/form-data" | "multipart/mixed";

export declare type Mutable<T> = {
    -readonly [P in keyof T]: T[P];
};

export declare const NASDAQ: NASDAQ_2;

export declare const NASDAQ100: Nasdaq100Name;

export declare type Nasdaq100 = EquityIndexId<"nasdaq-100">;

export declare type Nasdaq100Name = EquityIndexName<"Nasdaq-100">;

export declare type Nasdaq100Symbol = EquityIndexSymbol<"NDX">;

declare type NASDAQ_2 = ExchangeName<"NASDAQ">;

export declare type NASDAQId = ExchangeId<NASDAQ_2>;

export declare type NestedOmit<T, K extends string> = K extends `${infer P}.${infer S}` ? P extends keyof T ? T[P] extends object | undefined ? {
    [Key in keyof T]: Key extends P ? T[Key] extends object ? NestedOmit<T[Key], S> : undefined extends T[Key] ? NestedOmit<NonNullable<T[Key]>, S> | undefined : NestedOmit<T[Key], S> : T[Key];
} : T : T : Omit<T, K & keyof T>;

export declare type NestedSchemaField = Exclude<SchemaFieldType, SchemaFieldType.Array | SchemaFieldType.Vector | SchemaFieldType.NestedArray | SchemaFieldType.TaggedUnion | SchemaFieldType.BorshEnum | SchemaFieldType.SizedRawBytes>;

export declare const NETWORK_ERROR_MESSAGE: "network";

export declare const NETWORK_UNREACHABLE_MESSAGE: "network is unreachable";

export declare type NetworkAsset<T extends string> = Tagged<T, "NetworkAsset">;

export declare const NEW_YEARS_DAY: NewYearsDay;

export declare const NEW_YORK_TIMEZONE: NewYorkTimezone;

export declare type NewYearsDay = Holiday<"New Years Day">;

export declare type NewYorkTimezone = IANATimeZone<"America/New_York">;

export declare const NFP_RELEASE: NFPReleaseEvent;

export declare type NFPReleaseEvent = EconomicEvent<"NFPRelease">;

export declare const NIFTY50: Nifty50Name;

export declare type Nifty50 = EquityIndexId<"nifty-50">;

export declare type Nifty50Name = EquityIndexName<"Nifty 50">;

export declare type Nifty50Symbol = EquityIndexSymbol<"NSEI">;

export declare const NIKKEI225: Nikkei225Name;

export declare type Nikkei225 = EquityIndexId<"nikkei-225">;

export declare type Nikkei225Name = EquityIndexName<"Nikkei 225">;

export declare type Nikkei225Symbol = EquityIndexSymbol<"N225">;

/**
 * Error thrown when accounts is empty in the schema.
 */
export declare class NoAccountsEncoderError extends EncoderError {
    readonly type = "NO_ACCOUNTS";
    constructor(message?: string, originalError?: unknown);
}

export declare interface NodeJSError extends Error {
    code: string;
}

export declare type Nonce<N extends string | number = string> = Tagged<N, "Nonce">;

/**
 * Nonce information to be used to build an offline Transaction.
 */
export declare type NonceInformation = {
    /** The current blockhash stored in the nonce */
    nonce: Base58;
    /** AdvanceNonceAccount Instruction */
    nonceInstruction: AdvanceNonceAccountInstruction;
};

declare type NonFnKeys<T> = {
    [K in keyof T]: T[K] extends (...args: any) => any ? never : K;
}[keyof T];

export declare type NonFunctionProperties<T> = Pick<T, NonFunctionPropertyNames<T>>;

export declare type NonFunctionPropertyNames<T> = {
    [K in keyof T]: T[K] extends Function ? never : K;
}[keyof T];

export declare type NonNegative<T extends number> = `${T}` extends `-${string}` ? never : T;

export declare type NonNegativeBigInt<T extends bigint> = `${T}` extends `-${string}` ? never : T;

export declare type NonNil<T> = Exclude<T, null | undefined>;

export declare type NormalizeArray<T> = IsArrayish<T> extends true ? RearrayFrom<T, IntersectArrayElements<T>> : T;

export declare type NormalizeArrayProps<T> = {
    [K in keyof T]: NormalizeArray<T[K]>;
};

/**
 * Validates ISO-2 country code with optional fallback
 */
export declare function normalizeCountryCode(value: unknown): CountryCode | undefined;

export declare function normalizeCountryCode<T extends CountryCode>(value: unknown, fallback: T): CountryCode | T;

export declare function normalizeParameter(param: unknown): unknown;

/**
 * Normalize paramters for RPC requests:
 * - PublicKey to Base58 string
 * - bigint to string
 * - Uint8Array and Buffer to Base64
 */
export declare function normalizeParameters(params: unknown[]): unknown[];

export declare function normalizeSearchInputFull(raw: string): string;

/**
 * Validates IANA timezone string with optional fallback
 */
export declare function normalizeTimeZone(value: unknown): IANATimeZone | undefined;

export declare function normalizeTimeZone<T extends IANATimeZone>(value: unknown, fallback: T): IANATimeZone | T;

export declare function normalizeUrl(baseUrl: Url, ...pathSegments: string[]): Url;

export declare const NOT_FOUND_MESSAGE: "enotfound";

export declare type Note = Tagged<Text, "Note">;

export declare type NullableAwareRelationKeys<Container> = Extract<{
    [PropertyKey in keyof Container]: NonNullable<Container[PropertyKey]> extends Relation<any, any> ? PropertyKey : never;
}[keyof Container], string>;

export declare type NullableMetadata<T = Record<PropertyKey, any>> = Metadata<T> | null;

export declare type NullableOrUndefinedMetadata<T = Record<PropertyKey, any>> = Metadata<T> | null | undefined;

export declare type Nullish<T> = {
    [K in keyof T]?: T[K] | null | undefined;
};

/** All numeric tuples whose literal length is in [Min,Max] */
export declare type NumericTupleBetween<Min extends number, Max extends number, T extends number | bigint = number> = {
    [L in Range<Min, Max>]: BuildTuple<T, L>;
}[Range<Min, Max>];

export declare const NYSE: NYSE_2;

declare type NYSE_2 = ExchangeName<"NYSE">;

export declare type NYSEId = ExchangeId<NYSE_2>;

export declare type OAuthAccessToken<S extends string = string> = Tagged<AccessToken<Base64Url<S>>, "OAuthAccessToken">;

export declare type OAuthBearerToken<S extends string = string> = Tagged<BearerToken<Base64Url<S>>, "OAuthTokenType">;

export declare type OAuthClientId<S extends string = string> = Tagged<S, "OAuthClientId">;

export declare type OAuthClientSecret<S extends string = string> = Tagged<S, "OAuthClientSecret">;

export declare type OAuthCode<S extends string = string> = Tagged<S, "OAuthCode">;

export declare type OAuthCodeVerifier<S extends string = string> = Tagged<S, "OAuthCodeVerifier">;

export declare type OAuthIdToken<S extends string = string> = Tagged<Base64Url<S>, "OAuthIdToken">;

export declare type OAuthIssuer<S extends string = string> = Tagged<S, "OAuthIssuer">;

export declare type OAuthJTI<S extends string = string> = Tagged<S, "OAuthJTI">;

export declare type OAuthProvider<S extends string = string> = Tagged<S, "OAuthProvider">;

export declare type OAuthProviderUserId<S extends string = string> = Tagged<S, "OAuthProviderUserId">;

export declare type OAuthRefreshToken<S extends string = string> = Tagged<RefreshToken<Base64Url<S>>, "OAuthRefreshToken">;

export declare type OAuthScope<S extends string = string> = Tagged<S, "OAuthScope">;

export declare type OAuthState<S extends string = string> = Tagged<S, "OAuthState">;

export declare type OAuthSubScope<S extends string = string> = Tagged<OAuthScope<S>, "OAuthSubScope">;

export declare type OAuthToken = OAuthBearerToken | OAuthAccessToken | OAuthRefreshToken | OAuthIdToken;

export declare type OAuthVersion<V extends string | number = string> = Tagged<Version<V>, "OAuthVersion">;

export declare type ObservedDate<DateName extends string = string> = ObservedEvent<DateName>;

export declare type ObservedEvent<EventName extends string = string> = Tagged<EventName, "ObservedEvent">;

export declare const OKX: OKX_2;

declare type OKX_2 = ExchangeName<"OKX">;

export declare type OKXId = ExchangeId<OKX_2>;

export declare const ONDO_US_DOLLAR_YIELD: OndoUsDollarYield;

export declare const ONDO_US_DOLLAR_YIELD_NAME: OndoUsDollarYieldName;

export declare const ONDO_US_DOLLAR_YIELD_SYMBOL: OndoUsDollarYieldSymbol;

export declare type OndoUsDollarYield = TokenId<"ondo-us-dollar-yield">;

export declare type OndoUsDollarYieldName = TokenName<"Ondo US Dollar Yield">;

export declare type OndoUsDollarYieldSolana = SolanaAddressInfo<"USDSwr9ApdHk5bvJKMjzff41FfuX8bSxdKcR81vTwcA", {
    tokenId: SolanaToken<OndoUsDollarYield>;
    decimals: 6;
    program: "Token";
}>;

export declare type OndoUsDollarYieldSymbol = TokenSymbol<"USDY">;

export declare const ONE_DAY_MS: DurationMs<86400000>;

export declare const ONE_DAY_S: DurationS<86400>;

export declare const ONE_HOUR_MS: DurationMs<3600000>;

export declare const ONE_HOUR_S: DurationS<3600>;

export declare const ONE_MINUTE_MS: DurationMs<60000>;

export declare const ONE_MINUTE_S: DurationS<60>;

export declare const ONE_MONTH_MS: DurationMs<2592000000>;

export declare const ONE_MONTH_S: DurationS<2592000>;

export declare const ONE_SECOND_MS: DurationMs<1000>;

export declare const ONE_SECOND_S: DurationS<1>;

export declare const ONE_WEEK_MS: DurationMs<604800000>;

export declare const ONE_WEEK_S: DurationS<604800>;

export declare const ONE_YEAR_MS: DurationMs<31536000000>;

export declare const ONE_YEAR_S: DurationS<31536000>;

/**
 * Explicit break in the IR stream.
 *
 * Renderers decide whether this is a newline, paragraph break, spacing, etc.
 */
export declare interface OpBreak {
    /** Discriminator. */
    op: BreakOp;
    /** Optional number */
    size?: number;
}

export declare const OPENING_AUCTION_SESSION: OpeningAuctionSession;

export declare type OpeningAuctionSession = MarketSession<"Opening Auction">;

export declare const OPERA: Browser<"Opera">;

export declare type OperatingSystem<S extends string = string> = Tagged<Identity<S>, "OperatingSystem">;

export declare const OPERATION_TIMEOUT_MESSAGE: "operation timeout";

/**
 * A group-end opcode constrained by a domain's group-kind registry.
 */
declare type OpGroupEnd<Domain extends DisplayDomain> = {
    [Kind in DomainGroups<Domain> & string]: {
        op: GroupEndOp;
        kind: Kind;
        label?: string;
    };
}[DomainGroups<Domain>];

/**
 * End a group (pop the current group frame).
 */
export declare interface OpGroupEndBase {
    /** Discriminator. */
    op: GroupEndOp;
    /** Free-form group kind. */
    kind: string;
    /** Further categorization if needed. */
    label?: string;
}

/**
 * Enter the current group.
 *
 * Renderers typically increase indentation or switch layout mode here.
 */
export declare interface OpGroupEnter {
    /** Discriminator. */
    op: GroupEnterOp;
}

/**
 * Exit the current group.
 *
 * Renderers typically decrease indentation or close a layout context here.
 */
export declare interface OpGroupExit {
    /** Discriminator. */
    op: GroupExitOp;
}

/**
 * A group-exit-enter opcode constrained by a domain's group-kind registry.
 */
declare type OpGroupExitEnd<Domain extends DisplayDomain> = {
    [Kind in DomainGroups<Domain> & string]: {
        op: GroupExitEndOp;
        kind: Kind;
        label?: string;
    };
}[DomainGroups<Domain>];

/**
 * Exit marker emitted at the end of a group's final line.
 *
 * Some renderers treat this as a distinct layout event from `GROUP_EXIT`.
 */
export declare interface OpGroupExitEndBase {
    /** Discriminator. */
    op: GroupExitEndOp;
    /** Free-form group kind. */
    kind: string;
    /** Further categorization if needed. */
    label?: string;
}

/**
 * A group-start opcode constrained by a domain's group-kind registry.
 * With optional metadata.
 */
declare type OpGroupStart<Domain extends DisplayDomain> = {
    [Kind in DomainGroups<Domain> & string]: {
        op: GroupStartOp;
        kind: Kind;
        label?: string;
        meta?: Metadata;
    };
}[DomainGroups<Domain>];

/**
 * Start a group (unconstrained).
 *
 * This pushes a group frame on the renderer's stack.
 */
export declare interface OpGroupStartBase {
    /** Discriminator. */
    op: GroupStartOp;
    /** Free-form group kind. */
    kind: string;
    /** Further categorization if needed. */
    label?: string;
    /** Optional metadata. */
    meta?: Metadata;
}

/**
 * A group-start-enter opcode constrained by a domain's group-kind registry.
 * With optional metadata.
 */
declare type OpGroupStartEnter<Domain extends DisplayDomain> = {
    [Kind in DomainGroups<Domain> & string]: {
        op: GroupStartEnterOp;
        kind: Kind;
        label?: string;
        meta?: Metadata;
    };
}[DomainGroups<Domain>];

/**
 * Start a group and immediately enter it (unconstrained).
 *
 * Useful for groups that always contain child content and want a single opcode.
 */
export declare interface OpGroupStartEnterBase {
    /** Discriminator. */
    op: GroupStartEnterOp;
    /** Free-form group kind. */
    kind: string;
    /** Further categorization if needed. */
    label?: string;
    /** Optional metadata. */
    meta?: Metadata;
}

/**
 * An icon opcode constrained by a domain's icon-type and variant definitions.
 *
 * Variant behavior based on icon definition:
 * - ["type"] → variant?: string (any variant allowed, optional)
 * - ["type", "specificVariant"] → variant?: "specificVariant" (only that variant, optional)
 * - ["type", undefined] → variant?: undefined (must be omitted)
 */
declare type OpIcon<Domain extends DisplayDomain> = {
    [Type in DomainIconTypes<Domain> & string]: {
        op: IconOp;
        icon: Type;
        variant?: DomainIconVariant<Domain, Type>;
    };
}[DomainIconTypes<Domain>];

/**
 * A icon opcode (unconstrained).
 *
 * Renderers typically treat this as some type of graphic or glyph.
 */
export declare interface OpIconBase {
    /** Discriminator. */
    op: IconOp;
    /** Free-form type, i.e. "disc", "square". */
    icon: string;
    /** Variant i.e. "hollow", "solid" */
    variant?: string;
}

/**
 * A zero-width marker/anchor (unconstrained).
 *
 * This is useful for attaching metadata in-band (e.g., "start-of-section", "id:xyz"),
 * without rendering visible text.
 * It also can be used to otherwise highlight sections (e.g. "beginHighlight", "endHighlight")
 */
export declare interface OpMarker {
    /** Discriminator. */
    op: MarkerOp;
    /** Free-form marker tag. */
    tag: string;
    /** Optional metadata. */
    meta?: Metadata;
}

/**
 * A space.
 *
 * To use between SPAN, so formatting can allow for variants i.e `key=value`, `key: value`
 * or `key  |  value`.
 */
export declare interface OpSpace {
    /** Discriminator. */
    op: SpaceOp;
    /** Optional tag. */
    tag?: string;
    /** Optional number */
    size?: number;
}

/**
 * A span opcode constrained by a domain's span-tag and role definitions.
 *
 * Role behavior based on span definition:
 * - ["tag"] → role?: string (any role allowed, optional)
 * - ["tag", "specificRole"] → role?: "specificRole" (only that role, optional)
 * - ["tag", undefined] → role?: undefined (must be omitted)
 */
declare type OpSpan<Domain extends DisplayDomain> = {
    [SpanTag in DomainSpanTags<Domain> & string]: {
        op: SpanOp;
        tag: SpanTag;
        text: string | number | HasToString;
        role?: DomainSpanRole<Domain, SpanTag>;
    };
}[DomainSpanTags<Domain>];

/**
 * A span opcode (unconstrained).
 *
 * Renderers typically treat this as inline text with optional styling via `role`.
 */
export declare interface OpSpanBase {
    /** Discriminator. */
    op: SpanOp;
    /** Free-form span tag. */
    tag: string;
    /** Human-readable text content. */
    text: string | number | HasToString;
    /** Optional role (renderer-defined meaning). */
    role?: string;
}

export declare interface OptimisticValidation {
    optimisticValidation?: boolean;
}

declare type OptionalKeys<T> = {
    [K in keyof T]: IsOptional<T, K> extends true ? K : never;
}[keyof T];

export declare type OptionalNullish<T> = {
    [K in keyof T as undefined extends T[K] ? K : null extends T[K] ? K : never]?: T[K] | null | undefined;
} & {
    [K in keyof T as undefined extends T[K] ? never : null extends T[K] ? never : K]: T[K];
};

declare type OptionalProvidedKeys<T> = OptionalKeys<T> | (keyof T & "programId");

export declare class OptionalU64Wrapper implements IsCodable {
    value: bigint | null;
    constructor(properties: EncodableProps<OptionalU64Wrapper>);
    static decode(data: string | Uint8Array | Buffer | null): OptionalU64Wrapper;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare type OptionalUndefined<T> = {
    [K in keyof T as undefined extends T[K] ? K : never]?: T[K];
} & {
    [K in keyof T as undefined extends T[K] ? never : K]: T[K];
};

export declare type OptionExpiryType = Exclude<EventFrequency, "Daily" | "Yearly">;

export declare interface OptionsExpiryEntry extends BaseCalendarEntry {
    type: "OptionsExpiry";
    market: string;
    expiryType: OptionExpiryType;
}

export declare const ORCA: Orca;

export declare type Orca = ExchangeName<"Orca"> & SolanaProtocolName<"Orca">;

export declare const ORCA_URL: Url<"https://www.orca.so/">;

export declare type OrcaId = ExchangeId<Orca> & SolanaProtocolId<Orca>;

export declare interface OrderedOperation {
    ordinal: number;
    stackIndex: number;
    index: number;
    parentIndex: number;
    inferred: boolean;
}

/**
 * Ordering info size in bytes
 */
export declare const ORDERING_INFO_SIZE = 16;

export declare const OVERNIGHT_SESSION: OvernightSession;

export declare type OvernightSession = MarketSession<"Overnight">;

export declare type Override<T> = {
    __override: T;
};

/** Owner Address */
export declare type OwnerSpan = ["owner", LabelValueRole];

/**
 * Maximum over-the-wire size of a Solana packet payload (transaction bytes).
 *
 * 1280 is IPv6 minimum MTU
 * 40 bytes is the size of the IPv6 header
 * 8 bytes is the size of the fragment header
 */
export declare const PACKET_DATA_SIZE: number;

export declare interface PaginatedResponse<T = unknown> {
    data: T[];
    total: number;
    limit: number;
    offset: number;
    page: number;
}

/**
 * Parses coordinate string "lat,lng" into [Latitude, Longitude] with optional fallback
 */
export declare function parseCoordinates(value: unknown): [Latitude, Longitude] | undefined;

export declare function parseCoordinates<T extends [Latitude, Longitude]>(value: unknown, fallback: T): [Latitude, Longitude] | T;

export declare interface ParsedAccountKey {
    pubkey: SolanaAddress;
    signer: boolean;
    source: "transaction" | "lookupTable";
    writable: boolean;
}

export declare interface ParsedInstruction extends OrderedOperation {
    programId: PublicKey;
    accounts: PublicKey[];
    programIdIndex: number;
    accountIndexes: number[];
    instruction: MessageInstruction_2;
}

export declare type ParsedLog = BaseParsedLog | AnchorParsedLog;

export declare interface ParsedTransactionData extends RpcTransactionData {
    transaction: {
        message: {
            accountKeys: ParsedAccountKey[];
            instructions: {
                parsed?: {
                    info: Metadata;
                    type: string;
                };
                program?: SolanaAddress;
                programId: SolanaAddress;
                accounts?: SolanaAddress[];
                data?: Base58 | null;
                stackHeight?: number | null;
            }[];
            recentBlockhash: SolanaBlockhash;
            addressTableLookups?: AddressTableLookup[];
        };
        signatures: SolanaTransactionSignature[];
    };
    version?: SolanaTransactionVersion;
}

export declare function parseHostPort(value: string, defaultHost?: HostName, defaultPort?: Port): {
    host: HostName;
    port?: Port;
};

export declare function parseTpcOrIpcEndpoint(value: string, defaultHost?: HostName, defaultPort?: Port): {
    path: string;
} | {
    host: string;
    port?: Port;
};

export declare type PascalCase<T extends string = string> = Tagged<ValidPascalCasePattern<T>, "PascalCase">;

export declare type Password = Tagged<string, "Password">;

export declare type PasswordMinLength<T extends string, L extends number = 20> = T extends string ? T["length"] extends number ? T["length"] extends L ? T : never : never : never;

export declare const PAYPAL_USD: PayPalUsd;

export declare const PAYPAL_USD_NAME: PayPalUsdName;

export declare const PAYPAL_USD_SYMBOL: PayPalUsdSymbol;

export declare type PayPalUsd = TokenId<"paypal-usd">;

export declare type PayPalUsdName = TokenName<"PayPal USD">;

export declare type PayPalUsdSolana = SolanaAddressInfo<"2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo", {
    tokenId: SolanaToken<PayPalUsd>;
    decimals: 6;
    program: "Token 2022";
}>;

export declare type PayPalUsdSymbol = TokenSymbol<"PYUSD">;

export declare const PERENA: Perena;

export declare type Perena = SolanaProtocolName<"Perena">;

export declare const PERENA_URL: Url<"https://perena.org/">;

export declare type PerenaId = SolanaProtocolId<Perena>;

export declare type PerpetualTradingPair<TBase extends string = string, TQuote extends string = string> = Tagged<FuturesTradingPair<TBase, TQuote>, "PerpetualTradingPair">;

export declare type PersonName = Tagged<Identity<string>, "PersonName">;

export declare const PHANTOM_WALLET_URL: Url<"https://phantom.com/">;

export declare const PMI_RELEASE: PMIReleaseEvent;

export declare type PMIReleaseEvent = EconomicEvent<"PMIRelease">;

export declare const POOL_NTP_ORG: Domain<"pool.ntp.org">;

export declare type Port<N extends number = number> = Tagged<N, "Port">;

export declare const PPI_RELEASE: PPIReleaseEvent;

export declare type PPIReleaseEvent = EconomicEvent<"PPIRelease">;

export declare const PRE_MARKET_SESSION: PreMarketSession;

export declare type PreMarketSession = MarketSession<"Pre-Market">;

export declare const PRESIDENTS_DAY: PresidentsDay;

export declare type PresidentsDay = Holiday<"Presidents Day">;

export declare type Prettify<T> = {
    [K in keyof T]: T[K];
} & {};

export declare type PrimaryId = Tagged<Id<number>, "UserId">;

export declare type PrimaryIdStr = Tagged<Id<string>, "UserId">;

export declare type Primitive = string | number | boolean | bigint | symbol | null | undefined;

export declare type PrivateKey = Tagged<string | Uint8Array, "PrivateKey">;

export declare interface ProgramAccountsConfig {
    encoding?: SolanaRpcEncoding;
    commitment?: SolanaCommitmentLevel;
    dataSlice?: {
        length: number;
        offset?: number;
    };
    filters?: ProgramAccountsConfigFilter[];
}

export declare interface ProgramAccountsConfigEncodedBytes {
    encoding?: SolanaRpcEncoding;
    commitment?: SolanaCommitmentLevel;
    dataSlice?: {
        length: number;
        offset?: number;
    };
    filters?: {
        dataSize?: number;
        memcmp?: {
            offset: number;
            bytes: SolanaAddressLike | Base58 | Base64 | Buffer | Uint8Array;
        };
    }[];
}

export declare interface ProgramAccountsConfigFilter {
    dataSize?: number;
    memcmp?: {
        offset: number;
        bytes: SolanaAddressLike | Base58 | Base64 | Buffer | Uint8Array;
        encoding?: "base64" | "base58" | "bytes";
    };
}

/**
 * Error thrown when program address derivation fails
 */
export declare class ProgramAddressDerivationError extends PublicKeyError {
    readonly seeds?: (Buffer | Uint8Array)[] | undefined;
    readonly programId?: (PublicKey_2 | Buffer | Uint8Array) | undefined;
    readonly type = "PROGRAM_ID_DERIVATION";
    constructor(message?: string, seeds?: (Buffer | Uint8Array)[] | undefined, programId?: (PublicKey_2 | Buffer | Uint8Array) | undefined, originalError?: unknown);
}

export declare interface ProgramAnchorErrorLog extends ProgramLog {
    subType: "error";
    name: string;
    code?: string;
    number?: number | string;
    message?: string;
}

export declare interface ProgramDerivedAccountDefinition {
    seeds: readonly ProgramDerivedAccountSeed[];
    program?: ProgramDerivedAccountSeed;
}

export declare type ProgramDerivedAccountSchema = Record<string, ProgramDerivedAccountDefinition>;

export declare type ProgramDerivedAccountSeed = ProgramDerivedAccountSeedConst | ProgramDerivedAccountSeedArg | ProgramDerivedAccountSeedAccount;

export declare interface ProgramDerivedAccountSeedAccount {
    kind: "account";
    path: string;
    account?: string;
}

export declare interface ProgramDerivedAccountSeedArg {
    kind: "arg";
    path: string;
}

export declare interface ProgramDerivedAccountSeedConst {
    kind: "const";
    value: readonly number[];
}

/**
 * Thrown when program ID is missing from account meta list
 */
export declare class ProgramIdNotFoundError extends TransactionError {
    readonly programId?: SolanaAddress | undefined;
    readonly type = "PROGRAM_ID_NOT_FOUND";
    constructor(message?: string, programId?: SolanaAddress | undefined, originalError?: unknown);
}

export declare interface ProgramInstructionLog extends ProgramLog {
    subType: "instruction";
    instructionName: string;
}

export declare interface ProgramLog extends BaseLog {
    type: "log";
    programId: PublicKey_2;
    data: string | null;
}

export declare interface ProgramNotification {
    context: {
        slot: SolanaSlot;
    };
    value: {
        pubkey: SolanaAddress;
        account: AccountInfo;
    };
}

/** Program Id */
export declare type ProgramSpan = ["program", LabelValueRole];

export declare interface ProgramStack {
    programId: PublicKey_2;
    depth: number;
}

export declare interface ProgramSubscribeConfig {
    commitment?: SolanaCommitmentLevel;
    encoding?: "base58" | "base64" | "base64+zstd" | "jsonParsed";
    filters?: Array<{
        dataSize?: number;
        memcmp?: {
            offset: number;
            bytes: string;
            encoding?: "base58" | "base64";
        };
    }>;
}

export declare type ProtocolId<T extends string> = Tagged<T, "ProtocolId">;

export declare type ProtocolName<T extends string> = Tagged<T, "ProtocolName">;

/**
 * Default file, folder and directory paths.
 */
export declare const PUBLIC_DIR: "public";

/**
 * Public key length in bytes (32 bytes)
 */
export declare const PUBLIC_KEY_BYTE_LENGTH = 32;

export declare class PublicKey<S extends string = SolanaAddress> implements PublicKey_2, BrandedTransformable {
    readonly [PublicKeyBrand]: true;
    readonly [TransformableBrand]: true;
    private readonly _key;
    private _bs58;
    private static cache;
    private static readonly PROGRAM_DERIVED_ADDRESS_BYTES;
    private readonly [PublicKeySymbol];
    static [Symbol.hasInstance](value: unknown): boolean;
    constructor(value: PublicKeyLikeInput<S> | number);
    toBase58(): SolanaAddress;
    toBuffer(): Uint8Array;
    toBytes(): Uint8Array;
    toString(): SolanaAddress;
    equals(other: SolanaAddressLike | Uint8Array): boolean;
    static fromLegacy(publicKey: LegacyPublicKey | PublicKey): PublicKey;
    /**
     * Validates if a public key is on the Ed25519 curve
     */
    static isOnCurve(pubkey: SolanaAddressLike): boolean;
    /**
     * Validates that program id public key is valid Base58 or a known system program id
     */
    static validateProgramId(programId: SolanaAddressLike): void;
    static isNativeProgramAddress(address: SolanaBase58AddressString): boolean;
    /**
     * Derives a program address from seeds and program ID
     */
    static findProgramAddress(seeds: (Buffer | Uint8Array)[], programId: PublicKey | Buffer | Uint8Array): Promise<[PublicKey, number]>;
    private static findProgramAddressInternal;
    /**
     * Creates a program address without searching (deterministic)
     */
    static createProgramAddress(seeds: (Buffer | Uint8Array)[], programId: PublicKey | Buffer | Uint8Array): Promise<PublicKey>;
    /**
     * Finds the Associated Token Account address for a given mint and owner
     */
    static findAssociatedTokenAddress(owner: PublicKey | SolanaAddressLike, mint: PublicKey | SolanaAddressLike): Promise<PublicKey>;
    /**
     * Finds the Associated Token Account address for Token-2022 tokens
     */
    static findAssociatedTokenAddressToken2022(owner: PublicKey | SolanaAddressLike, mint: PublicKey | SolanaAddressLike): Promise<PublicKey>;
    private static findATAInternal;
    /**
     * Validates a string as a valid base58 public key
     */
    static isValidBase58(input: string): boolean;
    static isBase58PublicKey(input: string): input is SolanaAddress;
    /**
     * Creates a PublicKey from hex string
     */
    static fromHex(hex: string): PublicKey;
    /**
     * Converts PublicKey to hex string
     */
    toHex(): string;
}

declare interface PublicKey_2 extends BrandedPublicKey {
    toBase58(): SolanaAddress;
    toBuffer(): Uint8Array;
    toBytes(): Uint8Array;
    toString(): SolanaAddress;
    equals(other: SolanaAddressLike): boolean;
    toHex(): string;
}

export declare const PublicKeyBrand: unique symbol;

export declare class PublicKeyCache {
    private static instance;
    private readonly storage;
    private readonly isPersistent;
    private readonly commonAddresses;
    private readonly programAddresses;
    private readonly ataAddresses;
    private readonly uint8ArrayPool;
    private readonly POOL_SIZE;
    private isDirty;
    private saveTimeout;
    private readonly SAVE_DELAY;
    private constructor();
    static getInstance(): PublicKeyCache;
    private getPooledUint8Array;
    private returnToPool;
    getCommonAddress(base58: string): Uint8Array | null;
    setCommonAddress(base58: string, bytes: Uint8Array): void;
    getProgramAddress(key: string): [Uint8Array, number] | null;
    setProgramAddress(key: string, bytes: Uint8Array, nonce: number): void;
    getATAAddress(key: string): Uint8Array | null;
    setATAAddress(key: string, bytes: Uint8Array): void;
    private scheduleSave;
    private saveToPersistentStorage;
    private loadFromPersistentStorage;
    private serializeCache;
    private deserializeCache;
    clearAll(): Promise<void>;
}

export declare type PublicKeyConstructor = new (value: PublicKeyLikeInput) => PublicKey_2;

/**
 * Base class for PublicKey-related errors
 */
export declare abstract class PublicKeyError extends SolomonLabsError {
    readonly category = "PUBLIC_KEY";
}

export declare type PublicKeyLike = PublicKey_2 | LegacyPublicKey;

export declare type PublicKeyLikeConstructor = PublicKeyConstructor | LegacyPublicKeyConstructor;

export declare type PublicKeyLikeInput<S extends string = string> = HexLower<S, 64, 64> | Base64<S, 43, 44> | SolanaAddress | SolanaKitAddress | LiteralHexLower<S, 64, 64> | LiteralBase58<S, 32, 44> | LiteralBase64<S, 32, 44> | Uint8Array | Buffer | Bytes32 | ReadonlyBytes32 | ReadonlyUint8Array;

export declare const PYTH: Pyth;

export declare type Pyth = SolanaProtocolName<"Pyth">;

export declare const PYTH_URL: Url<"https://pyth.network/">;

export declare type PythId = SolanaProtocolId<Pyth>;

export declare interface QuarterEndEntry extends BaseCalendarEntry {
    type: "QuarterEnd";
    quarter: number;
    year: number;
    tradingDayEnd: Date;
}

export declare const QUOTA_EXCEEDED_MESSAGE: "quota exceeded";

declare type Range<From extends number, To extends number> = Exclude<Enumerate<To>, Enumerate<From>> | From;

export declare function rangeValidator(min: number, max: number): (value: number) => boolean;

export declare const RATE_LIMIT_MESSAGE: "rate limit";

export declare const RATE_X: RateX;

export declare const RATE_X_URL: Url<"https://ratex.trade/">;

export declare type RateX = SolanaProtocolName<"RateX">;

export declare type RateXId = SolanaProtocolId<RateX>;

export declare function rawAmountFromUiMeta(meta: {
    amount: string | number | bigint;
    decimals: number | null;
    uiAmount: number | string | null;
    uiAmountString: string;
}): BigDecimal;

export declare type RawPassword<L extends number = 12> = Tagged<PasswordMinLength<Password, L>, "RawPassword">;

export declare interface RawTransactionData extends RpcTransactionData {
    transaction: string | [string, string];
    version?: SolanaTransactionVersion;
}

export declare const RAYDIUM: Raydium;

export declare type Raydium = ExchangeName<"Raydium"> & SolanaProtocolName<"Raydium">;

export declare type RaydiumId = ExchangeId<Raydium> & SolanaProtocolId<Raydium>;

export declare const RE_OPENING_AUCTION_SESSION: ReOpeningAuctionSession;

export declare const READ_TIMEOUT_MESSAGE: "read timeout";

export declare type ReadonlyBytes32 = ReadonlyTuple<number, 32>;

/**
 * Thrown when transaction or message is readonly
 */
export declare class ReadonlyError extends TransactionError {
    readonly methodName?: string | undefined;
    readonly type = "READONLY";
    constructor(message?: string, methodName?: string | undefined, originalError?: unknown);
}

declare type ReadonlyKeys<T> = {
    [K in keyof T]: IsReadonly<T, K> extends true ? K : never;
}[keyof T];

export declare type ReadonlyTuple<T, Len extends number> = Readonly<BuildTuple<T, Len>>;

export declare interface ReadonlyUint8Array extends Omit<Uint8Array, TypedArrayMutableProperties> {
    readonly [n: number]: number;
}

export declare type RearrayFrom<T, E> = T extends readonly unknown[] ? readonly E[] : T extends unknown[] ? E[] : T;

export declare const RECENT_CALLS_MAX = 50;

export declare const RED_STONE: RedStone;

export declare const RED_STONE_URL: Url<"https://redstone.finance/">;

export declare function redactKeyValues(input: unknown, options?: RedactOptions): unknown;

export declare interface RedactOptions {
    extraSensitiveKeys?: readonly string[];
}

export declare type RedStone = SolanaProtocolName<"RedStone">;

export declare type RedStoneId = SolanaProtocolId<RedStone>;

export declare type RefreshToken<S extends string = string> = Tagged<ApiKey<S>, "RefreshToken">;

export declare const REGULAR_TRADING_SESSION: RegularTradingSession;

export declare type RegularTradingSession = MarketSession<"Regular Trading">;

export declare type Relation<EntityInterface, EntityClass extends EntityInterface = EntityInterface> = BrandedRelationInterface<EntityInterface> & BrandedRelationEntity<EntityClass> & EntityInterface;

export declare type RelationEntity<EntityClass> = ExtractBrandedType<EntityClass, typeof RelationEntityBrand>;

export declare const RelationEntityBrand: unique symbol;

export declare type RelationEntityKey<EntityClass, PropertyKey extends keyof EntityClass> = RelationEntity<EntityClass[PropertyKey]>;

export declare type RelationInterface<EntityInterface> = ExtractBrandedType<EntityInterface, typeof RelationInterfaceBrand>;

export declare const RelationInterfaceBrand: unique symbol;

export declare type RelationInterfaceKey<EntityInterface, PropertyKey extends keyof EntityInterface> = RelationInterface<EntityInterface[PropertyKey]>;

export declare type RelationKeys<Container> = {
    [PropertyKey in keyof Container]-?: Container[PropertyKey] extends Relation<any, any> ? PropertyKey : never;
}[keyof Container];

export declare type RemapIdBases<T, Alias extends StringMap, Only extends string = string> = {
    [K in keyof T as K extends `${infer P}Id` ? StripNullish<T[K]> extends PrimaryId ? P extends Only ? `${AliasOf<P, Alias>}Id` : K : K : K]: T[K];
};

declare type RemapIdKeyIfPrimary<K extends PropertyKey, V, Only extends string> = StripNullish<V> extends PrimaryId ? (K extends `${infer P}Id` ? (P extends Only ? `${P}Uuid` : K) : K) : K;

export declare type ReOpeningAuctionSession = MarketSession<"Re-Opening Auction">;

export declare type ReplyMock = HttpReply & {
    code: ReturnType<typeof mock.fn>;
    send: ReturnType<typeof mock.fn>;
    header: ReturnType<typeof mock.fn>;
    type: ReturnType<typeof mock.fn>;
    redirect: ReturnType<typeof mock.fn>;
};

export declare const REQUEST_LIMIT_MESSAGE: "request limit";

export declare const REQUEST_TIMEOUT_MESSAGE: "request timeout";

/**
 * Request Heap Frame instruction
 */
export declare class RequestHeapFrameInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    bytes: number;
    constructor(props: EncodableProps<RequestHeapFrameInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): RequestHeapFrameInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const REQUIRED_FIELD_MESSAGE: "required field";

declare type RequiredKeys<T> = Exclude<FilteredKeys<T>, DefaultedKeys<T> | OptionalKeys<T> | (keyof T & "programId")>;

export declare interface ResolvedAddressTableLookups {
    writable: PublicKeyLike[];
    readonly: PublicKeyLike[];
}

export declare const RETAIL_SALES_RELEASE: RetailSalesReleaseEvent;

export declare type RetailSalesReleaseEvent = EconomicEvent<"RetailSalesRelease">;

export declare interface ReturnLog extends BaseLog {
    type: "return";
    programId: PublicKey_2;
    data: Base64;
}

/** 5 = Revoke {} */
export declare class RevokeInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    source: SolanaAddressLike;
    owner: SolanaAddressLike;
    constructor(props: EncodableProps<RevokeInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): RevokeInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const RIPPLE: Ripple;

export declare type Ripple = AssetId<"ripple">;

export declare const RIPPLE_NAME: RippleName;

export declare const RIPPLE_SYMBOL: RippleSymbol;

export declare type RippleName = AssetName<"Ripple">;

export declare type RippleSymbol = AssetSymbol<"XRP">;

export declare interface RootNotification {
    root: SolanaSlot;
}

export declare function rot(str: string, shift: number): string;

export declare type RoundingMode = "Floor" | "Ceil" | "Round" | "Truncate";

export declare const RPC_CLIENT_FEATURE_ID: FeatureId<"solana.rpcClient">;

/**
 * Solana RPC client built on top of the general-purpose HttpClient
 */
export declare class RpcClient {
    private readonly httpClient;
    private readonly primaryEndpoint;
    private static readonly requestIdForEndpoint;
    private readonly optimisticValidation;
    private readonly observability;
    constructor(endpoint: Url | EndpointConfig, rateLimitConfig?: Partial<RateLimitConfig & OptimisticValidation>, retryConfig?: Partial<RetryConfig>);
    /** Get the next request ID based on primary endpoint */
    private getNextRequestId;
    /** Get current endpoint being used by HttpClient */
    private getCurrentEndpoint;
    /** Validate transaction signature */
    private validateTransactionSignature;
    /** Validate single-pubkey RPC calls (getAccountInfo, getBalance, ...) */
    private validateSingleAccountCall;
    /** Validate getMultipleAccounts */
    private validateMultipleAccountsCall;
    /** Validate sendTransaction / simulateTransaction parameters */
    private validateTransactionSubmission;
    /**
     * Request method that returns raw ArrayBuffer instead of parsed JSON.
     * Useful for binary parsing optimizations.
     */
    requestBuffer(method: string, params?: unknown[], options?: RpcRequestOptions): Promise<ArrayBuffer>;
    /**
     * Enhanced request method with abort signal support
     */
    request<T>(method: string, params?: unknown[] | object, options?: RpcRequestOptions): Promise<T>;
    getAccountInfo(publicKey: SolanaAddressLike, config?: RpcConfig, options?: RpcRequestOptions): Promise<GetAccountInfoResponse | null>;
    getMultipleAccountsInfo(publicKeys: SolanaAddressLike[], config?: RpcConfig, options?: RpcRequestOptions): Promise<GetMultipleAccountsResponse | null>;
    getTokenAccountsByOwner(owner: SolanaAddressLike, filter: TokenAccountsFilter, config?: RpcConfig, options?: RpcRequestOptions): Promise<GetTokenAccountsByOwnerResponse | null>;
    getTokenAccountBalance(owner: SolanaAddressLike, options?: RpcRequestOptions): Promise<GetTokenAccountBalanceResponse | null>;
    getTokenAccounts(config: GetTokenAccountsConfig, options?: RpcRequestOptions): Promise<GetTokenAccountsResponse | null>;
    getProgramAccounts(programId: SolanaAddressLike, config?: ProgramAccountsConfig, options?: RpcRequestOptions): Promise<GetProgramAccountsResponse | null>;
    /**
     * Enumerate every SPL-token account for `mint` with its exact raw balance.
     * Built on getProgramAccounts with the canonical dataSize=165 + mint memcmp
     * filter and a dataSlice over the 8-byte amount field (offset 64) so the
     * response stays small even for large holder sets.
     *
     * Caveats: requires a provider that permits getProgramAccounts on the token
     * program (blocked on the public mainnet endpoint), and dataSize=165 excludes
     * Token-2022 accounts that carry extensions (pass the Token-2022 programId for
     * a Token-2022 mint; extended accounts will still be missed by the size filter).
     */
    getTokenAccountsByMint(mint: SolanaAddressLike, programId?: SolanaAddressLike, options?: RpcRequestOptions): Promise<MintHolder[]>;
    getSignaturesForAddress(address: SolanaAddressLike, config?: GetSignaturesForAddressConfig, commitment?: SolanaCommitmentLevel, options?: RpcRequestOptions): Promise<ConfirmedSignatureInfo[] | ArrayBuffer>;
    getTransactionsForAddress(address: SolanaAddressLike, config?: GetTransactionsForAddressConfig & {
        transactionDetails?: "signatures";
        encoding?: never;
    }, options?: RpcRequestOptions): Promise<GetTransactionsForAddressSignaturesResponse | ArrayBuffer>;
    getTransactionsForAddress(address: SolanaAddressLike, config: GetTransactionsForAddressConfig & {
        transactionDetails: "full";
    }, options?: RpcRequestOptions): Promise<GetTransactionsForAddressFullResponse | ArrayBuffer>;
    getSlot(commitment?: SolanaCommitmentLevel, options?: RpcRequestOptions): Promise<number | null>;
    getBlockHeight(commitment?: SolanaCommitmentLevel, options?: RpcRequestOptions): Promise<number | null>;
    getBalance(publicKey: SolanaAddressLike, commitment?: SolanaCommitmentLevel, options?: RpcRequestOptions): Promise<GetBalanceResponse | null>;
    /**
     * Get latest blockhash
     */
    getLatestBlockhash(commitment?: SolanaCommitmentLevel, options?: RpcRequestOptions): Promise<GetLatestBlockhashResponse | null>;
    /**
     * Get latest blockhash and context (equivalent to getLatestBlockhashAndContext)
     */
    getLatestBlockhashAndContext(commitment?: SolanaCommitmentLevel, options?: RpcRequestOptions): Promise<GetLatestBlockhashAndContextResponse | null>;
    getTransaction(signature: string, config?: GetTransactionConfig, options?: RpcRequestOptions): Promise<GetTransactionResponse>;
    sendTransaction(transaction: string, config?: SendTransactionConfig, options?: RpcRequestOptions): Promise<string | null>;
    simulateTransaction(transaction: string, config?: SimulateTransactionConfig, options?: RpcRequestOptions): Promise<SimulateTransactionResponse | null>;
    /**
     * Get access to the underlying HttpClient for advanced use cases
     */
    getHttpClient(): HttpClient;
    /**
     * Update endpoint configuration
     */
    updateEndpoints(config: EndpointConfig): void;
    /**
     * Get current endpoint status including health information
     */
    getEndpointStatus(): {
        current: string;
        states: Array<{
            endpoint: string;
            isHealthy: boolean;
            consecutiveFailures: number;
        }>;
    };
    /**
     * Update rate limit configuration
     */
    updateRateLimitConfig(config: Partial<RateLimitConfig>): void;
    /**
     * Update retry configuration
     */
    updateRetryConfig(config: Partial<RetryConfig>): void;
    /**
     * Get current configuration
     */
    getConfig(): {
        rateLimit: RateLimitConfig;
        retry: RetryConfig;
        endpoint: EndpointConfig;
    };
    /**
     * Clear the request queue (useful for testing or resetting rate limits)
     */
    clearRequestQueue(): void;
    /**
     * Clean up resources
     */
    destroy(): Promise<void>;
}

export declare enum RpcClientEvents {
    Request = "request",
    Error = "error"
}

export declare type RpcClientObservability = ObservabilityLeaf<RpcClientStats, ObservabilityFeatureEventsFromEnum<typeof RpcClientEvents>>;

declare interface RpcClientRecentCall {
    startedTimestamp: TimestampMs;
    endedTimestamp: TimestampMs;
    duration: DurationMs;
    rpcMethod: string;
    ok: boolean;
    rpcErrorCode: number;
    error: SolomonLabsError | null;
}

export declare interface RpcClientStats extends ObservabilityStats {
    rpcCallsTotal: number;
    rpcCallsOk: number;
    rpcCallsFail: number;
    lastRpcMethod: string;
    lastRpcErrorCode: number;
    errors: SolomonLabsError[];
    calls: RpcClientRecentCall[];
}

export declare interface RpcConfig {
    encoding?: SolanaRpcEncoding;
    commitment?: SolanaCommitmentLevel;
}

/**
 * Base class for RPC-related errors
 */
export declare abstract class RpcError extends SolomonLabsError {
    readonly category = "RPC";
}

/**
 * Error thrown for RPC protocol errors
 */
export declare class RpcProtocolError extends RpcError {
    readonly code?: number | undefined;
    readonly data?: unknown | undefined;
    readonly type = "RPC_PROTOCOL";
    constructor(message: string, code?: number | undefined, data?: unknown | undefined, originalError?: unknown);
}

export declare interface RpcRequestOptions {
    signal?: AbortSignal;
    timeout?: number;
    /**
     * Response format for the RPC call.
     * - "json": Returns parsed JSON response (default)
     * - "buffer": Returns raw ArrayBuffer for binary parsing optimizations
     */
    responseFormat?: "json" | "buffer";
}

export declare interface RpcResponse<T = unknown> {
    jsonrpc: string;
    id: string | number;
    result?: T;
    error?: {
        code: number;
        message: string;
        data?: unknown;
    };
}

export declare interface RpcTransactionData {
    blockTime?: TimestampS | null;
    meta: TransactionMeta | null;
    slot: SolanaSlot;
}

/**
 * Solana RPC WebSocket client for real-time subscriptions.
 *
 * Supports:
 * - accountSubscribe / accountUnsubscribe
 * - logsSubscribe / logsUnsubscribe
 * - programSubscribe / programUnsubscribe
 * - signatureSubscribe / signatureUnsubscribe
 * - slotSubscribe / slotUnsubscribe
 * - blockSubscribe / blockUnsubscribe (requires unstable RPC)
 *
 * @example
 * ```typescript
 * const ws = new RpcWebsocketClient({
 *     url: "wss://api.mainnet-beta.solana.com"
 * });
 *
 * await ws.connect();
 *
 * // Subscribe to account changes
 * const subId = await ws.accountSubscribe(
 *     "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
 *     (notification) => console.log("Account changed:", notification),
 *     { encoding: "base64" }
 * );
 *
 * // Later, unsubscribe
 * await ws.accountUnsubscribe(subId);
 *
 * // Disconnect when done
 * ws.disconnect();
 * ```
 */
export declare class RpcWebsocketClient {
    private readonly wsAssistant;
    private readonly subscriptions;
    private readonly pendingRequests;
    private requestId;
    private messageLoopRunning;
    constructor(config: RpcWebsocketConfig);
    /**
     * Connect to the WebSocket server and start the message loop.
     */
    connect(): Promise<void>;
    /**
     * Disconnect from the WebSocket server.
     */
    disconnect(): void;
    /**
     * Check if the WebSocket is connected.
     */
    isConnected(): boolean;
    /**
     * Subscribe to account changes.
     *
     * @param account - The account public key to watch
     * @param callback - Called when the account data changes
     * @param config - Optional subscription configuration
     * @returns Subscription ID for unsubscribing
     */
    accountSubscribe(account: SolanaAddress, callback: (notification: AccountNotification) => void, config?: AccountSubscribeConfig): Promise<SubscriptionId>;
    /**
     * Unsubscribe from account changes.
     */
    accountUnsubscribe(subscriptionId: SubscriptionId): Promise<boolean>;
    /**
     * Subscribe to transaction logs.
     *
     * Note: Solana RPC only supports ONE address per logsSubscribe.
     * For multiple addresses, create multiple subscriptions.
     *
     * @param filter - "all", "allWithVotes", or { mentions: [address] }
     * @param callback - Called when a transaction log is received
     * @param config - Optional subscription configuration
     * @returns Subscription ID for unsubscribing
     */
    logsSubscribe(filter: LogsSubscribeFilter, callback: (notification: LogsNotification) => void, config?: LogsSubscribeConfig): Promise<SubscriptionId>;
    /**
     * Unsubscribe from transaction logs.
     */
    logsUnsubscribe(subscriptionId: SubscriptionId): Promise<boolean>;
    /**
     * Subscribe to program account changes.
     *
     * @param programId - The program public key to watch
     * @param callback - Called when a program account changes
     * @param config - Optional subscription configuration with filters
     * @returns Subscription ID for unsubscribing
     */
    programSubscribe(programId: SolanaAddress, callback: (notification: ProgramNotification) => void, config?: ProgramSubscribeConfig): Promise<SubscriptionId>;
    /**
     * Unsubscribe from program account changes.
     */
    programUnsubscribe(subscriptionId: SubscriptionId): Promise<boolean>;
    /**
     * Subscribe to a specific transaction signature.
     *
     * The subscription is automatically removed once the transaction
     * is confirmed or fails.
     *
     * @param signature - The transaction signature to watch
     * @param callback - Called when the transaction status changes
     * @param config - Optional subscription configuration
     * @returns Subscription ID for unsubscribing
     */
    signatureSubscribe(signature: SolanaTransactionSignature, callback: (notification: SignatureNotification) => void, config?: SignatureSubscribeConfig): Promise<SubscriptionId>;
    /**
     * Unsubscribe from signature status updates.
     */
    signatureUnsubscribe(subscriptionId: SubscriptionId): Promise<boolean>;
    /**
     * Subscribe to slot updates.
     *
     * @param callback - Called when a new slot is processed
     * @returns Subscription ID for unsubscribing
     */
    slotSubscribe(callback: (notification: SlotNotification) => void): Promise<SubscriptionId>;
    /**
     * Unsubscribe from slot updates.
     */
    slotUnsubscribe(subscriptionId: SubscriptionId): Promise<boolean>;
    /**
     * Subscribe to block updates.
     *
     * Note: Requires unstable RPC methods to be enabled on the node.
     *
     * @param filter - "all" or { mentionsAccountOrProgram: address }
     * @param callback - Called when a new block is received
     * @param config - Optional subscription configuration
     * @returns Subscription ID for unsubscribing
     */
    blockSubscribe(filter: BlockSubscribeFilter, callback: (notification: unknown) => void, config?: BlockSubscribeConfig): Promise<SubscriptionId>;
    /**
     * Unsubscribe from block updates.
     */
    blockUnsubscribe(subscriptionId: SubscriptionId): Promise<boolean>;
    /**
     * Get all active subscription IDs.
     */
    getActiveSubscriptions(): SubscriptionId[];
    /**
     * Get the number of active subscriptions.
     */
    getSubscriptionCount(): number;
    protected subscribe<T>(method: string, unsubscribeMethod: string, params: unknown[], callback: (notification: T) => void): Promise<SubscriptionId>;
    protected unsubscribe(method: string, subscriptionId: SubscriptionId): Promise<boolean>;
    protected sendRequest<T>(method: string, params: unknown[]): Promise<T>;
    protected startMessageLoop(): void;
    protected handleMessage(message: WebsocketResponse): void;
    /**
     * Create an RpcWebsocketClient from an HTTP RPC URL.
     * Automatically converts http(s) to ws(s).
     */
    static fromHttpUrl(httpUrl: Url, config?: Omit<RpcWebsocketConfig, "url">): RpcWebsocketClient;
}

export declare interface RpcWebsocketConfig {
    /** WebSocket URL (wss://...) */
    url: string;
    /** Heartbeat interval in milliseconds. Default: 30000 */
    heartbeatInterval?: number;
    /** Whether to auto-reconnect on disconnection. Default: true */
    autoReconnect?: boolean;
    /** Delay between reconnection attempts in milliseconds. Default: 5000 */
    reconnectDelay?: number;
    /** Maximum reconnection attempts. Default: 10 */
    maxReconnectAttempts?: number;
}

export declare type RsaPrivateKey = Tagged<PrivateKey, "RsaPrivateKey">;

export { run }

export declare const RUNTIME_ENVIRONMENT: RuntimeEnvironment;

export declare const SAFARI: Browser<"Safari">;

export declare function safeStringify(obj: unknown, space?: number): string;

export declare function sameValueZero(x: unknown, y: unknown): boolean;

export declare type ScalarFromBytes<BytesArrayType> = ScalarFromBytesEncoding<ExtractEncoding<BytesArrayType>>;

export declare type ScalarFromBytesEncoding<Encoding extends BytesType> = StripEndian<Encoding> extends infer Core ? Core extends `bu${infer Width}` ? Width extends BigUnsignedWidth ? BigUnsignedScalarMap[Width] : never : Core extends `bi${infer Width}` ? Width extends BigSignedWidth ? BigSignedScalarMap[Width] : never : Core extends `u${infer Width}` ? Width extends UnsignedWidth ? UnsignedScalarMap[Width] : never : Core extends `i${infer Width}` ? Width extends SignedWidth ? SignedScalarMap[Width] : never : never : never;

declare type ScalarTagName<ScalarTag> = ScalarTag extends Tagged<unknown, infer Label> ? Label : never;

declare type ScalarTagToBytesEncoding = {
    uint8: "u8";
    uint16: "u16";
    uint24: "u24";
    uint32: "u32";
    int8: "i8";
    int16: "i16";
    int24: "i24";
    int32: "i32";
    buint8: "bu8";
    buint16: "bu16";
    buint24: "bu24";
    buint32: "bu32";
    buint64: "bu64";
    buint128: "bu128";
    buint256: "bu256";
    bint8: "bi8";
    bint16: "bi16";
    bint24: "bi24";
    bint32: "bi32";
    bint64: "bi64";
    bint128: "bi128";
    bint256: "bi256";
};

export declare type Schema = Record<string, SchemaField>;

export declare type SchemaField = SchemaFieldPlain | SchemaFieldArray | SchemaFieldVector | SchemaFieldHasSize | SchemaFieldNestedArray | SchemaFieldBytes | SchemaFieldRemainingBytes | SchemaFieldTaggedUnion | SchemaFieldBorshEnum | SchemaFieldSizedRawBytes;

export declare interface SchemaFieldArray {
    type: SchemaFieldType.Array;
    size: number;
    coder?: IsCodableStatic;
    codableType?: NestedSchemaField;
    optional?: boolean;
    /** Map of leading discriminator byte -> variant coder */
    variants?: Record<number, IsCodableStatic>;
}

/**
 * Borsh enum (Rust data enum) schema field.
 *
 * Distinct from `TaggedUnion`: a Borsh enum is variable length and its
 * discriminator is always the leading byte, so there is no `size` and no
 * `discriminatorOffset`. The tag selects a variant coder, and the variant's
 * fields follow immediately. A unit variant maps to a coder with an empty
 * schema and contributes only its tag byte.
 */
export declare interface SchemaFieldBorshEnum {
    type: SchemaFieldType.BorshEnum;
    /** Map of leading discriminator byte -> variant coder */
    variants: Record<number, IsCodableStatic>;
    optional?: boolean;
}

export declare interface SchemaFieldBytes {
    type: SchemaFieldType.Bytes;
    optional?: boolean;
    coder?: IsCodableStatic;
    codableType?: NestedSchemaField;
}

export declare interface SchemaFieldHasSize {
    type: SchemaFieldType.FixedBytes;
    size: number;
    optional?: boolean;
    coder?: IsCodableStatic;
    codableType?: NestedSchemaField;
}

export declare interface SchemaFieldNestedArray {
    type: SchemaFieldType.NestedArray;
    outerSize: number;
    innerSize: number;
    codableType: NestedSchemaField;
    optional?: boolean;
}

export declare interface SchemaFieldPlain {
    type: Exclude<SchemaFieldType, SchemaFieldType.FixedBytes | SchemaFieldType.Array | SchemaFieldType.Vector | SchemaFieldType.NestedArray | SchemaFieldType.Bytes | SchemaFieldType.RemainingBytes | SchemaFieldType.RemainingBytesNoTrim | SchemaFieldType.TaggedUnion | SchemaFieldType.BorshEnum | SchemaFieldType.SizedRawBytes>;
    optional?: boolean;
}

export declare interface SchemaFieldRemainingBytes {
    type: SchemaFieldType.RemainingBytes | SchemaFieldType.RemainingBytesNoTrim;
    optional?: boolean;
    coder?: IsCodableStatic;
    codableType?: NestedSchemaField;
}

/**
 * Sized raw bytes schema field for variable-length byte fields whose size
 * is determined by a previously-decoded sibling field.
 */
export declare interface SchemaFieldSizedRawBytes {
    type: SchemaFieldType.SizedRawBytes;
    /** Name of a previously-decoded numeric field containing the byte count */
    sizeField: string;
    optional?: boolean;
}

/**
 * Tagged union schema field for discriminated unions encoded in fixed-size byte arrays.
 *
 * The discriminator byte at `discriminatorOffset` determines which coder to use.
 */
export declare interface SchemaFieldTaggedUnion {
    type: SchemaFieldType.TaggedUnion;
    /** Total size of the opaque data in bytes */
    size: number;
    /** Byte offset where discriminator lives within the fixed-size data */
    discriminatorOffset: number;
    /** Map of discriminator value -> coder class */
    variants: Record<number, IsCodableStatic>;
    optional?: boolean;
}

export declare type SchemaFieldToType<T extends SchemaField> = T extends SchemaFieldArray ? T["coder"] extends new (...args: unknown[]) => infer U ? MakeOptionalMap<U[], T["optional"] extends true ? true : false> : never : T extends SchemaFieldRemainingBytes ? T["coder"] extends new (...args: unknown[]) => infer U ? MakeOptionalMap<U, T["optional"] extends true ? true : false> : MakeOptionalMap<Uint8Array, T["optional"] extends true ? true : false> : T extends SchemaFieldBorshEnum ? T["variants"] extends Record<number, infer CoderType> ? CoderType extends new (...args: unknown[]) => infer U ? MakeOptionalMap<U, T["optional"] extends true ? true : false> : never : never : T extends SchemaFieldTaggedUnion ? T["variants"] extends Map<number, infer CoderType> ? CoderType extends new (...args: unknown[]) => infer U ? MakeOptionalMap<U, T["optional"] extends true ? true : false> : never : never : T extends {
    type: infer K;
} ? K extends keyof BaseTypeMap ? MakeOptionalMap<BaseTypeMap[K], T["optional"] extends true ? true : false> : never : never;

export declare enum SchemaFieldType {
    FixedBytes = "FixedBytes",
    Address = "Address",
    U8 = "U8",// unsigned int as number
    U16 = "U16",// unsigned int as number
    U24 = "U24",// unsigned int as number
    U32 = "U32",// unsigned int as number
    BU8 = "BU8",// unsigned int as bigint
    BU16 = "BU16",// unsigned int as bigint
    BU24 = "BU24",// unsigned int as bigint
    BU32 = "BU32",// unsigned int as bigint
    U64 = "U64",// unsigned bigint
    U128 = "U128",// unsigned bigint
    I8 = "I8",// signed int as number
    I16 = "I16",// signed int as number
    I24 = "I24",// signed int as number
    I32 = "I32",// signed int as number
    BI8 = "BI8",// signed int as bigint
    BI16 = "BI16",// signed int as bigint
    BI24 = "BI24",// signed int as bigint
    BI32 = "BI32",// signed int as bigint
    I64 = "I64",// signed bigint
    I128 = "I128",// signed bigint
    Array = "Array",
    Boolean = "Boolean",
    Vector = "Vector",
    NestedArray = "NestedArray",
    String = "String",
    Bytes = "Bytes",
    RemainingBytes = "RemainingBytes",
    RemainingBytesNoTrim = "RemainingBytesNoTrim",
    TaggedUnion = "TaggedUnion",
    BorshEnum = "BorshEnum",
    SizedRawBytes = "SizedRawBytes"
}

export declare interface SchemaFieldVector {
    type: SchemaFieldType.Vector;
    codableType?: NestedSchemaField;
    coder?: IsCodableStatic;
    optional?: boolean;
    /** Map of leading discriminator byte -> variant coder */
    variants?: Record<number, IsCodableStatic>;
}

export declare type SchemaToType<T extends Schema> = {
    [K in keyof T]: SchemaFieldToType<T[K]>;
};

export declare type Scope<S extends string = string> = Tagged<Identity<S>, "Scope">;

export declare type ScreamingSnakeCase<T extends string = string> = Tagged<ValidScreamingSnakeCasePattern<T>, "ScreamingSnakeCase">;

export declare type ScryptHash = Tagged<Hash, "ScryptHash">;

export declare const SEC_FETCH_DEST_DOCUMENT: "document";

export declare const SEC_FETCH_DEST_EMPTY: "empty";

export declare const SEC_FETCH_DEST_SCRIPT: "script";

export declare const SEC_FETCH_MODE_CORS: "cors";

export declare const SEC_FETCH_MODE_NAVIGATE: "navigate";

export declare const SEC_FETCH_MODE_NO_CORS: "no-cors";

export declare const SEC_FETCH_MODE_SAME_ORIGIN: "same-origin";

export declare const SEC_FETCH_SITE_CROSS_SITE: "cross-site";

export declare const SEC_FETCH_SITE_NONE: "none";

export declare const SEC_FETCH_SITE_SAME_ORIGIN: "same-origin";

export declare const SEC_FETCH_SITE_SAME_SITE: "same-site";

export declare const SEC_FETCH_USER_ACTIVATED: "?1";

export declare interface Secp256k1RecoverableSignature {
    signature: Uint8Array;
    recoveryId: number;
}

export declare interface SendTransactionConfig {
    encoding?: "base58" | "base64";
    skipPreflight?: boolean;
    preflightCommitment?: SolanaCommitmentLevel;
    maxRetries?: number;
}

export declare const SENSEX: SensexName;

export declare type Sensex = EquityIndexId<"sensex">;

export declare type SensexName = EquityIndexName<"Sensex">;

export declare type SensexSymbol = EquityIndexSymbol<"BSE30">;

declare interface Serializable {
    toString(): string;
    [key: string]: any;
}

export declare interface SerializableObject extends Record<string, SerializableValue_2> {
}

declare type SerializablePrimitive = string | number | boolean | null | undefined;

export declare type SerializableValue = string | number | boolean;

declare type SerializableValue_2 = SerializablePrimitive | SerializableValue_2[] | SerializableObject | Serializable | JsonSerializable_2;

export declare type Serialized<T, S extends SerializableValue = string> = BrandedSerialized<T> & SerializedValue<S>;

export declare type SerializedValue<S extends SerializableValue = string> = Tagged<S, "SerializedValue">;

export declare const SerialzedBrand: unique symbol;

export declare const SERVER_FEATURE: FeatureName<"Server">;

export declare type Session<SessionName extends string = string> = Tagged<ObservedEvent, SessionName>;

/**
 * Set Compute Unit Limit instruction
 */
export declare class SetComputeUnitLimitInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    units: number;
    constructor(props: EncodableProps<SetComputeUnitLimitInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): SetComputeUnitLimitInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/**
 * Set Compute Unit Price instruction
 */
export declare class SetComputeUnitPriceInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    microLamports: bigint;
    constructor(props: EncodableProps<SetComputeUnitPriceInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): SetComputeUnitPriceInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare type SetIntervalTimer = ReturnType<typeof setInterval> | number;

export declare type SetTimeoutTimer = ReturnType<typeof setTimeout> | number;

export declare const SETTLEMENT_AUCTION_SESSION: SettlementAuctionSession;

export declare const SETTLEMENT_PAUSE_SESSION: SettlementPauseSession;

export declare type SettlementAuctionSession = MarketSession<"Settlement Auction">;

export declare type SettlementPauseSession = MarketSession<"Settlement Pause">;

export declare const SGX: SGX_2;

declare type SGX_2 = ExchangeName<"SGX">;

export declare type SGXId = ExchangeId<SGX_2>;

export declare type Sha256Hash = Tagged<Hash, "Sha256Hash">;

export declare type Sha512Hash = Tagged<Hash, "Sha512Hash">;

export declare type ShallowWritable<T> = T extends ReadonlyArray<infer U> ? U[] : T extends object ? {
    -readonly [K in keyof T]: T[K];
} : T;

export declare const SHANGHAI_COMPOSITE: ShanghaiCompositeName;

export declare const SHANGHAI_TIMEZONE: ShanghaiTimezone;

export declare type ShanghaiComposite = EquityIndexId<"shanghai-composite">;

export declare type ShanghaiCompositeName = EquityIndexName<"Shanghai Composite">;

export declare type ShanghaiCompositeSymbol = EquityIndexSymbol<"SHCOMP">;

export declare type ShanghaiTimezone = IANATimeZone<"Asia/Shanghai">;

export declare const SHENZHEN_COMPONENT: ShenzhenComponentName;

export declare type ShenzhenComponent = EquityIndexId<"shenzhen-component">;

export declare type ShenzhenComponentName = EquityIndexName<"Shenzhen Component">;

export declare type ShenzhenComponentSymbol = EquityIndexSymbol<"SZCOMP">;

export declare function shortenAddress(address: SolanaAddress, startLength?: number, endLength?: number, separator?: string): string;

export declare function shortenTransactionHash(txHash: SolanaTransactionSignature, startLength?: number, endLength?: number, separator?: string): string;

/**
 * Signature length in bytes (64 bytes for Ed25519)
 */
export declare const SIGNATURE_BYTE_LENGTH = 64;

export declare const SIGNATURE_INVALID_MESSAGE: "signature invalid";

export declare const SIGNATURE_LENGTH_IN_BYTES = 64;

export declare interface SignatureNotification {
    context: {
        slot: SolanaSlot;
    };
    value: {
        err: TransactionMeta["err"];
    } | "receivedSignature";
}

export declare interface SignaturePubkeyPair {
    signature: Uint8Array | null;
    publicKey: PublicKeyLike;
}

/** Signature */
export declare type SignatureSpan = ["signature", LabelValueRole];

export declare interface SignatureStatusResponse {
    slot: SolanaSlot;
    confirmations: number | null;
    err: unknown | null;
    confirmationStatus?: SolanaCommitmentLevel | null;
}

export declare interface SignatureSubscribeConfig {
    commitment?: SolanaCommitmentLevel;
    enableReceivedNotification?: boolean;
}

declare type SignedScalarMap = {
    "8": I8;
    "16": I16;
    "24": I24;
    "32": I32;
};

declare type SignedWidth = "8" | "16" | "24" | "32";

export declare interface SimulateTransactionConfig {
    encoding?: "base58" | "base64";
    commitment?: SolanaCommitmentLevel;
    sigVerify?: boolean;
    replaceRecentBlockhash?: boolean;
    accounts?: {
        encoding: "base64";
        addresses: SolanaAddressLike[];
    };
}

export declare interface SimulateTransactionResponse {
    context: {
        apiVersion: string;
        slot: SolanaSlot;
    };
    value: {
        err: null | {
            InstructionError: [number, {
                Custom: number;
            } | string];
        };
        logs: string[];
        accounts?: ({
            data: [Base64, string];
            executable: boolean;
            lamports: number;
            owner: string;
            rentEpoch: number;
        } | null)[];
        unitsConsumed?: number;
        returnData?: {
            programId: string;
            data: [string, string];
        };
    };
}

export declare const SINGAPORE_TIMEZONE: SingaporeTimezone;

export declare type SingaporeTimezone = IANATimeZone<"Asia/Singapore">;

export declare type SingleProperty<K extends string, V> = {
    [P in K]: V;
} & {
    [P in Exclude<string, K>]?: never;
};

export declare interface SlotNotification {
    parent: SolanaSlot;
    root: SolanaSlot;
    slot: SolanaSlot;
}

/** Slot */
export declare type SlotSpan = ["slot", LabelValueRole];

export declare interface SlotSubscribeConfig {
}

export declare interface SlotUpdateNotification {
    slot: SolanaSlot;
    timestamp: number;
    type: "firstShredReceived" | "completed" | "createdBank" | "frozen" | "dead" | "optimisticConfirmation" | "root";
}

export declare type Slug<S extends string = string> = Tagged<Identity<S>, "Slug">;

export declare type SnakeCase<T extends string = string> = Tagged<ValidSnakeCasePattern<T>, "SnakeCase">;

export declare const SOCKET_HANG_UP_MESSAGE: "socket hang up";

export declare const SOCKET_TIMEOUT_MESSAGE: "socket timeout";

export declare type SocketFilepath<S extends string = string> = Tagged<S, "SocketFilepath">;

export declare const SOLANA: Solana;

export declare type Solana = AssetId<"solana">;

export declare const SOLANA_EXPLORER_URL: Url<"https://explorer.solana.com/">;

export declare const SOLANA_NAME: SolanaName;

export declare const SOLANA_NAME_SERVICE: SolanaNameService;

export declare const SOLANA_NAME_SERVICE_URL: Url<"https://www.sns.id/">;

export declare const SOLANA_SYMBOL: SolanaSymbol;

export declare type SolanaAddress<S extends string = string> = Tagged<Base58<S, 32, 44>, "SolanaAddress">;

export declare type SolanaAddressInfo<T extends string, Metadata extends SolanaAddressMetadata | SolanaTokenMetadata<TokenId<string>>> = Tagged<SolanaAddress<T>, "SolanaAddressInfo", Metadata>;

export declare type SolanaAddressLike = PublicKeyLike | SolanaAddress | SolanaKitAddress;

export declare type SolanaAddressMetadata = {
    systemProgram: true;
    name: string;
    description: string;
} | {
    protocol: SolanaProtocolName<string> | ExchangeName;
    name: string;
    description: string;
};

export declare function solanaAddressToInviteCode(address: SolanaAddress): SolomonInviteCode;

export declare function solanaAddressToPublicKeyLike<DerivedPublicKeyType extends PublicKeyLike = PublicKey>(address: DerivedPublicKeyType | string | Uint8Array, referencePublicKey?: DerivedPublicKeyType | string): DerivedPublicKeyType;

export declare type SolanaBase58AddressString = SolanaAddress | SolanaKitAddress;

export declare type SolanaBlockhash<S extends string = string> = Tagged<Base58<S, 32, 44>, "SolanaBlockhash">;

export declare type SolanaCommitmentLevel = "processed" | "confirmed" | "finalized";

export declare function solanaExplorerLink(identifier: SolanaTransactionSignature | SolanaAddress | number, linkType: ExplorerLinkType, explorer?: SolanaExplorer, network?: SolanaNetwork): Url;

export declare type SolanaKitAddress<T extends string = string> = T & {
    readonly "__brand:@solana/kit": "Address";
} & {
    readonly "__stringEncoding:@solana/kit": "base58";
};

export declare type SolanaLogSubType = "instruction" | "error";

export declare type SolanaLogType = BaseSolanaLogType | SyntheticLogType;

export declare type SolanaName = AssetName<"Solana">;

export declare type SolanaNameService = SolanaProtocolName<"Solana Name Service">;

export declare type SolanaNameServiceId = SolanaProtocolId<SolanaNameService>;

export declare type SolanaProtocolId<T extends string> = BlockchainProtocolId<T>;

export declare type SolanaProtocolName<T extends string> = BlockchainProtocolName<T>;

export declare type SolanaRpcEncoding = "json" | "jsonParsed" | "base58" | "base64";

export declare type SolanaSlot = Tagged<number, "SolanaSlot">;

export declare type SolanaSymbol = AssetSymbol<"SOL">;

export declare type SolanaToken<T extends TokenId<string>> = Tagged<NetworkAsset<T>, "SolanaToken">;

export declare type SolanaTokenMetadata<SolanaTokenId extends TokenId<string>> = {
    tokenId: SolanaToken<SolanaTokenId>;
    decimals: number;
    program: "Token" | "Token 2022";
};

export declare type SolanaTransactionSignature<S extends string = string> = Tagged<Base58<S, 64, 88>, "SolanaTransactionSignature">;

export declare type SolanaTransactionVersion = "legacy" | number;

export declare const SOLAYER_USD: SolayerUsd;

export declare const SOLAYER_USD_NAME: SolayerUsdName;

export declare const SOLAYER_USD_SYMBOL: SolayerUsdSymbol;

export declare type SolayerUsd = TokenId<"solayer-usd">;

export declare type SolayerUsdName = TokenName<"Solayer USD">;

export declare type SolayerUsdSolana = SolanaAddressInfo<"susdabGDNbhrnCa6ncrYo81u4s9GM8ecK2UwMyZiq4X", {
    tokenId: SolanaToken<StandXDusd>;
    decimals: 6;
    program: "Token 2022";
}>;

export declare type SolayerUsdSymbol = TokenSymbol<"sUSD">;

export declare const SOLOMON_API_ADMIN_PREFIX: "/admin";

export declare const SOLOMON_API_DEFAULT_AUTH_REQUIRED_PATHS: readonly ["auth", "admin"];

export declare const SOLOMON_API_DOMAIN: Domain<"solomonlabs.io">;

export declare const SOLOMON_API_PREFIX: "/api";

export declare const SOLOMON_API_SCOPE_INTERNAL: "internal";

export declare const SOLOMON_API_SCOPE_PUBLIC: "public";

export declare const SOLOMON_DATA_API_DOMAIN: Domain<"data.solomonlabs.io">;

export declare const SOLOMON_DATA_API_URL: Url<"https://data.solomonlabs.io">;

export declare const SOLOMON_DEFAULT_API_RATE_LIMIT_DELAY: 100;

export declare const SOLOMON_DEFAULT_API_TIMEOUT: 30000;

export declare const SOLOMON_DOCUMENTATION_INTERNAL_PREFIX: "/internal/documentation";

export declare const SOLOMON_DOCUMENTATION_PREFIX: "/documentation";

export declare const SOLOMON_LABS: SolomonLabs;

export declare const SOLOMON_LABS_CACHE_INDEX_KEY: "__slcind";

export declare const SOLOMON_LABS_CACHE_KEY: "__slc_";

export declare const SOLOMON_LABS_CLIENT_SLUG: Slug<"rest-client">;

export declare const SOLOMON_LABS_CLIENT_VERSION: Version<"1.0.0">;

export declare const SOLOMON_LABS_DATABASE_NAME_PREFIX: "sl_db";

export declare const SOLOMON_LABS_DEFAULT_DATABASE_NAME: "sl_db_dflt";

export declare const SOLOMON_LABS_DEV: Identity<"dev">;

export declare const SOLOMON_LABS_DOMAIN: Domain<"solomonlabs.org">;

export declare const SOLOMON_LABS_IDENTIFIER: "__sl";

export declare const SOLOMON_LABS_KEY_VALUE_STORE: "kvs";

export declare const SOLOMON_LABS_PUBLIC_KEYS_STORE: "pks";

export declare const SOLOMON_LABS_SCOPE: Scope<"SolomonLabs">;

export declare const SOLOMON_LABS_SLUG: Slug<"solomon-labs">;

export declare const SOLOMON_LABS_URL: Url<"https://solomonlabs.org/">;

export declare const SOLOMON_REWARDS_API_DOMAIN: Domain<"rewards.solomonlabs.io">;

export declare const SOLOMON_REWARDS_API_URL: Url<"https://rewards.solomonlabs.io">;

export declare const SOLOMON_SCHEMA_COMPONENT_DEFINITION: "components#/definitions";

export declare const SOLOMON_SCHEMA_COMPONENTS_URI: Url<`#/components/schemas`>;

export declare const SOLOMON_SCHEMA_NAMESPACE: "components";

export declare const SOLOMON_STAKED_USDV: SolomonStakedUsdv;

export declare const SOLOMON_STAKED_USDV_NAME: SolomonStakedUsdvName;

export declare const SOLOMON_STAKED_USDV_SYMBOL: SolomonStakedUsdvSymbol;

export declare const SOLOMON_USDV: SolomonUsdv;

export declare const SOLOMON_USDV_NAME: SolomonUsdvName;

export declare const SOLOMON_USDV_SYMBOL: SolomonUsdvSymbol;

export declare type SolomonApiScope = "public" | "internal";

export declare type SolomonInviteCode = Tagged<string, "SolomonInviteCode">;

export declare type SolomonLabs = SolanaProtocolName<"Solomon Labs">;

/**
 * Base class for all Solomon Labs errors.
 */
export declare abstract class SolomonLabsError extends Error {
    readonly originalError?: unknown | undefined;
    /** High-level domain bucket e.g. "HTTP", "RPC", "CONNECTION" */
    abstract readonly category: string;
    /** Fine-grained identifier e.g. "TIMEOUT", "INSUFFICIENT_DATA" */
    abstract readonly type: string;
    constructor(message: string, originalError?: unknown | undefined);
    toString(): string;
    toJSON(): Record<string, unknown>;
    private serializeValue;
    private getRootCause;
    private findRootError;
    /**
     * Clone this error while ensuring the clone only holds WeakRef references
     * to all object/function values stored on the instance.
     */
    clone(): this;
}

export declare type SolomonLabsId = SolanaProtocolId<SolomonLabs>;

export declare type SolomonStakedUsdv = TokenId<"solomon-staked-usdv">;

export declare type SolomonStakedUsdvName = TokenName<"Solomon sUSDv">;

export declare type SolomonStakedUsdvSymbol = TokenSymbol<"sUSDv">;

export declare type SolomonUsdv = TokenId<"solomon-usdv">;

export declare type SolomonUsdvName = TokenName<"Solomon USDv">;

export declare type SolomonUsdvSolana = SolanaAddressInfo<"Ex5DaKYMCN6QWFA4n67TmMwsH8MJV68RX6YXTmVM532C", {
    tokenId: SolanaToken<SolomonUsdv>;
    decimals: 9;
    program: "Token";
}>;

export declare type SolomonUsdvSymbol = TokenSymbol<"USDv">;

export declare const SOLSCAN_URL: Url<"https://solscan.io/">;

export declare type SomeKeys<T, ToOmit extends string> = readonly (keyof Omit<T, ToOmit>)[] & {
    length: UnionToTuple<keyof Omit<T, ToOmit>>["length"];
};

export declare type SortOrder = "ASC" | "DESC";

export declare const SP500: SP500Name;

export declare type SP500Name = EquityIndexName<"S&P 500">;

export declare type SP500Symbol = EquityIndexSymbol<"SPX">;

export declare const SP_ASX200: SPASX200Name;

/** A space, to use between SPAN. */
export declare type SpaceOp = 2;

/** A span definition: [tag] or [tag, role] or [tag, undefined] */
declare type SpanDef = [string] | [string, string] | [string, undefined];

/**
 * Opcode discriminator for the display IR.
 */
/** A textual span (leaf node). */
export declare type SpanOp = 0;

/**
 * Extract role from span def:
 * - ["tag"] (length 1) → string (any role allowed)
 * - ["tag", Role] where Role is string → Role (only that role)
 * - ["tag", undefined] → undefined (no role allowed)
 */
declare type SpanRole<Span, Tag extends string> = Span extends [Tag, infer Role] ? Role : Span extends [Tag] ? string : never;

/** Extract tag from span def */
declare type SpanTag<Span> = Span extends [infer Tag, ...any[]] ? Tag : never;

export declare type SPASX200 = EquityIndexId<"sp-asx-200">;

export declare type SPASX200Name = EquityIndexName<"S&P/ASX 200">;

export declare type SPASX200Symbol = EquityIndexSymbol<"XJO">;

export declare type SpotTradingPair<TBase extends string = string, TQuote extends string = string> = Tagged<TradingPair<TBase, TQuote>, "SpotTradingPair">;

export declare type SpreadArgument<T> = T extends any[] ? "Use ...args instead of args: T[]" : readonly T[];

export declare const SSE: SSE_2;

declare type SSE_2 = ExchangeName<"SSE">;

export declare type SSEId = ExchangeId<SSE_2>;

export declare const STAKE_PROGRAM_ID = "Stake11111111111111111111111111111111111111";

export declare const STAKE_PROGRAM_ID_STRING: SolanaAddress<"HSnn7bDvkZSEwujZDPtUcdo9KL7Conycgmy8m6mBFD5">;

export declare const STAKE_PROGRAM_LOCKED_USDV_ADDRESS_STRING: SolanaAddress<"DDVcfRt7XUxWAX6U4uHvbkj97uXCUA7w8aHsg4QPyyvD">;

export declare const STAKE_PROGRAM_STAKED_USDV_ADRESS_STRING: SolanaAddress<"BsPrkRjar8ktWagbcxsEzSBSpVnaj47nasjpFHWp1VMF">;

export declare const STANDX_DUSD: StandXDusd;

export declare const STANDX_DUSD_NAME: StandXDusdName;

export declare const STANDX_DUSD_SYMBOL: StandXDusdSymbol;

export declare type StandXDusd = TokenId<"standx-dusd">;

export declare type StandXDusdName = TokenName<"StandX DUSD">;

export declare type StandXDusdSolana = SolanaAddressInfo<"DUSDt4AeLZHWYmcXnVGYdgAzjtzU5mXUVnTMdnSzAttM", {
    tokenId: SolanaToken<StandXDusd>;
    decimals: 6;
    program: "Token 2022";
}>;

export declare type StandXDusdSymbol = TokenSymbol<"DUSD">;

/** For highlighting */
export declare type StartEndRole = "start" | "end";

export declare const STATIC_ADDRESSES_CACHE: {
    readonly [base58Key: SolanaAddress]: Uint8Array;
};

/** Common union that might match a role, or kind or icon variants */
export declare type StatusType = "success" | "ok" | "failure" | "error" | "warning";

export declare interface StorageInterface {
    length: number;
    getItem(key: string): string | null;
    setItem(key: string, value: string): void;
    key(index: number): string | null;
    removeItem(ley: string): void;
    clear(): void;
}

export declare const STRAITS_TIMES_INDEX: StraitsTimesIndexName;

export declare type StraitsTimesIndex = EquityIndexId<"straits-times-index">;

export declare type StraitsTimesIndexName = EquityIndexName<"Straits Times Index">;

export declare type StraitsTimesIndexSymbol = EquityIndexSymbol<"STI">;

export declare type StrArray = string[] | readonly string[];

export declare type StringMap<KLiteral extends string = string, VLiteral extends string = string> = Record<KLiteral, VLiteral>;

declare type StripEndian<Encoding extends BytesType> = Encoding extends `${infer Core}-be` ? Core : Encoding;

/**
 * ---------------------------------------------------------------------------
 * Encoding -> byte length
 * ---------------------------------------------------------------------------
 */
declare type StripEndian_2<Encoding extends BytesType> = Encoding extends `${infer Core}-be` ? Core : Encoding;

declare type StripNullish<T> = Exclude<T, null | undefined>;

export declare type StripRelations<EntityInterface, EntityClass extends EntityInterface = EntityInterface> = {
    [PropertyKey in keyof EntityClass as EntityClass[PropertyKey] extends Relation<EntityInterface, EntityClass> ? never : PropertyKey]: EntityClass[PropertyKey] extends Relation<EntityInterface, EntityClass> ? never : EntityClass[PropertyKey];
};

export declare type StrListArg = string | StrArray;

export declare type StrTupleArray = [string, string][] | readonly (readonly [string, string])[];

/**
 * High-level helper that allows tagging a struct layout with left/right
 * padding in bytes. Offsets can be derived from `byteLength` plus the
 * padding information if needed.
 */
export declare type StructToWire<Struct, LeftPaddingBytes extends number = 0, RightPaddingBytes extends number = 0> = {
    readonly leftPaddingBytes: LeftPaddingBytes;
    readonly rightPaddingBytes: RightPaddingBytes;
    readonly fields: StructWireLayout<Struct>;
};

/**
 * Map a whole struct type to wire descriptors per field.
 *
 * This does not assign concrete offsets; it just ensures that every field
 * in the struct has a valid wire mapping and exposes the encoding and
 * byteLength for each.
 */
export declare type StructWireLayout<Struct> = {
    readonly [Key in keyof Struct]: WireFieldForKey<Struct, Key>;
};

export declare interface Subscription<T> {
    id: SubscriptionId;
    method: string;
    unsubscribeMethod: string;
    callback: (notification: T) => void;
}

export declare type SubscriptionId = number;

/** Some sub typing */
export declare type SubtypeSpan = ["subtype", LabelValueRole];

export declare interface SuccessLog extends BaseLog {
    type: "success";
    programId: PublicKey_2;
}

export declare type Summary = Tagged<Text, "Summary">;

export declare type Surname = Tagged<Identity<string>, "Surname">;

export declare const SWISS_MARKET_INDEX: SwissMarketIndexName;

export declare type SwissMarketIndex = EquityIndexId<"swiss-market-index">;

export declare type SwissMarketIndexName = EquityIndexName<"Swiss Market Index">;

export declare type SwissMarketIndexSymbol = EquityIndexSymbol<"SMI">;

export declare const SYDNEY_TIMEZONE: SydneyTimezone;

export declare type SydneyTimezone = IANATimeZone<"Australia/Sydney">;

export declare type SyncFunction<T = any> = Function & ((...args: any[]) => Exclude<T, Promise<any>>);

/** 17 = SyncNative {} (for wrapped SOL accounts) */
export declare class SyncNativeInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    constructor(props: EncodableProps<SyncNativeInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): SyncNativeInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare type SyntheticLogType = "anchorEvent" | "truncated" | "orphan";

/**
 * Solana System Program ID
 */
export declare const SYSTEM_PROGRAM_ID: SolanaAddress;

export declare const SYSTEM_PROGRAM_ID_STRING: SystemProgram;

export declare type SystemProgram = SolanaAddressInfo<"11111111111111111111111111111111", {
    systemProgram: true;
    name: "System Program";
    description: "System Program for low-level instructions";
}>;

/**
 * 2 = Transfer { lamports: u64 }
 */
export declare class SystemTransferInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    from: SolanaAddressLike;
    to: SolanaAddressLike;
    lamports: bigint | number | string;
    constructor(props: EncodableProps<SystemTransferInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): SystemTransferInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const SYSVAR_RECENT_BLOCKHASHES_ADDRESS_STRING: SysvarRecentBlockhashes;

export declare const SYSVAR_RENT_PROGRAM_ID_STRING: SysvarRentProgram;

export declare type SysvarRecentBlockhashes = SolanaAddressInfo<"SysvarRecentB1ockHashes11111111111111111111", {
    systemProgram: true;
    name: "Sysvar Recent Blockhashes";
    description: 'This sysvar is deprecated since "1.9.0"';
}>;

export declare type SysvarRentProgram = SolanaAddressInfo<"SysvarRent111111111111111111111111111111111", {
    systemProgram: true;
    name: "Sysvar Rent Program";
    description: "Calculate how much rent to burn from the collected rent";
}>;

export declare const SZSE: SZSE_2;

declare type SZSE_2 = ExchangeName<"SZSE">;

export declare type SZSEId = ExchangeId<SZSE_2>;

export declare type TableName<S extends string = string> = Tagged<S, "TableName">;

export declare type Tag<Type, Token extends PropertyKey, TagMetadata> = TagContainer<{
    [K in Token]: TagMetadata;
}, Type>;

declare const tag: unique symbol;

export declare type TagContainer<Token, Inner> = {
    readonly [tag]: Token;
    readonly [tagType]: Inner;
};

export declare type Tagged<Type, TagName extends PropertyKey, TagMetadata = never> = Type & Tag<ExtractBaseType<Type>, TagName, TagMetadata>;

export declare type TaggedFlat = Tagged<string, PropertyKey, never>;

declare const tagType: unique symbol;

declare type TagTypeStringToId<T extends TaggedFlat> = ToCleaned<ExtractStringFromTagged<T>>;

export declare type TemplatedDescription = Description & TemplatedText;

export declare type TemplatedExplanation = Explanation & TemplatedText;

export declare type TemplatedSummary = Summary & TemplatedText;

export declare type TemplatedText = Tagged<Text, "Templated">;

export declare type TemplatedTitle = Title & TemplatedText;

export declare const TEN_MINUTES_MS: DurationMs<600000>;

export declare const TEN_MINUTES_S: DurationS<600>;

export declare const TEN_SECOND_MS: DurationMs<10000>;

export declare const TEN_SECOND_S: DurationS<10>;

export declare const testConfig: {
    timeout: number;
    parallel: boolean;
    coverage: {
        enabled: boolean;
        threshold: {
            statements: number;
            branches: number;
            functions: number;
            lines: number;
        };
    };
    env: {
        NODE_ENV: string;
        LOG_LEVEL: string;
    };
};

export declare const TestFixtures: {
    validAddresses: string[];
    invalidAddresses: string[];
    tokens: {
        SOL: string;
        USDC: string;
        USDT: string;
    };
    mockTokenAccount: {
        mint: string;
        owner: string;
        amount: string;
        decimals: number;
        state: number;
    };
    mockWhirlpool: {
        tokenMintA: string;
        tokenMintB: string;
        tickSpacing: number;
        liquidity: string;
        sqrtPrice: string;
        feeRate: number;
    };
};

export declare class TestRunner {
    testDir: string;
    passed: number;
    failed: number;
    skipped: number;
    constructor();
    findTestFiles(dir?: string): Promise<string[]>;
    runTests(pattern?: string | null): Promise<void>;
}

export declare class TestUtils {
    static readonly BASE58_ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    private static _PublicKeyCtor;
    /** Synchronous, throws if package truly isn’t present */
    private static get PublicKey();
    static createMockAccountData(size: number): Uint8Array;
    static createMockAddress(): SolanaAddress;
    static createValidBase58(): SolanaAddress;
    static createInvalidBase58(): SolanaAddress;
    static delay(ms: number): Promise<void>;
    static measureTime<T>(fn: () => Promise<T> | T): Promise<{
        result: T;
        timeMs: number;
    }>;
    static encodeBase58(bytes: Uint8Array): Base58;
    static randomValidAddress(): SolanaAddress;
    static randomValidPublicKey(): any;
    static randomValidAddressBytes(): Uint8Array;
    static randomValidProgramAddress(): SolanaAddress;
    static randomOffCurveAddress(): SolanaAddress;
    static assertBigIntApproxEqual(actual: bigint, expected: bigint, tolerance?: bigint, message?: string): void;
    static assertThrowsInstanceOf<T extends Error>(fn: () => void, errorClass: new (...args: any[]) => T, message?: string): void;
    static assertThrowsInstanceOfAsync<T extends Error>(fn: () => Promise<void | unknown | string | null | Function>, // eslint-disable-line
    errorClass: new (...args: any[]) => T, message?: string): Promise<void>;
    static assertKeyUndefinedOrNull(obj: unknown, key: string): void;
    static replaceTaskController(classInstance: any, taskProperty: string, taskMethodName: string): void;
    /**
     * Return the dirname of the immediate caller (one frame up).
     * Works in both CJS and ESM without using __dirname or import.meta.url.
     */
    static getImmediateCallerDirname(): string;
}

export declare const TETHER: Tether;

export declare type Tether = TokenId<"tether">;

export declare const TETHER_NAME: TetherName;

export declare const TETHER_SYMBOL: TetherSymbol;

export declare type TetherName = TokenName<"Tether">;

export declare type TetherSolana = SolanaAddressInfo<"Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB", {
    tokenId: SolanaToken<Tether>;
    decimals: 6;
    program: "Token";
}>;

export declare type TetherSymbol = TokenSymbol<"USDT">;

export declare type Text = Tagged<string, "Text">;

export declare type TextMimeType = "text/plain" | "text/html" | "text/css" | "text/csv" | "text/xml" | "text/javascript";

/** Catch-all for the rest */
export declare type TextSpan = ["text"];

export declare const THANKSGIVING: Thanksgiving;

export declare type Thanksgiving = Holiday<"Thanksgiving">;

/** 11 = ThawAccount {} */
export declare class ThawAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    account: SolanaAddressLike;
    mint: SolanaAddressLike;
    authority: SolanaAddressLike;
    constructor(props: EncodableProps<ThawAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): ThawAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare const THIRTY_MINUTES_MS: DurationMs<1800000>;

export declare const THIRTY_MINUTES_S: DurationS<1800>;

export declare const THIRTY_SECOND_MS: DurationMs<30000>;

export declare const THIRTY_SECOND_S: DurationS<30>;

export declare const THREE_MINUTES_MS: DurationMs<180000>;

export declare const THREE_MINUTES_S: DurationS<180>;

export declare const THROTTLED_MESSAGE: "throttled";

export declare type TickerSymbol<TBase extends string = string, TQuote extends string = string> = Tagged<SpotTradingPair<TBase, TQuote> | LinearTradingPair<TBase, TQuote> | InverseTradingPair<TBase, TQuote>, "TickerSymbol">;

export declare type Time = Tagged<DateOrTime, "Time">;

export declare const TIMED_OUT_MESSAGE: "timed out";

export declare const TIMEOUT_ERROR_MESSAGE: "timeout";

export declare type Timestamp<T extends number | bigint = number> = Tagged<T, "Timestamp">;

export declare type TimestampMs<T extends number = number> = Tagged<Timestamp<T>, "TimestampMs">;

export declare type TimestampNs<T extends bigint = bigint> = Tagged<Timestamp<T>, "TimestampNs">;

export declare type TimestampS<T extends number = number> = Tagged<Timestamp<T>, "TimestampS">;

export declare type TimestampUs<T extends number = number> = Tagged<Timestamp<T>, "TimestampUs">;

export declare type TimeUnits = "Millisecond" | "Second" | "Minute" | "Hour" | "Day" | "Week" | "Month" | "Year" | "Decade" | "Century" | "Millennium";

export declare type TimeZoneOffet<Offset extends string = string> = Tagged<Offset, "TimeZoneOffset">;

export declare type Title = Tagged<Text, "Title">;

export declare type TitleCase<T extends string = string> = Tagged<ValidTitleCasePattern<T>, "TitleCase">;

export declare function toCamelCase(...segments: ReadonlyArray<StrListArg>): CamelCase;

export declare function toCamelCaseSafe(name: string): CamelCase;

declare type ToCleaned<S extends string> = S extends `${infer F}${infer R}` ? `${CleanChar<F>}${ToCleaned<R>}` : "";

export declare function toKebabCase(...segments: ReadonlyArray<StrListArg>): KebabCase;

export declare enum Token2022AccountType {
    Uninitialized = 0,
    Mint = 1,
    Account = 2
}

export declare class Token2022ConfidentialMintBurn implements IsCodable {
    confidentialSupply: Uint8Array;
    decryptableSupply: Uint8Array;
    supplyElgamalPubkey: Uint8Array;
    pendingBurn: Uint8Array;
    constructor(properties: EncodableProps<Token2022ConfidentialMintBurn>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022ConfidentialMintBurn;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022ConfidentialTransferAccount implements IsCodable {
    approved: boolean;
    elgamalPubkey: Uint8Array;
    pendingBalanceLo: Uint8Array;
    pendingBalanceHi: Uint8Array;
    availableBalance: Uint8Array;
    decryptableAvailableBalance: Uint8Array;
    allowConfidentialCredits: boolean;
    allowNonConfidentialCredits: boolean;
    pendingBalanceCreditCounter: bigint;
    maximumPendingBalanceCreditCounter: bigint;
    expectedPendingBalanceCreditCounter: bigint;
    actualPendingBalanceCreditCounter: bigint;
    constructor(properties: EncodableProps<Token2022ConfidentialTransferAccount>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022ConfidentialTransferAccount;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022ConfidentialTransferFeeAmount implements IsCodable {
    withheldAmount: Uint8Array;
    constructor(properties: EncodableProps<Token2022ConfidentialTransferFeeAmount>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022ConfidentialTransferFeeAmount;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022ConfidentialTransferFeeConfig implements IsCodable {
    authority: PublicKeyLike;
    withdrawWithheldAuthorityElgamalPubkey: Uint8Array;
    harvestToMintEnabled: boolean;
    withheldAmount: Uint8Array;
    constructor(properties: EncodableProps<Token2022ConfidentialTransferFeeConfig>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022ConfidentialTransferFeeConfig;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022ConfidentialTransferMint implements IsCodable {
    authority: PublicKeyLike;
    autoApproveNewAccounts: boolean;
    auditorElgamalPubkey: Uint8Array;
    constructor(properties: EncodableProps<Token2022ConfidentialTransferMint>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022ConfidentialTransferMint;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022CpiGuard implements IsCodable {
    lockCpi: boolean;
    constructor(properties: EncodableProps<Token2022CpiGuard>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022CpiGuard;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022DefaultAccountState implements IsCodable {
    state: number;
    constructor(properties: EncodableProps<Token2022DefaultAccountState>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022DefaultAccountState;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare type Token2022ExtensionData = Token2022TransferFeeConfig | Token2022TransferFeeAmount | Token2022MintCloseAuthority | Token2022ConfidentialTransferMint | Token2022ConfidentialTransferAccount | Token2022DefaultAccountState | Token2022ImmutableOwner | Token2022MemoTransfer | Token2022NonTransferable | Token2022InterestBearingConfig | Token2022CpiGuard | Token2022PermanentDelegate | Token2022NonTransferableAccount | Token2022TransferHook | Token2022TransferHookAccount | Token2022ConfidentialTransferFeeConfig | Token2022ConfidentialTransferFeeAmount | Token2022MetadataPointer | Token2022TokenMetadata | Token2022GroupPointer | Token2022TokenGroup | Token2022GroupMemberPointer | Token2022TokenGroupMember | Token2022ConfidentialMintBurn | Token2022ScaledUiAmountConfig | Token2022PausableConfig | Token2022PausableAccount | Token2022PermissionedBurnConfig;

export declare enum Token2022ExtensionType {
    Uninitialized = 0,
    TransferFeeConfig = 1,
    TransferFeeAmount = 2,
    MintCloseAuthority = 3,
    ConfidentialTransferMint = 4,
    ConfidentialTransferAccount = 5,
    DefaultAccountState = 6,
    ImmutableOwner = 7,
    MemoTransfer = 8,
    NonTransferable = 9,
    InterestBearingConfig = 10,
    CpiGuard = 11,
    PermanentDelegate = 12,
    NonTransferableAccount = 13,
    TransferHook = 14,
    TransferHookAccount = 15,
    ConfidentialTransferFeeConfig = 16,
    ConfidentialTransferFeeAmount = 17,
    MetadataPointer = 18,
    TokenMetadata = 19,
    GroupPointer = 20,
    TokenGroup = 21,
    GroupMemberPointer = 22,
    TokenGroupMember = 23,
    ConfidentialMintBurn = 24,
    ScaledUiAmount = 25,
    Pausable = 26,
    PausableAccount = 27,
    PermissionedBurn = 28
}

export declare class Token2022GroupMemberPointer implements IsCodable {
    authority: PublicKeyLike;
    memberAddress: PublicKeyLike;
    constructor(properties: EncodableProps<Token2022GroupMemberPointer>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022GroupMemberPointer;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022GroupPointer implements IsCodable {
    authority: PublicKeyLike;
    groupAddress: PublicKeyLike;
    constructor(properties: EncodableProps<Token2022GroupPointer>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022GroupPointer;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022ImmutableOwner implements IsCodable {
    constructor(_properties: Token2022ImmutableOwner);
    static decode(data: string | Uint8Array | Buffer | null): Token2022ImmutableOwner;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022InterestBearingConfig implements IsCodable {
    rateAuthority: PublicKeyLike;
    initializationTimestamp: bigint;
    preUpdateAverageRate: number;
    lastUpdateTimestamp: bigint;
    currentRate: number;
    constructor(properties: EncodableProps<Token2022InterestBearingConfig>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022InterestBearingConfig;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022MemoTransfer implements IsCodable {
    requireIncomingTransferMemos: boolean;
    constructor(properties: EncodableProps<Token2022MemoTransfer>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022MemoTransfer;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022MetadataPointer implements IsCodable {
    authority: PublicKeyLike;
    metadataAddress: PublicKeyLike;
    constructor(properties: EncodableProps<Token2022MetadataPointer>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022MetadataPointer;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022MintCloseAuthority implements IsCodable {
    closeAuthority: PublicKeyLike;
    constructor(properties: EncodableProps<Token2022MintCloseAuthority>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022MintCloseAuthority;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022NonTransferable implements IsCodable {
    constructor(_properties: Token2022NonTransferable);
    static decode(data: string | Uint8Array | Buffer | null): Token2022NonTransferable;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022NonTransferableAccount implements IsCodable {
    constructor(_properties: Token2022NonTransferableAccount);
    static decode(data: string | Uint8Array | Buffer | null): Token2022NonTransferableAccount;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022PausableAccount implements IsCodable {
    constructor(_properties: Token2022PausableAccount);
    static decode(data: string | Uint8Array | Buffer | null): Token2022PausableAccount;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022PausableConfig implements IsCodable {
    authority: PublicKeyLike;
    paused: boolean;
    constructor(properties: EncodableProps<Token2022PausableConfig>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022PausableConfig;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022PermanentDelegate implements IsCodable {
    delegate: PublicKeyLike;
    constructor(properties: EncodableProps<Token2022PermanentDelegate>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022PermanentDelegate;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022PermissionedBurnConfig implements IsCodable {
    authority: PublicKeyLike;
    constructor(properties: EncodableProps<Token2022PermissionedBurnConfig>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022PermissionedBurnConfig;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare type Token2022Program = SolanaAddressInfo<"TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", {
    systemProgram: true;
    name: "Token 2022 Program";
    description: "2022 SPL Program";
}>;

export declare class Token2022ScaledUiAmountConfig implements IsCodable {
    authority: PublicKeyLike;
    multiplierBytes: Uint8Array;
    multiplier: number;
    newMultiplierEffectiveTimestamp: bigint;
    newMultiplierBytes: Uint8Array;
    newMultiplier: number;
    constructor(properties: EncodableProps<Token2022ScaledUiAmountConfig>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022ScaledUiAmountConfig;
    encode(): Uint8Array;
    static getSchema(): Schema;
    private static bytesToFloat64;
}

export declare class Token2022TlvData implements IsCodable {
    data: Uint8Array;
    entries: Token2022TlvEntry[];
    constructor(properties: EncodableProps<Token2022TlvData>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TlvData;
    encode(): Uint8Array;
    static getSchema(): Schema;
    private static decodeEntries;
}

export declare class Token2022TlvEntry implements IsCodable {
    extensionType: number;
    length: number;
    data: Uint8Array;
    extensionTypeName: string;
    decoded: Token2022ExtensionData | null;
    constructor(properties: EncodableProps<Token2022TlvEntry>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TlvEntry;
    encode(): Uint8Array;
    static getSchema(): Schema;
    private static extensionTypeNameFromNumber;
    private static decodeExtensionData;
}

export declare class Token2022TokenGroup implements IsCodable {
    updateAuthority: PublicKeyLike;
    mint: PublicKeyLike;
    size: bigint;
    maxSize: bigint;
    constructor(properties: EncodableProps<Token2022TokenGroup>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TokenGroup;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022TokenGroupMember implements IsCodable {
    mint: PublicKeyLike;
    group: PublicKeyLike;
    memberNumber: bigint;
    constructor(properties: EncodableProps<Token2022TokenGroupMember>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TokenGroupMember;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022TokenMetadata implements IsCodable {
    updateAuthority: PublicKeyLike;
    mint: PublicKeyLike;
    name: string;
    symbol: string;
    uri: string;
    additionalMetadata: Token2022TokenMetadataField[];
    constructor(properties: EncodableProps<Token2022TokenMetadata>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TokenMetadata;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022TokenMetadataField implements IsCodable {
    key: string;
    value: string;
    constructor(properties: EncodableProps<Token2022TokenMetadataField>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TokenMetadataField;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022TransferFee implements IsCodable {
    epoch: bigint;
    maximumFee: bigint;
    transferFeeBasisPoints: number;
    constructor(properties: EncodableProps<Token2022TransferFee>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TransferFee;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022TransferFeeAmount implements IsCodable {
    withheldAmount: bigint;
    constructor(properties: EncodableProps<Token2022TransferFeeAmount>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TransferFeeAmount;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022TransferFeeConfig implements IsCodable {
    transferFeeConfigAuthority: PublicKeyLike;
    withdrawWithheldAuthority: PublicKeyLike;
    withheldAmount: bigint;
    olderTransferFee: Token2022TransferFee;
    newerTransferFee: Token2022TransferFee;
    constructor(properties: EncodableProps<Token2022TransferFeeConfig>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TransferFeeConfig;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022TransferHook implements IsCodable {
    authority: PublicKeyLike;
    programId: PublicKeyLike;
    constructor(properties: EncodableProps<Token2022TransferHook>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TransferHook;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare class Token2022TransferHookAccount implements IsCodable {
    transferring: boolean;
    constructor(properties: EncodableProps<Token2022TransferHookAccount>);
    static decode(data: string | Uint8Array | Buffer | null): Token2022TransferHookAccount;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare const TOKEN_2022_PROGRAM_ID: PublicKey<"TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb" & Tag<string, "Base58Encoded", never> & Tag<string, "SolanaAddress", never> & Tag<string, "SolanaAddressInfo", {
systemProgram: true;
name: "Token 2022 Program";
description: "2022 SPL Program";
}>>;

export declare const TOKEN_2022_PROGRAM_ID_STRING: Token2022Program;

export declare const TOKEN_EXPIRED_MESSAGE: "token expired";

export declare const TOKEN_PROGRAM_ID: PublicKey<"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" & Tag<string, "Base58Encoded", never> & Tag<string, "SolanaAddress", never> & Tag<string, "SolanaAddressInfo", {
systemProgram: true;
name: "Token Program";
description: "Original SPL Program";
}>>;

export declare const TOKEN_PROGRAM_ID_STRING: TokenProgram;

export declare const TOKEN_SOL_ADDRESS_STRING: WrappedSolana;

export declare class TokenAccount implements IsDecodable {
    static readonly decodePolicy: DecodeDataPolicy;
    address: PublicKeyLike;
    mint: PublicKeyLike;
    owner: PublicKeyLike;
    amount: bigint;
    delegateOption: number;
    delegate: PublicKeyLike;
    state: number;
    isNativeOption: number;
    isNative: bigint;
    delegatedAmount: bigint;
    closeAuthorityOption: number;
    closeAuthority: PublicKeyLike;
    constructor(properties: TokenAccount, address: PublicKeyLike);
    static decode(data: string | Uint8Array | Buffer | null | undefined, address: SolanaAddressLike): TokenAccount;
    static get(address: SolanaAddressLike): Promise<TokenAccount>;
    static getSchema(): Schema;
}

export declare class TokenAccount2022 implements IsDecodable {
    address: PublicKeyLike;
    mint: PublicKeyLike;
    owner: PublicKeyLike;
    amount: bigint;
    delegateOption: number;
    delegate: PublicKeyLike;
    state: number;
    isNativeOption: number;
    isNative: bigint;
    delegatedAmount: bigint;
    closeAuthorityOption: number;
    closeAuthority: PublicKeyLike;
    accountType: Token2022AccountType;
    tlvData: Token2022TlvData;
    constructor(properties: TokenAccount2022, address: PublicKeyLike);
    static decode(data: string | Uint8Array | Buffer | null | undefined, address: SolanaAddressLike): TokenAccount2022;
    static get(address: SolanaAddressLike): Promise<TokenAccount2022>;
    static getSchema(): Schema;
}

declare interface TokenAccount2022_2 {
    mint: PublicKeyLike;
    owner: PublicKeyLike;
    amount: bigint;
    delegateOption: number;
    delegate: PublicKeyLike;
    state: number;
    isNativeOption: number;
    isNative: bigint;
    delegatedAmount: bigint;
    closeAuthorityOption: number;
    closeAuthority: PublicKeyLike;
    accountType: number;
    tlvData: unknown;
}

declare interface TokenAccount2022DataResult {
    address: PublicKeyLike;
    data: Uint8Array | null;
    decoded: TokenAccount2022_2 | null;
    accountType: AccountDataType.TokenAccount2022;
    executable: boolean;
    lamports: number;
    owner: string;
    rentEpoch: number;
}

declare interface TokenAccount_2 {
    mint: PublicKeyLike;
    owner: PublicKeyLike;
    amount: bigint;
    delegateOption: number;
    delegate: PublicKeyLike;
    state: number;
    isNativeOption: number;
    isNative: bigint;
    delegatedAmount: bigint;
    closeAuthorityOption: number;
    closeAuthority: PublicKeyLike;
}

export declare interface TokenAccountBalance {
    amount: string;
    decimals: number;
    uiAmount: number;
    uiAmountString: string;
}

declare interface TokenAccountDataResult {
    address: PublicKeyLike;
    data: Uint8Array | null;
    decoded: TokenAccount_2 | null;
    accountType: AccountDataType.TokenAccount;
    executable: boolean;
    lamports: number;
    owner: string;
    rentEpoch: number;
}

export declare interface TokenAccountResponse {
    account: AccountInfo;
    pubkey: string;
}

export declare interface TokenAccountsFilter {
    mint?: SolanaAddressLike;
    programId?: SolanaAddressLike;
}

export declare type TokenId<T extends string = string> = Tagged<AssetId<T>, "TokenId">;

export declare interface TokenInstructionEvent extends BaseInstructionEvent {
    decoded: TransferInstruction | TransferCheckedInstruction | BurnInstruction | BurnCheckedInstruction | MintToInstruction | MintToCheckedInstruction;
    details: TokenInstructionEventDetails;
}

export declare interface TokenInstructionEventDetails {
    mintAddress: PublicKey;
    uiAmount: BigDecimal;
    amount: BigDecimal;
    decimals: number;
    sourceTokenAccount: PublicKey | null;
    sourceOwnerAddress: PublicKey;
    destinationTokenAccount: PublicKey | null;
    destinationOwnerAddress: PublicKey;
}

export declare type TokenInstructionNoDetailsEvent = BaseInstructionEvent & {
    decoded: TransferInstruction | TransferCheckedInstruction | BurnInstruction | BurnCheckedInstruction | MintToInstruction | MintToCheckedInstruction;
    details?: null;
};

export declare class TokenMint implements IsDecodable {
    address: PublicKeyLike;
    mintAuthorityOption: number;
    mintAuthority: PublicKeyLike;
    supply: bigint;
    decimals: number;
    isInitialized: boolean;
    freezeAuthorityOption: number;
    freezeAuthority: PublicKeyLike;
    constructor(properties: TokenMint, address: PublicKeyLike);
    static decode(data: string | Uint8Array | Buffer | null | undefined, address: SolanaAddressLike): TokenMint;
    static get(address: SolanaAddressLike): Promise<TokenMint>;
    static getSchema(): Schema;
}

export declare class TokenMint2022 implements IsDecodable {
    address: PublicKeyLike;
    mintAuthorityOption: number;
    mintAuthority: PublicKeyLike;
    supply: bigint;
    decimals: number;
    isInitialized: boolean;
    freezeAuthorityOption: number;
    freezeAuthority: PublicKeyLike;
    accountType: Token2022AccountType;
    tlvData: Token2022TlvData;
    constructor(properties: TokenMint2022, address: PublicKeyLike);
    static decode(data: string | Uint8Array | Buffer | null | undefined, address: SolanaAddressLike): TokenMint2022;
    static get(address: SolanaAddressLike): Promise<TokenMint2022>;
    static getSchema(): Schema;
}

declare interface TokenMint2022_2 {
    mintAuthorityOption: number;
    mintAuthority: PublicKeyLike;
    supply: bigint;
    decimals: number;
    isInitialized: boolean;
    freezeAuthorityOption: number;
    freezeAuthority: PublicKeyLike;
    accountType: number;
    tlvData: unknown;
}

declare interface TokenMint2022DataResult {
    address: PublicKeyLike;
    data: Uint8Array | null;
    decoded: TokenMint2022_2 | null;
    accountType: AccountDataType.TokenMint2022;
    executable: boolean;
    lamports: number;
    owner: string;
    rentEpoch: number;
}

declare interface TokenMint_2 {
    mintAuthorityOption: number;
    mintAuthority: PublicKeyLike;
    supply: bigint;
    decimals: number;
    isInitialized: boolean;
    freezeAuthorityOption: number;
    freezeAuthority: PublicKeyLike;
}

declare interface TokenMintDataResult {
    address: PublicKeyLike;
    data: Uint8Array | null;
    decoded: TokenMint_2 | null;
    accountType: AccountDataType.TokenMint;
    executable: boolean;
    lamports: number;
    owner: string;
    rentEpoch: number;
}

export declare type TokenMovementInstruction = TransferInstruction | TransferCheckedInstruction | BurnInstruction | BurnCheckedInstruction | MintToInstruction | MintToCheckedInstruction;

export declare type TokenName<T extends string = string> = Tagged<AssetName<T>, "TokenName">;

export declare type TokenProgram = SolanaAddressInfo<"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", {
    systemProgram: true;
    name: "Token Program";
    description: "Original SPL Program";
}>;

export declare type TokenSymbol<T extends string = string> = Tagged<AssetSymbol<T>, "TokenSymbol">;

export declare const TOKYO_TIMEZONE: TokyoTimezone;

export declare type TokyoTimezone = IANATimeZone<"Asia/Tokyo">;

export declare const TOO_MANY_REQUESTS_MESSAGE: "too many requests";

/**
 * Error thrown when data is longer than the fields in the data type.
 */
export declare class TooMuchDataDecoderError extends DecoderError {
    readonly type = "TOO_MUCH_DATA";
    constructor(message?: string, originalError?: unknown);
}

export declare function toPascalCase(...segments: ReadonlyArray<StrListArg>): PascalCase;

export declare function toPascalCaseSafe(name: string): PascalCase;

export declare const TOPIX: TopixName;

export declare type Topix = EquityIndexId<"topix">;

export declare type TopixName = EquityIndexName<"TOPIX">;

export declare type TopixSymbol = EquityIndexSymbol<"TPX">;

export declare function toScreamingSnakeCase(...segments: ReadonlyArray<StrListArg>): ScreamingSnakeCase;

export declare function toScreamingSnakeCaseSafe(name: string): ScreamingSnakeCase;

export declare function toSlug(...slugValue: string[]): Slug;

export declare function toSnakeCase(...segments: ReadonlyArray<StrListArg>): SnakeCase;

export declare function toTitleCase(...segments: ReadonlyArray<StrListArg>): TitleCase;

export declare type TradingPair<TBase extends string = string, TQuote extends string = string> = TBase extends string ? TQuote extends string ? Tagged<ConstructTradingPair<ExtractStringFromTagged<TBase>, ExtractStringFromTagged<TQuote>>, "TradingPair"> : Tagged<ValidTradingPairPattern<TBase>, "TradingPair"> : Tagged<ValidTradingPairPattern<TBase>, "TradingPair">;

export declare class Transaction {
    private readonly shared;
    private readonly construction;
    private readonly rpc;
    private readonly internals;
    constructor(initArgs?: TransactionConstructor | null, args?: SignaturePubkeyPair[] | MessageDeserializeArgs, rpcData?: RpcTransactionData);
    addInstruction(...instructions: MessageInstruction[]): this;
    addInstructions(...instructions: MessageInstruction[]): this;
    add(...instructions: MessageInstruction[]): this;
    setFeePayer(payer: PublicKeyLike): this;
    get feePayer(): PublicKeyLike | null;
    getFeePayer(): PublicKeyLike | null;
    setRecentBlockhash(blockhash: SolanaBlockhash): this;
    getRecentBlockhash(): SolanaBlockhash | null;
    get recentBlockhash(): SolanaBlockhash | null;
    getSlot(): SolanaSlot | null;
    get slot(): SolanaSlot | null;
    getBlockTime(): TimestampS | null;
    get blockTime(): TimestampS | null;
    getSignature(): SolanaTransactionSignature | null;
    get signature(): SolanaTransactionSignature | null;
    getSignatures(): SolanaTransactionSignature[];
    get signatures(): SolanaTransactionSignature[];
    getSignaturePairs(): SignaturePubkeyPair[];
    getMeta(): TransactionMeta | null;
    get meta(): TransactionMeta | null;
    getStatus(): TransactionStatus;
    getCompiledInstructions(): CompiledInstruction[];
    get compiledInstructions(): CompiledInstruction[];
    getInstructions(): MessageInstruction[];
    get instructions(): MessageInstruction[];
    enableLookupTables(enabled?: boolean): this;
    disableLookupTables(): this;
    addLookupTableAccounts(...accounts: AddressLookupTableAccount[]): this;
    setLookupTableMode(mode: AddressTableLookupMode): this;
    setLookupTableAccounts(accounts: AddressLookupTableAccount[]): this;
    flushLookupTables(): this;
    get version(): ValidSolanaTransactionVersion;
    getVersion(): ValidSolanaTransactionVersion;
    isVersioned(): boolean;
    getAddressTableLookups(): AddressTableLookup[];
    getLoadedAddresses(): ResolvedAddressTableLookups | null;
    isReadonly(): boolean;
    setIsReadonly(isReadonly: boolean): this;
    get message(): Message;
    getMessage(): Message;
    compileMessage(): Message;
    serializeMessage(): Uint8Array;
    messageData(): Promise<Uint8Array>;
    getStaticAccountKeys(): PublicKeyLike[];
    get staticAccountKeys(): PublicKeyLike[];
    getAccountKeys(): PublicKeyLike[];
    get accountKeys(): PublicKeyLike[];
    /**
     * Returns true if the transaction succeeded (meta exists and has no error)
     */
    get ok(): boolean;
    /**
     * Returns the transaction error if present, null otherwise
     * Avoids needing to check: transaction.meta?.err
     */
    get err(): TransactionMetaError;
    isOk(): boolean;
    getErr(): TransactionMetaError;
    get keys(): PublicKeyLike[];
    get programId(): PublicKeyLike;
    get data(): Uint8Array;
    sign(..._signers: PublicKeyLike[]): void;
    partialSign(..._signers: PublicKeyLike[]): void;
    setSigners(..._signers: PublicKeyLike[]): void;
    addSignature(pubkey: PublicKeyLike, signature: Uint8Array | Buffer): void;
    verifySignatures(requireAllSignatures?: boolean): Promise<boolean>;
    serialize(requireAllSignatures?: boolean): Uint8Array;
    serializeBase64(requireAllSignatures?: boolean): Base64;
    /**
     * Populate Transaction object from message and signatures
     */
    static populate(message: Message, signatures?: SignaturePubkeyPair[]): Transaction;
    /**
     * Parse a wire transaction into a Transaction object
     */
    static from(buffer: Buffer | Uint8Array | number[], args?: MessageDeserializeArgs): Transaction;
    static deserialize(serializedTransaction: Uint8Array, args?: MessageDeserializeArgs): Transaction;
    static deserializeFromTransactionData(transactionData: TransactionData): Transaction;
    toJSON(): TransactionJSON;
    /**
     * Get the estimated fee associated with a transaction.
     * We use our own cost estimator that doesn't need to use an RPC endpoint.
     */
    getEstimatedFee(_connection: any): Promise<number | null>;
}

export declare class TransactionCache {
    private static enabled;
    private static maxSize;
    private static slots;
    private static indexBySig;
    private static writeIndex;
    private static sizeValue;
    static setEnabled(enabled: boolean): void;
    static getEnabled(): boolean;
    static setMaxSize(size: number): void;
    static getMaxSize(): number;
    static size(): number;
    static clear(): void;
    static remove(signature: SolanaTransactionSignature): boolean;
    static get(signature: SolanaTransactionSignature): Transaction | null;
    static set(transaction: Transaction): void;
}

export declare interface TransactionConstruction {
    feePayer?: PublicKeyLike | null;
    lastValidBlockHeight?: number;
    nonceInfo?: NonceInformation;
    minNonceContextSlot?: number;
    signatures: SignaturePubkeyPair[];
}

declare interface TransactionConstructionConstructor {
    feePayer?: PublicKeyLike | null;
    lastValidBlockHeight?: number;
    nonceInfo?: NonceInformation;
    minNonceContextSlot?: number;
    signatures?: SignaturePubkeyPair[];
    recentBlockhash?: SolanaBlockhash;
    instructions?: MessageInstruction[];
    isReadonly?: boolean;
}

export declare type TransactionConstructor = TransactionConstructionConstructor | Message | Buffer | Uint8Array | number[] | TransactionData;

export declare interface TransactionCostEstimate {
    baseFee: number;
    signatureFees: number;
    computeUnitFees: number;
    priorityFee: number;
    accountCreationCosts: number;
    pdaCreationCosts: number;
    totalCostLamports: number;
    totalCostSol: number;
    breakdown: CostBreakdownItem[];
}

export declare class TransactionCostEstimator {
    private static readonly DEFAULT_BASE_FEE_LAMPORTS;
    private static readonly ADDITIONAL_SIGNATURE_FEE;
    private static readonly DEFAULT_COMPUTE_UNIT_PRICE;
    private static readonly DEFAULT_COMPUTE_UNIT_LIMIT;
    private static readonly DEFAULT_PRIORITY_FEE;
    private static readonly LAMPORTS_PER_SOL;
    private static readonly DEFAULT_LAMPORTS_PER_BYTE_YEAR;
    private static readonly DEFAULT_EXEMPTION_THRESHOLD;
    private static readonly ACCOUNT_OVERHEAD;
    private static readonly ATA_SIZE;
    private constructor();
    /**
     * Calculate rent cost for account size using proper Solana rent formula
     */
    private static calculateRentCost;
    /**
     * Calculate signature fees based on transaction structure
     */
    private static calculateSignatureFees;
    /**
     * Calculate compute unit fees from compute budget instructions
     */
    private static calculateComputeUnitFees;
    /**
     * Check if instruction is a compute budget instruction
     */
    private static isComputeBudgetInstruction;
    private static isLegacyComputeBudgetInstruction;
    /**
     * Estimate the cost of a transaction
     */
    static estimateTransactionCost(transaction: Transaction, config?: EstimatorConfig): Promise<TransactionCostEstimate>;
    /**
     * Analyze a single instruction for potential costs
     */
    private static analyzeInstructionCosts;
    static getNetworkFeeValue(transaction: Transaction, config?: EstimatorConfig): number;
    /**
     * Check if instruction has encoding schema
     */
    private static hasSchema;
    /**
     * Analyze instruction using its encoding schema
     */
    private static analyzeEncodableInstruction;
    /**
     * Get estimated cost for an account by size
     */
    static getAccountCreationCost(accountSize: number, lamportsPerByteYear?: number, exemptionThreshold?: number): number;
    /**
     * Convert lamports to SOL
     */
    static lamportsToSol(lamports: number): number;
    /**
     * Convert SOL to lamports
     */
    static solToLamports(sol: number): number;
    /**
     * Get minimum transaction cost (base fee + priority fee)
     */
    static getMinimumTransactionCost(config?: EstimatorConfig): number;
    /**
     * Estimate cost for creating an ATA
     */
    static getATACreationCost(lamportsPerByteYear?: number, exemptionThreshold?: number): number;
    /**
     * Quick estimate without checking existing accounts
     */
    static quickEstimate(transaction: Transaction, config?: EstimatorConfig): Promise<TransactionCostEstimate>;
    /**
     * Detailed estimate with account existence checks
     */
    static detailedEstimate(transaction: Transaction, config?: EstimatorConfig): Promise<TransactionCostEstimate>;
}

export declare type TransactionData = RawTransactionData | LegacyJsonTransactionData | VersionedJsonTransactionData | ParsedTransactionData;

/**
 * Thrown when transaction deserialization fails
 */
export declare class TransactionDeserializationError extends TransactionError {
    readonly reason?: string | undefined;
    readonly type = "DESERIALIZATION";
    constructor(message?: string, reason?: string | undefined, originalError?: unknown);
}

/**
 * Base class for Transaction-related errors
 */
export declare abstract class TransactionError extends SolomonLabsError {
    readonly category = "TRANSACTION";
}

export declare type TransactionEvent = InstructionEvent | LogEvent;

export declare interface TransactionInnerInstructionGroup {
    index: number;
    instructions: TransactionInstruction[];
}

export declare interface TransactionInstruction {
    programIdIndex: number;
    accounts: number[];
    data?: Base58 | null;
    stackHeight?: number | null;
}

export declare interface TransactionInstructionJSON {
    keys: {
        pubkey: SolanaAddress;
        isSigner: boolean;
        isWritable: boolean;
    }[];
    programId: SolanaAddress;
    data: number[];
}

export declare interface TransactionInternals {
    signatures: Map<SolanaAddress, Uint8Array>;
    isReadonly: boolean;
    type: "constructed" | "wire" | "rpc";
}

export declare interface TransactionJSON {
    recentBlockhash: SolanaBlockhash | null;
    feePayer: SolanaAddress | null;
    nonceInfo: {
        nonce: string;
        nonceInstruction: TransactionInstructionJSON;
    } | null;
    instructions: TransactionInstructionJSON[];
    signers: SolanaAddress[];
}

export declare interface TransactionMessageHeader {
    numReadonlySignedAccounts: number;
    numReadonlyUnsignedAccounts: number;
    numRequiredSignatures: number;
}

export declare interface TransactionMeta {
    err: TransactionMetaError;
    fee: number;
    innerInstructions?: TransactionInnerInstructionGroup[];
    logMessages?: string[];
    postBalances: number[];
    postTokenBalances?: TransactionMetaTokenBalance[];
    preBalances: number[];
    preTokenBalances?: TransactionMetaTokenBalance[];
    rewards?: {
        commission?: number;
        lamports: number;
        postBalance: number;
        pubkey: string;
        rewardType: "fee" | "rent" | "voting" | "staking";
    }[];
    status: {
        Ok: null;
    } | {
        Err: unknown;
    };
    computeUnitsConsumed?: number;
    returnData?: {
        programId: SolanaAddress;
        data: [string, string];
    };
    loadedAddresses?: {
        readonly: SolanaAddress[];
        writable: SolanaAddress[];
    };
}

export declare type TransactionMetaError = TransactionMetaInstructionError | null | {
    [key: string]: unknown;
};

export declare interface TransactionMetaInstructionError {
    InstructionError: [number, {
        Custom: number;
    } | string];
}

export declare interface TransactionMetaTokenBalance {
    accountIndex: number;
    mint: SolanaAddress;
    owner?: SolanaAddress;
    programId?: SolanaAddress;
    uiTokenAmount: {
        amount: string;
        decimals: number;
        uiAmount: number | null;
        uiAmountString: string;
    };
}

export declare class TransactionParseCache {
    private static enabled;
    private static maxSize;
    private static slots;
    private static indexBySig;
    private static writeIndex;
    private static sizeValue;
    static setEnabled(enabled: boolean): void;
    static getEnabled(): boolean;
    static setMaxSize(size: number): void;
    static getMaxSize(): number;
    static size(): number;
    static clear(): void;
    static remove(transaction: Transaction): boolean;
    static getInstructions(transaction: Transaction): ParsedInstruction[] | null;
    static setInstructions(transaction: Transaction, instructions: ParsedInstruction[]): void;
    static getLogs(transaction: Transaction): ParsedLog[] | null;
    static setLogs(transaction: Transaction, logs: ParsedLog[]): void;
    static getEvents(transaction: Transaction): TransactionEvent[] | null;
    static setEvents(transaction: Transaction, events: TransactionEvent[]): void;
    private static rebuildToSize;
    private static collectOldestToNewest;
    private static getSlot;
    private static getOrCreateEntry;
    private static insertNew;
    private static cleanupSlotIfEmpty;
}

export declare type TransactionResponse = LegacyJsonTransactionData | VersionedJsonTransactionData | ParsedTransactionData;

export declare type TransactionResponseWithTimestamp = TransactionResponse & {
    blockTime: TimestampS;
};

/**
 * Thrown when transaction serialization fails
 */
export declare class TransactionSerializationError extends TransactionError {
    readonly reason?: string | undefined;
    readonly type = "SERIALIZATION";
    constructor(message?: string, reason?: string | undefined, originalError?: unknown);
}

export declare interface TransactionShared {
    signatures: SolanaTransactionSignature[];
    message: Message;
}

/**
 * Thrown when transaction size exceeds limits
 */
export declare class TransactionSizeExceededError extends TransactionError {
    readonly actualSize?: number | undefined;
    readonly maxSize?: number | undefined;
    readonly type = "SIZE_EXCEEDED";
    constructor(message?: string, actualSize?: number | undefined, maxSize?: number | undefined, originalError?: unknown);
}

export declare type TransactionStatus = "Ok" | "Err" | undefined;

/**
 * Thrown when transaction validation fails
 */
export declare class TransactionValidationError extends TransactionError {
    readonly validationRule?: string | undefined;
    readonly type = "VALIDATION";
    constructor(message?: string, validationRule?: string | undefined, originalError?: unknown);
}

/** 12 = TransferChecked { amount: u64, decimals: u8 } */
export declare class TransferCheckedInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    source: SolanaAddressLike;
    mint: SolanaAddressLike;
    destination: SolanaAddressLike;
    owner: SolanaAddressLike;
    amount: bigint | number | string;
    decimals: number;
    constructor(props: EncodableProps<TransferCheckedInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): TransferCheckedInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/** 3 = Transfer { amount: u64 } */
export declare class TransferInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    source: SolanaAddressLike;
    destination: SolanaAddressLike;
    owner: SolanaAddressLike;
    amount: bigint | number | string;
    constructor(props: EncodableProps<TransferInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): TransferInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

/**
 * 11 = TransferWithSeed { lamports: u64, fromSeed: String, fromOwner: Pubkey }
 */
export declare class TransferWithSeedInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    from: SolanaAddressLike;
    base: SolanaAddressLike;
    to: SolanaAddressLike;
    lamports: bigint | number | string;
    fromSeed: string;
    fromOwner: PublicKeyLike;
    constructor(props: EncodableProps<TransferWithSeedInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): TransferWithSeedInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

export declare type Transformable = BrandedTransformable | string | bigint | number | boolean;

export declare const TransformableBrand: unique symbol;

export declare type TransformerName<T extends string = string> = Tagged<T, "TransformerName">;

/**
 * TransformIdsToUuids<T, Only>
 * - Renames `${X}Id` -> `${X}Uuid` when the value is PrimaryId (optionally null/undefined)
 * - Converts the value type from PrimaryId -> Uuid, preserving null/undefined
 * - Preserves optionality of properties
 * - `Only` can whitelist prefixes (e.g., "season" | "campaign"). Default: all.
 */
export declare type TransformIdsToUuids<T, Only extends string = string> = {
    [K in keyof T as RemapIdKeyIfPrimary<K & string, T[K], Only>]: StripNullish<T[K]> extends PrimaryId ? K extends `${infer P}Id` ? P extends Only ? WithSameNullish<T[K], Uuid> : T[K] : T[K] : T[K];
};

export declare type TransformIdsToUuidsAs<T, Alias extends StringMap, Only extends string = string> = {
    [K in keyof T as StripNullish<T[K]> extends PrimaryId ? K extends `${infer P}Id` ? P extends Only ? `${AliasOf<P, Alias>}Uuid` : K : K : K]: StripNullish<T[K]> extends PrimaryId ? K extends `${infer P}Id` ? P extends Only ? WithSameNullish<T[K], Uuid> : T[K] : T[K] : T[K];
};

export declare type TransformIdsToUuidsAsDeep<T, Alias extends StringMap, Only extends string = string> = T extends (...args: infer A) => infer R ? (...args: A) => R : T extends ReadonlyArray<infer U> ? TransformIdsToUuidsAsDeep<U, Alias, Only>[] : T extends object ? {
    [K in keyof T as StripNullish<T[K]> extends PrimaryId ? K extends `${infer P}Id` ? P extends Only ? `${AliasOf<P, Alias>}Uuid` : K : K : K]: StripNullish<T[K]> extends PrimaryId ? K extends `${infer P}Id` ? P extends Only ? WithSameNullish<T[K], Uuid> : T[K] : T[K] : TransformIdsToUuidsAsDeep<T[K], Alias, Only>;
} : T;

export declare type TransformIdsToUuidsAsPlus<T, UuidAlias extends StringMap, KeepIdAlias extends StringMap = {}> = TransformIdsToUuidsAs<RemapIdBases<T, KeepIdAlias, keyof KeepIdAlias & string>, UuidAlias, keyof UuidAlias & string>;

export declare type TransformIdsToUuidsDeep<T, Only extends string = string> = T extends (...args: infer A) => infer R ? (...args: A) => R : T extends ReadonlyArray<infer U> ? TransformIdsToUuidsDeep<U, Only>[] : T extends object ? {
    [K in keyof T as RemapIdKeyIfPrimary<K & string, T[K], Only>]: StripNullish<T[K]> extends PrimaryId ? K extends `${infer P}Id` ? P extends Only ? WithSameNullish<T[K], Uuid> : T[K] : T[K] : TransformIdsToUuidsDeep<T[K], Only>;
} : T;

/**
 * Validates and trims string with optional fallback
 */
export declare function trimString(value: unknown, fallback?: string): string | undefined;

export declare function trimString<T extends string>(value: unknown, fallback: T): string | T;

export declare function trimString<T extends string>(value: unknown): T | undefined;

export declare interface TruncatedLog extends BaseLog {
    type: "truncated";
}

export declare type Tuple<T, Len extends number> = BuildTuple<T, Len>;

export declare const TWELVE_HOURS_MS: DurationMs<43200000>;

export declare const TWELVE_HOURS_S: DurationS<43200>;

export declare const TWO_HOURS_MS: DurationMs<7200000>;

export declare const TWO_HOURS_S: DurationS<7200>;

export declare const TWO_MINUTES_MS: DurationMs<120000>;

export declare const TWO_MINUTES_S: DurationS<120>;

declare type TypedArrayMutableProperties = "copyWithin" | "fill" | "reverse" | "set" | "sort";

export declare const TYPEFORM_API_URL: Url<"https://api.typeform.com">;

/** Some typing */
export declare type TypeSpan = ["type", LabelValueRole];

export declare class U128Wrapper implements IsCodable {
    value: bigint;
    constructor(properties: EncodableProps<U128Wrapper>);
    static decode(data: string | Uint8Array | Buffer | null): U128Wrapper;
    encode(): Uint8Array;
    static getSchema(): Schema;
}

export declare type U16<uint16 extends number = number> = Tagged<WholeNumberPattern<uint16>, "uint16">;

export declare type U16Bytes = Bytes<"u16">;

export declare type U24<uint24 extends number = number> = Tagged<WholeNumberPattern<uint24>, "uint24">;

export declare type U24Bytes = Bytes<"u24">;

export declare type U32<uint32 extends number = number> = Tagged<WholeNumberPattern<uint32>, "uint32">;

export declare type U32Bytes = Bytes<"u32">;

/**
 * These types mitigate the huge headache of Javascript only having "number"
 * and "bigint" types. Tagging these at the defintion site helps avoid encoding
 * the incorrect types.
 * Bigint already enforces whole numbers so the literal trick isn't needed there.
 */
export declare type U8<uint8 extends number = number> = Tagged<WholeNumberPattern<uint8>, "uint8">;

export declare type U8Bytes = Bytes<"u8">;

export declare function uiAmountFromMeta(meta: {
    amount: string | number | bigint;
    decimals: number | null;
    uiAmount: number | string | null;
    uiAmountString: string;
}): BigDecimal;

export declare function uiAmountFromRawAmount(amount: string | number | bigint, decimals: number): BigDecimal;

export declare function uint8ArrayLast4ToString(bytes: Uint8Array): string;

export declare const UNAUTHORIZED_MESSAGE: "unauthorized";

/**
 * Thrown when using the wrong deserialize method for a message version
 */
export declare class UnexpectedMessageVersionError extends MessageError {
    readonly version: number | string;
    readonly expectedVersion: number | string;
    readonly type = "UNEXPECTED_VERSION";
    constructor(message: string | undefined, version: number | string, expectedVersion: number | string, originalError?: unknown);
}

export declare type UnionContains<Union, Target> = Target extends Union ? true : false;

export declare type UnionToIntersection<U> = (U extends unknown ? (x: U) => void : never) extends (x: infer I) => void ? I : never;

export declare type UnionToTuple<T> = ((T extends any ? (t: T) => T : never) extends infer U ? (U extends any ? (u: U) => any : never) extends (v: infer V) => any ? V : never : never) extends (_: any) => infer W ? [...UnionToTuple<Exclude<T, W>>, W] : [];

export declare const UNITED_STATES_DOLLAR: UnitedStatesDollar;

export declare const UNITED_STATES_DOLLAR_NAME: UnitedStatesDollarName;

export declare const UNITED_STATES_DOLLAR_SYMBOL: UnitedStatesDollarSymbol;

export declare type UnitedStatesDollar = AssetId<"united-states-dollar">;

export declare type UnitedStatesDollarName = AssetName<"United States Dollar">;

export declare type UnitedStatesDollarSymbol = AssetSymbol<"USD">;

export declare type UnitedStatesHoliday = IndependenceDay | MartinLutherKingJrDay | PresidentsDay | MemorialDay | Juneteenth | LaborDay | ColumbusDay | VeteransDay | Thanksgiving;

export declare type UnixTimestampMilliseconds<T extends number> = `${T}` extends `${string}.${string}` ? never : `${T}` extends `${infer First}${infer Rest}` ? First extends "1" | "2" ? Len<Rest> extends 12 ? IsAllDigits<Rest> extends true ? T : never : never : never : never;

export declare type UnixTimestampNanoseconds<T extends number> = `${T}` extends `${string}.${string}` ? never : `${T}` extends `${infer First}${infer Rest}` ? First extends "1" | "2" ? Len<Rest> extends 18 ? IsAllDigits<Rest> extends true ? T : never : never : never : never;

export declare type UnixTimestampSeconds<T extends number> = `${T}` extends `${string}.${string}` ? never : `${T}` extends `${infer First}${infer Rest}` ? First extends "1" | "2" ? Len<Rest> extends 9 ? IsAllDigits<Rest> extends true ? T : never : never : never : never;

declare interface UnknownAccountDataResult {
    address: PublicKeyLike;
    data: Uint8Array | null;
    accountType: AccountDataType.Unknown;
    decoded?: null;
    executable: boolean;
    lamports: number;
    owner: string;
    rentEpoch: number;
}

/**
 * Error thrown when field type can't be determined.
 */
export declare class UnknownDataTypeDecoderError extends DecoderError {
    readonly type = "UNKNOWN_DATA";
    constructor(message?: string, originalError?: unknown);
}

/**
 * Error thrown when field type can't be determined.
 */
export declare class UnknownDataTypeEncoderError extends EncoderError {
    readonly type = "UNKNOWN_DATA";
    constructor(message?: string, originalError?: unknown);
}

/**
 * Helper function to safely convert unknown values to string
 */
export declare function unknownToString(data: unknown): string;

export declare function unrot(str: string, shift: number): string;

declare type UnsignedScalarMap = {
    "8": U8;
    "16": U16;
    "24": U24;
    "32": U32;
};

declare type UnsignedWidth = "8" | "16" | "24" | "32";

/**
 * Thrown when unsupported message version is encountered
 */
export declare class UnsupportedMessageVersionError extends MessageError {
    readonly version: number | string;
    readonly type = "UNSUPPORTED_VERSION";
    constructor(message: string | undefined, version: number | string, originalError?: unknown);
}

/**
 * Thrown when deserializeFromTransactionData is called with a transaction data format,
 * that is not supported.
 */
export declare class UnsupportedTransactionDataFormatError extends TransactionError {
    readonly type = "UNSUPPORTED_TRANSACTION_DATA_FORMAT";
    constructor(message?: string, originalError?: unknown);
}

export declare const UPGRADE_INSECURE_REQUESTS_ENABLED: "1";

/**
 * 12 = UpgradeNonceAccount {}
 */
export declare class UpgradeNonceAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    nonceAccount: SolanaAddressLike;
    constructor(props: EncodableProps<UpgradeNonceAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): UpgradeNonceAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

declare type UppercaseAlphaNumeric = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

export declare type Url<S extends string = string> = Tagged<S, "Url">;

export declare type UrlInfo<Metadata extends UrlMetadata> = Tagged<Url, "UrlInfo", Metadata>;

export declare type UrlMetadata = {
    url: string;
    description: string;
};

export declare const US: US_2;

declare type US_2 = CountryCode<"US">;

export declare const USDC: Usdc;

export declare type Usdc = TokenId<"usdc">;

export declare const USDC_NAME: UsdcName;

export declare const USDC_SYMBOL: UsdcSymbol;

export declare type UsdcName = TokenName<"USDC">;

export declare type UsdcSolana = SolanaAddressInfo<"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", {
    tokenId: SolanaToken<Usdc>;
    decimals: 6;
    program: "Token";
}>;

export declare type UsdcSymbol = TokenSymbol<"USDC">;

export declare const USDS: Usds;

export declare type Usds = TokenId<"usds">;

export declare const USDS_NAME: UsdsName;

export declare const USDS_SYMBOL: UsdsSymbol;

export declare type UsdsName = TokenName<"USDS">;

export declare type UsdsSolana = SolanaAddressInfo<"USDSwr9ApdHk5bvJKMjzff41FfuX8bSxdKcR81vTwcA", {
    tokenId: SolanaToken<Usds>;
    decimals: 6;
    program: "Token";
}>;

export declare type UsdsSymbol = TokenSymbol<"USDS">;

export declare type UserAgent<S extends string = string> = Tagged<S, "UserAgent">;

export declare type UserId = Tagged<PrimaryId, "UserId">;

export declare type Username = Tagged<Identity<string>, "Username">;

export declare type UTF8 = Tagged<Encoded, "UTF8">;

export declare type Uuid = Tagged<Identity<string>, "Uuid">;

export declare type UuidV4<T extends string = string> = Tagged<Uuid & UuidV4Pattern<T>, "UuidV4">;

export declare type UuidV4Pattern<T extends string> = T extends `${string}-${string}-4${string}-${string}-${string}` ? T : never;

export declare type UuidV5<T extends string = string> = Tagged<Uuid & UuidV5Pattern<T>, "UuidV5">;

export declare type UuidV5Pattern<T extends string> = T extends `${string}-${string}-5${string}-${string}-${string}` ? T : never;

export declare type ValidAssetIdPattern<T extends string> = string extends T ? T : ValidString<T, AlphaNumeric | "-", 1, 100> extends true ? T : never;

export declare type ValidAssetNamePattern<T extends string> = string extends T ? T : ValidString<T, AlphaNumeric | " ", 1, 50> extends true ? T : never;

export declare type ValidAssetSymbolPattern<T extends string> = string extends T ? T : ValidString<T, AlphaNumeric, 1, 10> extends true ? T : never;

export declare function validateGetProgramAccountsCall(endpoint: string, programId: SolanaAddressLike, config: ProgramAccountsConfig): void;

export declare const VALIDATION_ERROR_MESSAGE: "validation error";

export declare type ValidBase58<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? S : _Base58OK<S, Min, Max> extends true ? S : never;

export declare type ValidBase64<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? S : _Base64OK<S, Min, Max> extends true ? S : never;

export declare type ValidBase64Url<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? S : _Base64UrlOK<S, Min, Max> extends true ? S : never;

export declare type ValidBinanceTickerPattern<T extends string> = string extends T ? T : T extends `${infer _BaseQuote}_PERP` ? T extends `${infer Base}${infer Quote}_PERP` ? ValidAssetSymbolPattern<Base> extends never ? never : ValidAssetSymbolPattern<Quote> extends never ? never : T : never : T extends `${infer Base}${infer Quote}` ? ValidAssetSymbolPattern<Base> extends never ? never : ValidAssetSymbolPattern<Quote> extends never ? never : T : never;

declare type ValidCamelCasePattern<S extends string> = S extends `${infer First}${infer _Rest}` ? First extends Lowercase<First> ? AllCharsInSet<S, AlphaNumeric> extends true ? S extends `${string}_${string}` | `${string}-${string}` | `${string} ${string}` ? never : S : never : never : S extends "" ? never : S;

export declare type ValidEcPrivateKey = {
    format: "pkcs8-der";
    key: Uint8Array;
} | {
    format: "jwk";
    jwk: {
        kty: "EC";
        crv: "P-256";
        d: string;
        x: string;
        y: string;
    };
};

export declare type ValidEcPublicKey = {
    format: "spki-der";
    key: Uint8Array;
} | {
    format: "raw-uncompressed";
    key: Uint8Array;
} | {
    format: "jwk";
    jwk: {
        kty: "EC";
        crv: "P-256";
        x: string;
        y: string;
    };
};

export declare type ValidEd25519PrivateKey = {
    format: "pkcs8-der";
    key: Uint8Array;
} | {
    format: "raw-seed";
    key: Uint8Array;
} | {
    format: "raw-priv-64";
    key: Uint8Array;
} | {
    format: "jwk";
    jwk: {
        kty: "OKP";
        crv: "Ed25519";
        d: string;
        x: string;
    };
};

export declare type ValidEmailPattern<T extends string> = T extends `${string}@${string}.${string}` ? T : never;

export declare type ValidHexLower<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? S : _HexLowerOK<S, Min, Max> extends true ? S : never;

export declare type ValidHexUpper<S extends string, Min extends number, Max extends number | undefined = undefined> = string extends S ? S : _HexUpperOK<S, Min, Max> extends true ? S : never;

declare type ValidKebabCasePattern<S extends string> = S extends `${string}-${string}` ? AllCharsInSet<S, LowercaseAlphaNumeric | "-"> extends true ? S extends `-${string}` | `${string}-` ? never : S : never : AllCharsInSet<S, LowercaseAlphaNumeric> extends true ? S extends "" ? never : S : never;

declare type ValidPascalCasePattern<S extends string> = S extends `${infer First}${infer _Rest}` ? First extends Uppercase<First> ? AllCharsInSet<S, AlphaNumeric> extends true ? S extends `${string}_${string}` | `${string}-${string}` | `${string} ${string}` ? never : S : never : never : S extends "" ? never : S;

export declare type ValidRsaPrivateKey = {
    format: "pkcs8-der";
    key: Uint8Array;
} | {
    format: "jwk";
    jwk: {
        kty: "RSA";
        n: string;
        e: string;
        d: string;
        p: string;
        q: string;
        dp: string;
        dq: string;
        qi: string;
    };
};

export declare type ValidRsaPublicKey = {
    format: "spki-der";
    key: Uint8Array;
} | {
    format: "jwk";
    jwk: {
        kty: "RSA";
        n: string;
        e: string;
    };
};

declare type ValidScreamingSnakeCasePattern<S extends string> = S extends `${string}_${string}` ? AllCharsInSet<S, UppercaseAlphaNumeric | "_"> extends true ? S extends `_${string}` | `${string}_` ? never : S : never : AllCharsInSet<S, UppercaseAlphaNumeric> extends true ? S extends "" ? never : S : never;

declare type ValidSnakeCasePattern<S extends string> = S extends `${string}_${string}` ? AllCharsInSet<S, UppercaseAlphaNumeric | "_"> extends true ? S extends `_${string}` | `${string}_` ? never : S : never : AllCharsInSet<S, UppercaseAlphaNumeric> extends true ? S extends "" ? never : S : never;

export declare type ValidSolanaTransactionVersion = "legacy" | 0;

declare type ValidString<S extends string, Allowed extends string, Min extends number, Max extends number | undefined = undefined, Even extends boolean = false> = AllCharsInSet<S, Allowed> extends true ? (Even extends true ? IsEvenLength<S> : true) extends true ? Between<Len<S>, Min, Max> extends true ? true : false : false : false;

declare type ValidTitleCasePattern<S extends string> = S extends `${infer First}${infer Rest}` ? First extends Uppercase<First> ? Rest extends Lowercase<Rest> ? AllCharsInSet<S, AlphaNumeric | " "> extends true ? S : never : never : never : S extends "" ? never : S;

export declare type ValidTradingPairPattern<T extends string> = string extends T ? T : T extends `${infer Base}-${infer Quote}` ? ValidAssetSymbolPattern<Base> extends never ? never : ValidAssetSymbolPattern<Quote> extends never ? never : T : never;

export declare function valueToBigint(value: unknown): bigint;

export declare function valueToNumber(value: unknown): number;

export declare function valueToPublicKey(value: unknown): PublicKey;

export declare const VAULT_PROGRAM_ID_STRING: SolanaAddress<"D4AiKFxjZBrhd6MeRAAj6wNxFcZJuRMBrXW9UpqeS8Ax">;

/**
 * Error thrown when node fails to verify a Ed25519 signed message.
 */
export declare class VerifyEd25519Error extends CryptoUtilError {
    readonly type = "VERIFY_ED25519_ERROR";
    constructor(message?: string, originalError?: unknown);
}

export declare type Version<V extends string | number = string> = Tagged<Identity<V>, "Version">;

export declare const VERSION_PREFIX_MASK = 127;

export declare interface VersionedJsonTransactionData extends RpcTransactionData {
    transaction: {
        message: VersionedTransactionMessage;
        signatures: SolanaTransactionSignature[];
    };
    version: number;
}

export declare interface VersionedTransactionMessage extends LegacyTransactionMessage {
    addressTableLookups?: AddressTableLookup[];
}

export declare const VETERANS_DAY: VeteransDay;

export declare type VeteransDay = Holiday<"Veterans Day">;

export declare type VideoMimeType = "video/mp4" | "video/webm" | "video/ogg";

export declare const VOTE_PROGRAM_ID = "Vote111111111111111111111111111111111111111";

export declare interface WallClock {
    year: number;
    month: number;
    day: number;
    hour?: number;
    minute?: number;
    second?: number;
    millisecond?: number;
    nanosecond?: number;
}

export declare type WebhookSecret<S extends string = string> = Tagged<ApiSecret<S>, "WebhookSecret">;

export declare const WELL_KNOWN_TOKENS_STRINGS: {
    SOL: WrappedSolana;
    USDC: UsdcSolana;
    USDT: TetherSolana;
    USDe: EthenaUsdeSolana;
    PYUSD: PayPalUsdSolana;
    USDS: UsdsSolana;
    USDv: SolomonUsdvSolana;
    FDUSD: FirstDigitalUsdSolana;
    USDY: OndoUsDollarYieldSolana;
    DUSD: StandXDusdSolana;
    sUSD: SolayerUsdSolana;
    sUSDv: SolanaAddress<"pTA4St7D5WshfLUPBXoaxn5m8e3k2ort2DVt3gUTa17">;
    lockedUSDv: SolanaAddress<"5zHAA3Gk8tG3Bze2o6S2bboFBCTYoJmDvk8oyjcFVAbz">;
};

export declare type WesternHoliday = NewYearsDay | ChristmasDay | GoodFriday | EasterSaturday | EasterMonday;

export declare type WholeNumberPattern<T extends number> = `${T}` extends `${string}.${string}` ? never : T;

/**
 * Usually never want to do this but there are some edge cases.
 * Normally widening is the least desired side effect when using TypeScript otherwise.
 */
export declare type Widen<T> = T extends string ? string : T extends number ? number : T extends boolean ? boolean : T extends object ? {
    [K in keyof T]: Widen<T[K]>;
} : T;

export declare const WINDOWS: OperatingSystem<"Windows">;

/**
 * ---------------------------------------------------------------------------
 * Struct helpers
 * ---------------------------------------------------------------------------
 */
/**
 * Per-field wire descriptor for a specific struct field.
 */
export declare type WireFieldForKey<Struct, Key extends keyof Struct> = WireLayoutForValue<Struct[Key]> extends infer W extends WireScalar<BytesType, number, unknown> ? W & {
    readonly key: Key;
} : never;

/**
 * ---------------------------------------------------------------------------
 * Public mapping: value type -> wire scalar descriptor
 * ---------------------------------------------------------------------------
 */
/**
 * Map a logical value type to its wire representation.
 *
 * - Known Solana types use the fixed byte widths above.
 * - Tagged numeric / bigint types use the BytesEncodingFromScalar mapping
 *   plus ByteLengthForEncoding to compute their lengths.
 */
export declare type WireLayoutForValue<Value> = [WireScalarForKnown<Value>] extends [never] ? WireScalarForTagged<Value> : WireScalarForKnown<Value>;

/**
 * Describe how a scalar value is stored on the wire.
 *
 * - `encoding` is the BytesType tag (u8, u16, bu64, ...).
 * - `byteLength` is the number of bytes this field occupies.
 * - `scalar` is the logical scalar type implied by the encoding.
 */
export declare interface WireScalar<Encoding extends BytesType, ByteLength extends number, Scalar = ScalarFromBytesEncoding<Encoding>> {
    readonly encoding: Encoding;
    readonly byteLength: ByteLength;
    readonly scalar: Scalar;
}

/**
 * ---------------------------------------------------------------------------
 * Well-known fixed-size Solana types
 * ---------------------------------------------------------------------------
 *
 * 1. SolanaAddress               = 32 bytes
 * 2. SolanaTransactionSignature  = 64 bytes
 * 3. SolanaSlot                  = 32 bytes
 * 4. TimestampMs                 = 64 bytes
 * 5. TimestampS                  = 32 bytes
 */
declare type WireScalarForKnown<Value> = Value extends SolanaAddress ? WireScalar<"u8", 32, SolanaAddress> : Value extends SolanaTransactionSignature ? WireScalar<"u8", 64, SolanaTransactionSignature> : Value extends SolanaSlot ? WireScalar<"u8", 32, SolanaSlot> : Value extends TimestampMs ? WireScalar<"u8", 64, TimestampMs> : Value extends TimestampS ? WireScalar<"u8", 32, TimestampS> : never;

/**
 * ---------------------------------------------------------------------------
 * Tagged scalar helpers
 * ---------------------------------------------------------------------------
 */
/**
 * For any Tagged<number|bigint, "..."> scalar that has a BytesEncodingFromScalar
 * mapping, describe its wire representation.
 */
declare type WireScalarForTagged<Value> = Value extends Tagged<unknown, string> ? BytesEncodingFromScalar<Value> extends infer Encoding ? [Encoding] extends [BytesType] ? Encoding extends BytesType ? ByteLengthForEncoding<Encoding> extends infer ByteLength extends number ? WireScalar<Encoding, ByteLength, ScalarFromBytesEncoding<Encoding>> : never : never : never : never : never;

/**
 * 5 = WithdrawNonceAccount { lamports: u64 }
 */
export declare class WithdrawNonceAccountInstruction implements IsEncodable {
    static readonly name: string;
    static readonly discriminator: Uint8Array<ArrayBuffer>;
    programId: PublicKeyLike;
    nonceAccount: SolanaAddressLike;
    to: SolanaAddressLike;
    recentBlockhashSysvar: SolanaAddressLike;
    rentSysvar: SolanaAddressLike;
    nonceAuthority: SolanaAddressLike;
    lamports: bigint | number | string;
    constructor(props: EncodableProps<WithdrawNonceAccountInstruction>);
    encode(): Uint8Array;
    static decode(data: DecodableInstruction): WithdrawNonceAccountInstruction;
    accounts(): InstructionAccount[];
    instruction(): EncodedInstruction;
    static getSchema(): Schema;
    static getAccountsSchema(): EncodeAccountsSchemaOrNull;
}

declare type WithSameNullish<From, To> = (null extends From ? To | null : To) extends infer T1 ? undefined extends From ? T1 | undefined : T1 : never;

export declare type WithVersion = {
    version?: ValidSolanaTransactionVersion;
};

export declare const WORLD_TIME_API_ENDPOINT: Url<"https://worldtimeapi.org/api/timezone/Etc/UTC">;

export declare type WrappedSolana = SolanaAddressInfo<"So11111111111111111111111111111111111111112", {
    tokenId: SolanaToken<PayPalUsd>;
    decimals: 9;
    program: "Token";
}>;

export declare const WRITE_TIMEOUT_MESSAGE: "write timeout";

export declare const XETR: XETR_2;

declare type XETR_2 = ExchangeName<"XETR">;

export declare type XETRId = ExchangeId<XETR_2>;

export { }
