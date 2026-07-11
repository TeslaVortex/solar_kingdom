// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {Vortex369} from "../Vortex369.sol";
import {SolarKingdom} from "../SolarKingdom.sol";

contract SolarKingdomTest is Test {
    Vortex369 vortex;
    SolarKingdom kingdom;
    address soul = address(0xBEEF);

    function setUp() public {
        vortex = new Vortex369(bytes32(uint256(1)), 1, address(0x369));
        kingdom = new SolarKingdom(address(vortex));
    }

    function testMintNodeGuardian() public {
        uint256 nodeId = vortex.activateScalarNode(3, bytes32(uint256(7)), 6, 100);
        uint256 badgeId = kingdom.mintBadge(soul, SolarKingdom.BadgeKind.NodeGuardian, nodeId);
        assertEq(badgeId, 1);
        assertTrue(kingdom.hasBadgeKind(soul, uint8(SolarKingdom.BadgeKind.NodeGuardian)));

        (address s, SolarKingdom.BadgeKind k, uint256 n, ) = kingdom.getBadge(badgeId);
        assertEq(s, soul);
        assertEq(uint8(k), uint8(SolarKingdom.BadgeKind.NodeGuardian));
        assertEq(n, nodeId);
    }

    function testSoulBoundNoTransfer() public {
        kingdom.mintBadge(soul, SolarKingdom.BadgeKind.Legacy999, 0);
        vm.expectRevert(SolarKingdom.SoulBound.selector);
        kingdom.transferFrom(soul, address(this), 1);
    }

    function testDuplicateKindReverts() public {
        kingdom.mintBadge(soul, SolarKingdom.BadgeKind.RainbowLattice, 1);
        vm.expectRevert(SolarKingdom.AlreadyHasKind.selector);
        kingdom.mintBadge(soul, SolarKingdom.BadgeKind.RainbowLattice, 2);
    }
}
