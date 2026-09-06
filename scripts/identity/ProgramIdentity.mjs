import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

import { ed25519PublicKeyFromSolanaKeypair } from "../core/Ed25519.mjs";
import {
    replaceClientProgramIdentity,
    replaceNetworkConfigFaucetIdentity,
    replaceNetworkConfigProgramIdentity,
    replaceRustProgramIdentity,
} from "../core/NetworkIdentity.mjs";
import { parseSolanaKeypairSource } from "../core/SolanaKeypairFile.mjs";

export async function stampProgramIdentities(root, programKeypairPath, faucetKeypairPath) {
    const read = (path) => readFile(resolve(root, path), "utf8");
    const programId = ed25519PublicKeyFromSolanaKeypair(
        parseSolanaKeypairSource(await read(programKeypairPath), programKeypairPath),
    );
    const faucetProgramId = ed25519PublicKeyFromSolanaKeypair(
        parseSolanaKeypairSource(await read(faucetKeypairPath), faucetKeypairPath),
    );
    if (programId === faucetProgramId) throw new Error("Chancery and faucet program identities must differ");

    const rustPath = "programs/chancery/src/lib.rs";
    const faucetPath = "programs/faucet/src/lib.rs";
    const networkPath = "config/networks/devnet.ts";
    const clientPath = "clients/ts/src/constants.ts";
    const idlPath = "programs/chancery/idl/idl.json";
    const [rust, faucet, network, client, idlSource] = await Promise.all([
        read(rustPath), read(faucetPath), read(networkPath), read(clientPath), read(idlPath),
    ]);
    const idl = JSON.parse(idlSource);
    idl.address = programId;
    if (idl.metadata !== undefined && idl.metadata.deployments !== undefined) {
        idl.metadata.deployments = { devnet: programId };
    }
    const updates = [
        { path: rustPath, source: rust, updated: replaceRustProgramIdentity(rust, programId, rustPath) },
        { path: faucetPath, source: faucet, updated: replaceRustProgramIdentity(faucet, faucetProgramId, faucetPath) },
        { path: networkPath, source: network, updated: replaceNetworkConfigFaucetIdentity(
            replaceNetworkConfigProgramIdentity(network, programId, networkPath), faucetProgramId, networkPath,
        ) },
        { path: clientPath, source: client, updated: replaceClientProgramIdentity(client, programId, clientPath) },
        { path: idlPath, source: idlSource, updated: JSON.stringify(idl, null, 4) + "\n" },
    ];
    const changedPaths = [];
    for (const update of updates) {
        if (update.source === update.updated) continue;
        await writeFile(resolve(root, update.path), update.updated);
        changedPaths.push(update.path);
    }
    return { programId, faucetProgramId, changedPaths };
}
