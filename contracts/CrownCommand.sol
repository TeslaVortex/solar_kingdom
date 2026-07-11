// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {IVortex369} from "./IVortex369.sol";

/// @title CrownCommand — append-only decree ledger bound to scalar nodes
/// @notice Stores decree hashes + nodeId references; full decree text may stay off-chain.
contract CrownCommand {
    IVortex369 public immutable vortex;

    struct Decree {
        address commander;
        uint256 nodeId;
        bytes32 decreeHash;
        uint64 sealedAt;
        bool exists;
    }

    uint256 public nextDecreeId = 1;
    mapping(uint256 => Decree) private _decrees;

    event DecreeSealed(
        address indexed commander,
        uint256 indexed decreeId,
        uint256 indexed nodeId,
        bytes32 decreeHash,
        uint256 timestamp
    );

    error NodeNotActive();
    error EmptyDecree();
    error DecreeNotFound();

    constructor(address vortex369) {
        vortex = IVortex369(vortex369);
    }

    /// @notice Bind a Crown decree to an active scalar node on Vortex369.
    function sealWithScalar(uint256 nodeId, string calldata decree)
        external
        returns (uint256 decreeId)
    {
        if (bytes(decree).length == 0) revert EmptyDecree();
        if (!vortex.isNodeActive(nodeId)) revert NodeNotActive();

        bytes32 decreeHash = keccak256(bytes(decree));
        decreeId = nextDecreeId++;
        _decrees[decreeId] = Decree({
            commander: msg.sender,
            nodeId: nodeId,
            decreeHash: decreeHash,
            sealedAt: uint64(block.timestamp),
            exists: true
        });

        emit DecreeSealed(msg.sender, decreeId, nodeId, decreeHash, block.timestamp);
    }

    function getDecree(uint256 decreeId)
        external
        view
        returns (
            address commander,
            uint256 nodeId,
            bytes32 decreeHash,
            uint64 sealedAt
        )
    {
        Decree storage d = _decrees[decreeId];
        if (!d.exists) revert DecreeNotFound();
        return (d.commander, d.nodeId, d.decreeHash, d.sealedAt);
    }
}
