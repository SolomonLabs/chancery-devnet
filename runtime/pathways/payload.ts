const PATHWAY_POLICY_SIZE = 616;
const PATHWAY_POLICY_PAYLOAD_SIZE = 552;
const LIMIT_POLICY_ID_OFFSET = 224;
const STATUS_FLAGS_OFFSET = 352;
const PATHWAY_PAUSE = 8n;

export function pathwayPolicyPayload(account: Uint8Array, limitPolicyId: Uint8Array | null = null): Uint8Array {
    if (account.length !== PATHWAY_POLICY_SIZE) throw new Error("unexpected PathwayPolicy account size");
    if (limitPolicyId !== null && limitPolicyId.length !== 32) throw new Error("limit policy identifier must contain 32 bytes");
    const payload = Uint8Array.from(account.subarray(0, PATHWAY_POLICY_PAYLOAD_SIZE));
    if (limitPolicyId !== null) payload.set(limitPolicyId, LIMIT_POLICY_ID_OFFSET);
    const view = new DataView(payload.buffer);
    view.setBigUint64(STATUS_FLAGS_OFFSET, view.getBigUint64(STATUS_FLAGS_OFFSET, true) & ~PATHWAY_PAUSE, true);
    return payload;
}
