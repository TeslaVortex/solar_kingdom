// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title Vortex369 — Eternal Solar Kingdom Genesis Registry
/// @notice Anchors the 0.000999 ETH sacrifice and tracks 369/999 harmonics
contract Vortex369 {
    bytes32 public immutable genesisTxHash;
    uint256 public immutable genesisBlock;
    address public immutable sacrificeTo;
    uint256 public harmonic369;
    uint256 public harmonic999;

    event RitualSealed(address indexed sealer, string idm, uint256 harmonic369, uint256 harmonic999);
    event Legacy99Activated(bytes32 indexed genesisTxHash, uint256 blockNumber);

    constructor(
        bytes32 _genesisTxHash,
        uint256 _genesisBlock,
        address _sacrificeTo
    ) {
        genesisTxHash = _genesisTxHash;
        genesisBlock = _genesisBlock;
        sacrificeTo = _sacrificeTo;
        emit Legacy99Activated(_genesisTxHash, _genesisBlock);
    }

    function sealRitual(string calldata idm) external {
        harmonic369 += 1;
        if (harmonic369 % 3 == 0) {
            harmonic999 += 1;
        }
        emit RitualSealed(msg.sender, idm, harmonic369, harmonic999);
    }

    function getGenesis()
        external
        view
        returns (bytes32 txHash, uint256 blockNumber, address to, uint256 h369, uint256 h999)
    {
        return (genesisTxHash, genesisBlock, sacrificeTo, harmonic369, harmonic999);
    }
}