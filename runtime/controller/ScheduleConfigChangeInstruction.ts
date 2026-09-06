import { PublicKey } from "@solomon-labs/publickey";

import { AUTHORITY_ROLE, IX, MODULE } from "../../clients/ts/src/constants.js";
import type { ProposeConfigChangeInstruction } from "../../clients/ts/src/instructions/ProposeConfigChangeInstruction.js";
import { ExecuteChanceryInstruction } from "./ExecuteChanceryInstruction.js";
import type { ControllerConfiguration } from "./types.js";

export class ScheduleConfigChangeInstruction extends ExecuteChanceryInstruction {
    private readonly timelockSeconds: bigint;
    private readonly lifetimeSeconds: bigint;

    constructor(configuration: ControllerConfiguration, caller: PublicKey, proposal: ProposeConfigChangeInstruction,
        timelockSeconds: bigint, lifetimeSeconds: bigint) {
        super(configuration, caller, proposal);
        if (this.roleMask !== (1 << AUTHORITY_ROLE.GOVERNANCE)
            || this.chanceryData[0] !== MODULE.CONTROL
            || this.chanceryData[1] !== IX.control.PROPOSE_CONFIG_CHANGE) {
            throw new Error("scheduled instruction must be a governance config-change proposal");
        }
        const maximumSigned64 = (1n << 63n) - 1n;
        if (timelockSeconds < 0n || timelockSeconds > maximumSigned64
            || lifetimeSeconds <= 0n || lifetimeSeconds > maximumSigned64) {
            throw new Error("config-change delay and lifetime are outside the supported range");
        }
        this.timelockSeconds = timelockSeconds;
        this.lifetimeSeconds = lifetimeSeconds;
    }

    override encode(): Uint8Array {
        const data = new Uint8Array(18 + this.chanceryData.length);
        data.set([0x02, this.roleMask]);
        const view = new DataView(data.buffer);
        view.setBigUint64(2, this.timelockSeconds, true);
        view.setBigUint64(10, this.lifetimeSeconds, true);
        data.set(this.chanceryData, 18);
        return data;
    }
}
