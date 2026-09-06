const DEFAULT_MAX_JSON_BYTES = 16 * 1024 * 1024;
const DEFAULT_MAX_JSON_DEPTH = 128;

export function compareUnicodeCodeUnits(left, right) {
    if (left < right) return -1;
    if (left > right) return 1;
    return 0;
}

export function containsControlCharacter(value) {
    for (let index = 0, length = value.length; index < length; index += 1) {
        const codeUnit = value.charCodeAt(index);
        if (codeUnit <= 0x1f || (codeUnit >= 0x7f && codeUnit <= 0x9f)) return true;
    }
    return false;
}

export function cloneJsonValue(value) {
    const serialized = JSON.stringify(value);
    if (serialized === undefined) throw new Error("JSON value cannot be cloned");
    return JSON.parse(serialized);
}

function isJsonWhitespace(character) {
    return character === " " || character === "\t" || character === "\n" || character === "\r";
}

function isPrimitiveDelimiter(character) {
    return (
        character === undefined ||
        isJsonWhitespace(character) ||
        character === "," ||
        character === "]" ||
        character === "}"
    );
}

export function parseStrictJson(source, sourceName, options = {}) {
    const maxBytes = options.maxBytes ?? DEFAULT_MAX_JSON_BYTES;
    const maxDepth = options.maxDepth ?? DEFAULT_MAX_JSON_DEPTH;
    const sourceBytes = new TextEncoder().encode(source).length;
    if (!Number.isSafeInteger(maxBytes) || maxBytes < 1) throw new Error("maxBytes must be a positive safe integer");
    if (!Number.isSafeInteger(maxDepth) || maxDepth < 1) throw new Error("maxDepth must be a positive safe integer");
    if (sourceBytes > maxBytes) throw new Error(`${sourceName}: exceeds ${maxBytes} bytes`);

    let index = 0;
    const fail = (message) => {
        throw new Error(`${sourceName}: ${message} at byte ${index}`);
    };
    const skipWhitespace = () => {
        while (index < source.length && isJsonWhitespace(source[index])) index += 1;
    };
    const parseString = () => {
        if (source[index] !== '"') fail("expected string");
        const start = index++;
        while (index < source.length) {
            const character = source[index];
            if (character === '"') {
                index += 1;
                try {
                    return JSON.parse(source.slice(start, index));
                } catch (error) {
                    throw new Error(`${sourceName}: invalid JSON string at byte ${start}`, { cause: error });
                }
            }
            if (character === "\\") {
                index += 2;
                if (index > source.length) fail("unterminated string escape");
            } else {
                if (character.charCodeAt(0) < 0x20) fail("unescaped control character in string");
                index += 1;
            }
        }
        fail("unterminated string");
    };
    const parsePrimitive = () => {
        const start = index;
        while (!isPrimitiveDelimiter(source[index])) index += 1;
        const token = source.slice(start, index);
        if (token.length === 0) fail("expected JSON value");
        try {
            return JSON.parse(token);
        } catch (error) {
            throw new Error(`${sourceName}: invalid JSON token ${token}`, { cause: error });
        }
    };
    const parseArray = (depth) => {
        if (depth > maxDepth) fail(`exceeds maximum nesting depth ${maxDepth}`);
        const output = [];
        index += 1;
        skipWhitespace();
        if (source[index] === "]") {
            index += 1;
            return output;
        }
        while (index < source.length) {
            output.push(parseValue(depth));
            skipWhitespace();
            if (source[index] === "]") {
                index += 1;
                return output;
            }
            if (source[index] !== ",") fail("expected ',' or ']'");
            index += 1;
            skipWhitespace();
        }
        fail("unterminated array");
    };
    const parseObject = (depth) => {
        if (depth > maxDepth) fail(`exceeds maximum nesting depth ${maxDepth}`);
        const output = {};
        const keys = new Set();
        index += 1;
        skipWhitespace();
        if (source[index] === "}") {
            index += 1;
            return output;
        }
        while (index < source.length) {
            const key = parseString();
            if (keys.has(key)) fail(`duplicate object key ${JSON.stringify(key)}`);
            keys.add(key);
            skipWhitespace();
            if (source[index] !== ":") fail("expected ':'");
            index += 1;
            output[key] = parseValue(depth);
            skipWhitespace();
            if (source[index] === "}") {
                index += 1;
                return output;
            }
            if (source[index] !== ",") fail("expected ',' or '}'");
            index += 1;
            skipWhitespace();
        }
        fail("unterminated object");
    };
    const parseValue = (parentDepth) => {
        skipWhitespace();
        const character = source[index];
        if (character === "{") return parseObject(parentDepth + 1);
        if (character === "[") return parseArray(parentDepth + 1);
        if (character === '"') return parseString();
        if (character === undefined) fail("unexpected end of input");
        return parsePrimitive();
    };
    const value = parseValue(0);
    skipWhitespace();
    if (index !== source.length) fail("trailing content");
    return value;
}

export function assertExactObjectKeys(value, sourceName, allowedKeys) {
    if (value === null || typeof value !== "object" || Array.isArray(value)) {
        throw new Error(`${sourceName} must be an object`);
    }
    const allowed = new Set(allowedKeys);
    for (const key of Object.keys(value)) {
        if (!allowed.has(key)) throw new Error(`${sourceName} contains unknown field ${key}`);
    }
}

export function canonicalJson(value) {
    if (value === null || typeof value === "string" || typeof value === "boolean") return JSON.stringify(value);
    if (typeof value === "number") {
        if (!Number.isFinite(value)) throw new Error("canonical JSON does not support non-finite numbers");
        return JSON.stringify(value);
    }
    if (Array.isArray(value)) return `[${value.map((entry) => canonicalJson(entry)).join(",")}]`;
    if (typeof value === "object") {
        const keys = Object.keys(value).sort(compareUnicodeCodeUnits);
        return `{${keys.map((key) => `${JSON.stringify(key)}:${canonicalJson(value[key])}`).join(",")}}`;
    }
    throw new Error(`canonical JSON does not support ${typeof value}`);
}
