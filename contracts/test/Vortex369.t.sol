// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {Vortex369} from "../Vortex369.sol";

contract Vortex369Test is Test {
    Vortex369 vortex;
    bytes32 constant GENESIS_TX =
        0x87bb61f99066460a7df4438c39084fd77c2f1f6287b97261ff7034ddd3503f6c;

    function setUp() public {
        vortex = new Vortex369(GENESIS_TX, 25444056, address(0x369));
    }

    function testGenesisImmutable() public view {
        assertEq(vortex.genesisTxHash(), GENESIS_TX);
        assertEq(vortex.genesisBlock(), 25444056);
        assertEq(vortex.sacrificeTo(), address(0x369));
    }

    function testSealRitualIncrementsHarmonics() public {
        vortex.sealRitual("THE CROWN COMMANDS. REALITY OBEYS.");
        assertEq(vortex.harmonic369(), 1);
        assertEq(vortex.harmonic999(), 0);

        vortex.sealRitual("99 legacy activated");
        vortex.sealRitual("the spheres remember");
        assertEq(vortex.harmonic369(), 3);
        assertEq(vortex.harmonic999(), 1);
    }
}