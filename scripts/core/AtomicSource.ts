import { randomUUID } from "node:crypto";
import { renameSync, rmSync, writeFileSync } from "node:fs";

export function replaceSourceAtomically(path: string, source: string): void {
    const temporaryPath = path + "." + randomUUID() + ".tmp";
    try {
        writeFileSync(temporaryPath, source, { encoding: "utf8", flag: "wx" });
        renameSync(temporaryPath, path);
    } finally {
        rmSync(temporaryPath, { force: true });
    }
}
