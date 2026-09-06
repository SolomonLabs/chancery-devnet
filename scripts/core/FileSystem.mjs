import { createHash, randomUUID } from "node:crypto";
import { access, lstat, mkdir, open, readdir, readFile, realpath, rename, rm } from "node:fs/promises";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";

import { compareUnicodeCodeUnits, containsControlCharacter } from "./StrictJson.mjs";

export const workspaceRoot = resolve(import.meta.dirname, "..", "..");

export function repositoryRelativePath(absolutePath) {
    return relative(workspaceRoot, absolutePath).split(sep).join("/");
}

export function assertCanonicalRepositoryPath(value, sourceName) {
    if (
        typeof value !== "string" ||
        value.length === 0 ||
        value.startsWith("/") ||
        value.includes("\\") ||
        containsControlCharacter(value)
    ) {
        throw new Error(`${sourceName} must be a non-empty canonical repository-relative POSIX path`);
    }
    const segments = value.split("/");
    if (segments.some((segment) => segment.length === 0 || segment === "." || segment === "..")) {
        throw new Error(`${sourceName} contains an invalid path segment`);
    }
    return value;
}

export function resolveRepositoryPath(relativePath, sourceName = "repository path") {
    const canonicalPath = assertCanonicalRepositoryPath(relativePath, sourceName);
    const absolutePath = resolve(workspaceRoot, canonicalPath);
    const relativePathFromRoot = relative(workspaceRoot, absolutePath);
    if (relativePathFromRoot.startsWith("..") || isAbsolute(relativePathFromRoot)) {
        throw new Error(`${sourceName} escapes the repository`);
    }
    return absolutePath;
}

export async function pathExists(path) {
    try {
        await access(path);
        return true;
    } catch (error) {
        if (error !== null && typeof error === "object" && (error.code === "ENOENT" || error.code === "ENOTDIR")) {
            return false;
        }
        throw error;
    }
}

export function sha256Bytes(bytes) {
    return createHash("sha256").update(bytes).digest("hex");
}

export async function sha256File(path) {
    return sha256Bytes(await readFile(path));
}

export function deterministicJsonBytes(value) {
    return new TextEncoder().encode(`${JSON.stringify(value, null, 4)}\n`);
}

export async function assertRegularNonSymlinkFile(path, sourceName, options = {}) {
    const absolutePath = resolve(path);
    const containmentRoot = options.containmentRoot === undefined ? undefined : resolve(options.containmentRoot);
    if (containmentRoot !== undefined) {
        const relativePathFromRoot = relative(containmentRoot, absolutePath);
        if (
            relativePathFromRoot === ".." ||
            relativePathFromRoot.startsWith(`..${sep}`) ||
            isAbsolute(relativePathFromRoot)
        ) {
            throw new Error(`${sourceName} escapes the containment root`);
        }
        if (options.rejectSymlinkComponents === true) {
            const segments = relativePathFromRoot.split(sep).filter((segment) => segment.length > 0);
            let currentPath = containmentRoot;
            for (let index = 0, length = segments.length; index < length; index += 1) {
                currentPath = resolve(currentPath, segments[index]);
                const componentStatus = await lstat(currentPath);
                if (componentStatus.isSymbolicLink()) {
                    throw new Error(`${sourceName} path contains a symbolic link: ${currentPath}`);
                }
            }
        }
    }

    const fileStats = await lstat(absolutePath);
    if (!fileStats.isFile() || fileStats.isSymbolicLink()) {
        throw new Error(`${sourceName} must be a regular non-symlink file`);
    }
    const minimumBytes = options.minimumBytes ?? 0;
    const maximumBytes = options.maximumBytes ?? Number.MAX_SAFE_INTEGER;
    if (fileStats.size < minimumBytes || fileStats.size > maximumBytes) {
        throw new Error(`${sourceName} must be between ${String(minimumBytes)} and ${String(maximumBytes)} bytes`);
    }
    if (containmentRoot !== undefined) {
        const [resolvedPath, resolvedRoot] = await Promise.all([realpath(absolutePath), realpath(containmentRoot)]);
        if (resolvedPath !== resolvedRoot && !resolvedPath.startsWith(`${resolvedRoot}${sep}`)) {
            throw new Error(`${sourceName} resolves outside ${repositoryRelativePath(containmentRoot)}`);
        }
    }
    return fileStats;
}

export async function assertRegularContainedFile(path, containmentRoot, sourceName) {
    return assertRegularNonSymlinkFile(path, sourceName, { containmentRoot });
}

export async function collectRegularFilesRecursively(path, options = {}) {
    const absolutePath = resolve(path);
    const exists = await pathExists(absolutePath);
    if (!exists) {
        if (options.missingOkay === true) return [];
        throw new Error(`file collection root is missing: ${absolutePath}`);
    }
    const output = [];
    const predicate = options.predicate ?? (() => true);
    const excludedDirectoryNames = options.excludedDirectoryNames ?? new Set();

    async function collect(candidatePath) {
        const status = await lstat(candidatePath);
        if (status.isSymbolicLink()) throw new Error(`file collection path must not be a symlink: ${candidatePath}`);
        if (status.isFile()) {
            if (predicate(candidatePath)) output.push(candidatePath);
            return;
        }
        if (!status.isDirectory()) return;
        const entries = await readdir(candidatePath, { withFileTypes: true });
        entries.sort((left, right) => compareUnicodeCodeUnits(left.name, right.name));
        for (let index = 0, length = entries.length; index < length; index += 1) {
            const entry = entries[index];
            if (entry.isDirectory() && excludedDirectoryNames.has(entry.name)) continue;
            await collect(resolve(candidatePath, entry.name));
        }
    }

    await collect(absolutePath);
    return output;
}

export async function writeFileAtomically(path, bytes, mode = 0o600) {
    const byteArray = typeof bytes === "string" ? new TextEncoder().encode(bytes) : bytes;
    if (!(byteArray instanceof Uint8Array)) throw new Error("atomic file bytes must be a Uint8Array or string");
    await mkdir(dirname(path), { recursive: true });
    const temporaryPath = `${path}.tmp-${process.pid}-${randomUUID()}`;
    const handle = await open(temporaryPath, "wx", mode);
    try {
        await handle.writeFile(byteArray);
        await handle.sync();
    } catch (error) {
        await handle.close();
        await rm(temporaryPath, { force: true });
        throw error;
    }
    await handle.close();
    try {
        await rename(temporaryPath, path);
        const directoryHandle = await open(dirname(path), "r");
        try {
            await directoryHandle.sync();
        } finally {
            await directoryHandle.close();
        }
    } catch (error) {
        await rm(temporaryPath, { force: true });
        throw error;
    }
}

export function sortedRepositoryPaths(paths) {
    return [...paths].sort(compareUnicodeCodeUnits);
}
