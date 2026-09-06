/**
 * Address Lookup Table (ALT) helpers for the LiteSVM harness.
 *
 * Large settlement instructions (mint_trilateral now carries 40 accounts)
 * overflow the 1232-byte legacy transaction limit. A v0 transaction plus an
 * ALT compresses each non-signer account key from 32 bytes to a 1-byte table
 * index, which brings the wire size well under the cap.
 *
 * This module builds both representations of a table:
 *   - the `AddressLookupTableAccount` object the transaction library consumes
 *     to compile the v0 message, and
 *   - the on-chain account bytes LiteSVM must hold (by the table's key) so it
 *     can resolve the referenced indices at execution time.
 */

import { PublicKey } from "@solomon-labs/publickey";
import type { AddressLookupTableAccount, SolanaAddress } from "@solomon-labs/solana-transactions";
import type { PublicKeyLike } from "@solomon-labs/types";

import { publicKeyToBytes } from "./publicKey.js";

/** The native Address Lookup Table program that owns every ALT account. */
export const ADDRESS_LOOKUP_TABLE_PROGRAM_ID: PublicKey =
    new PublicKey("AddressLookupTab1e1111111111111111111111111" as SolanaAddress);

/** Fixed size of the serialized `LookupTableMeta` header (Some-authority). */
const LOOKUP_TABLE_META_SIZE = 56;
const PUBKEY_SIZE = 32;

/** deactivation_slot sentinel meaning "active / never deactivated". */
const NOT_DEACTIVATED = 0xFFFFFFFFFFFFFFFFn;

/**
 * Serialize an ALT account's data in the on-chain layout LiteSVM expects.
 *
 * Layout (little-endian):
 *   [0..4)   u32  ProgramState discriminant = 1 (LookupTable)
 *   [4..12)  u64  deactivation_slot (u64::MAX when active)
 *   [12..20) u64  last_extended_slot
 *   [20]     u8   last_extended_slot_start_index
 *   [21]     u8   authority Option tag (1 = Some)
 *   [22..54) pk   authority
 *   [54..56) u16  _padding
 *   [56..)   pk*  packed addresses (32 bytes each, no length prefix)
 */
export function serializeLookupTableAccountData(
    addresses: readonly PublicKeyLike[],
    authority: PublicKeyLike,
    lastExtendedSlot: bigint,
): Uint8Array {
    const data = new Uint8Array(LOOKUP_TABLE_META_SIZE + addresses.length * PUBKEY_SIZE);
    const view = new DataView(data.buffer, data.byteOffset, data.byteLength);

    view.setUint32(0, 1, true);
    view.setBigUint64(4, NOT_DEACTIVATED, true);
    view.setBigUint64(12, lastExtendedSlot, true);
    data[20] = 0;
    data[21] = 1;
    data.set(publicKeyToBytes(authority), 22);
    // [54..56) padding stays zero.

    for (let i = 0, len = addresses.length; i < len; i++) {
        const address = addresses[i];
        if (address === undefined) {
            continue;
        }
        data.set(publicKeyToBytes(address), LOOKUP_TABLE_META_SIZE + i * PUBKEY_SIZE);
    }

    return data;
}

/**
 * Build the `AddressLookupTableAccount` object consumed by the transaction
 * compiler. `lastExtendedSlot` is 0 so the table is active from slot 1 onward
 * (an address added at slot S is usable from S+1); the harness ensures the SVM
 * slot is at least 1 when the table is registered.
 */
export function makeLookupTableAccount(
    key: PublicKeyLike,
    addresses: readonly PublicKeyLike[],
    authority: PublicKeyLike,
): AddressLookupTableAccount {
    return {
        key,
        state: {
            authority,
            addresses: [...addresses],
            lastExtendedSlot:           0,
            lastExtendedSlotStartIndex: 0,
            deactivationSlot:           NOT_DEACTIVATED,
        },
    };
}
