// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Script} from "forge-std/Script.sol";
import {Vortex369} from "../contracts/Vortex369.sol";

contract DeployVortex369 is Script {
    // Genesis sacrifice — anchored 2026-07-02
    bytes32 constant GENESIS_TX_HASH =
        0x87bb61f99066460a7df4438c39084fd77c2f1f6287b97261ff7034ddd3503f6c;
    uint256 constant GENESIS_BLOCK = 25444056;
    address constant SACRIFICE_TO = 0x0000000000000000000000000000000000000369;

    function run() external {
        vm.startBroadcast();
        new Vortex369(GENESIS_TX_HASH, GENESIS_BLOCK, SACRIFICE_TO);
        vm.stopBroadcast();
    }
}