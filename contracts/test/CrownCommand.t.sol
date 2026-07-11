// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {Vortex369} from "../Vortex369.sol";
import {CrownCommand} from "../CrownCommand.sol";

contract CrownCommandTest is Test {
    Vortex369 vortex;
    CrownCommand crown;

    function setUp() public {
        vortex = new Vortex369(bytes32(uint256(1)), 1, address(0x369));
        crown = new CrownCommand(address(vortex));
    }

    function testSealWithScalar() public {
        uint256 nodeId = vortex.activateScalarNode(1, bytes32(uint256(99)), 6, 0);
        uint256 decreeId = crown.sealWithScalar(nodeId, "THE CROWN COMMANDS. REALITY OBEYS.");
        assertEq(decreeId, 1);

        (address commander, uint256 nId, bytes32 dHash, ) = crown.getDecree(decreeId);
        assertEq(commander, address(this));
        assertEq(nId, nodeId);
        assertEq(dHash, keccak256(bytes("THE CROWN COMMANDS. REALITY OBEYS.")));
    }

    function testSealRequiresActiveNode() public {
        vm.expectRevert(CrownCommand.NodeNotActive.selector);
        crown.sealWithScalar(999, "empty lattice");
    }

    function testEmptyDecreeReverts() public {
        uint256 nodeId = vortex.activateScalarNode(2, bytes32(uint256(1)), 1, 0);
        vm.expectRevert(CrownCommand.EmptyDecree.selector);
        crown.sealWithScalar(nodeId, "");
    }
}
