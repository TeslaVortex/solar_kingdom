// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface IVortex369 {
    function isNodeActive(uint256 nodeId) external view returns (bool);

    function queryNodeState(uint256 nodeId)
        external
        view
        returns (
            address sealer,
            uint8 phase,
            uint16 shellCoherence,
            uint32 harmonicIndex,
            bytes32 sealHash,
            uint64 activatedAt,
            bool active
        );
}
