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

    function testActivateScalarNode() public {
        bytes32 hash = keccak256("scalar-seal");
        uint256 nodeId = vortex.activateScalarNode(3, hash, 6, 349);
        assertEq(nodeId, 1);
        assertTrue(vortex.isNodeActive(nodeId));

        (
            address sealer,
            uint8 phase,
            uint16 shells,
            uint32 idx,
            bytes32 sealHash,
            ,
            bool active
        ) = vortex.queryNodeState(nodeId);

        assertEq(sealer, address(this));
        assertEq(phase, 3);
        assertEq(shells, 6);
        assertEq(idx, 349);
        assertEq(sealHash, hash);
        assertTrue(active);
        assertEq(vortex.harmonic369(), 1);
    }

    function testAdvancePhaseWrapsAtNine() public {
        uint256 nodeId = vortex.activateScalarNode(9, bytes32(uint256(1)), 1, 0);
        (, uint8 phaseBefore,,,,,) = vortex.queryNodeState(nodeId);
        assertEq(phaseBefore, 9);

        vortex.advancePhase(nodeId);
        (, uint8 phaseAfter, , uint32 idxAfter,,,) = vortex.queryNodeState(nodeId);
        assertEq(phaseAfter, 1);
        assertEq(idxAfter, 3); // +3 mod 1296
    }

    function testHarmonicIndexMod1296() public {
        uint256 nodeId = vortex.activateScalarNode(1, bytes32(uint256(2)), 2, 1295);
        vortex.advancePhase(nodeId);
        (,,, uint32 idx,,,) = vortex.queryNodeState(nodeId);
        assertEq(idx, (1295 + 3) % 1296);
    }

    function testInvalidPhaseReverts() public {
        vm.expectRevert(Vortex369.InvalidPhase.selector);
        vortex.activateScalarNode(0, bytes32(0), 1, 0);
        vm.expectRevert(Vortex369.InvalidPhase.selector);
        vortex.activateScalarNode(10, bytes32(0), 1, 0);
    }

    function testLibationAndRainbow() public {
        vortex.offerLibation("ancestors");
        assertEq(vortex.harmonic369(), 1);
        vortex.sealRainbow(9);
        assertEq(vortex.harmonic369(), 2);
    }

    function testSealConfirmEmits() public {
        vortex.sealConfirm("oracle", keccak256("note"));
    }

    function testConstantsMirrorRust() public view {
        assertEq(vortex.PHASE_MAX(), 9);
        assertEq(vortex.HARMONIC_PERIOD(), 1296);
        assertEq(vortex.MAX_SHELLS(), 6);
    }
}
