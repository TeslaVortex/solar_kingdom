// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Script, console2} from "forge-std/Script.sol";
import {Vortex369} from "../contracts/Vortex369.sol";
import {CrownCommand} from "../contracts/CrownCommand.sol";
import {SolarKingdom} from "../contracts/SolarKingdom.sol";

/// @notice Deploy full Phase 2B stack: Vortex369 + CrownCommand + SolarKingdom
contract DeployPhase2B is Script {
    bytes32 constant GENESIS_TX_HASH =
        0x87bb61f99066460a7df4438c39084fd77c2f1f6287b97261ff7034ddd3503f6c;
    uint256 constant GENESIS_BLOCK = 25444056;
    address constant SACRIFICE_TO = 0x0000000000000000000000000000000000000369;

    function run() external {
        vm.startBroadcast();
        Vortex369 vortex = new Vortex369(GENESIS_TX_HASH, GENESIS_BLOCK, SACRIFICE_TO);
        CrownCommand crown = new CrownCommand(address(vortex));
        SolarKingdom kingdom = new SolarKingdom(address(vortex));
        vm.stopBroadcast();

        console2.log("Vortex369:", address(vortex));
        console2.log("CrownCommand:", address(crown));
        console2.log("SolarKingdom:", address(kingdom));
    }
}
