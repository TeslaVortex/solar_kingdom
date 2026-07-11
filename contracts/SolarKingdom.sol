// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title SolarKingdom — soul-bound badges for scalar lattice milestones
/// @notice Minimal non-transferable token IDs (not full ERC-721) — gas-aware SBT.
contract SolarKingdom {
    enum BadgeKind {
        NodeGuardian, // 369 Node Guardian
        LatticePhase, // Lattice Phase Complete
        RainbowLattice, // Double-Edged Rainbow Lattice
        Legacy999 // Legacy 99→999
    }

    struct Badge {
        address soul;
        BadgeKind kind;
        uint256 nodeId;
        uint64 mintedAt;
        bool exists;
    }

    address public immutable minter; // typically deployer / ops
    address public vortex;

    uint256 public nextBadgeId = 1;
    mapping(uint256 => Badge) private _badges;
    mapping(address => mapping(uint8 => bool)) public hasBadgeKind;

    event BadgeMinted(
        address indexed soul,
        uint256 indexed badgeId,
        BadgeKind kind,
        uint256 nodeId,
        uint256 timestamp
    );

    error NotMinter();
    error AlreadyHasKind();
    error SoulBound();
    error BadgeNotFound();

    constructor(address _vortex) {
        minter = msg.sender;
        vortex = _vortex;
    }

    modifier onlyMinter() {
        if (msg.sender != minter) revert NotMinter();
        _;
    }

    /// @notice Mint a soul-bound badge (transfer disabled by design — no transfer function).
    function mintBadge(address soul, BadgeKind kind, uint256 nodeId)
        external
        onlyMinter
        returns (uint256 badgeId)
    {
        uint8 k = uint8(kind);
        if (hasBadgeKind[soul][k]) revert AlreadyHasKind();

        badgeId = nextBadgeId++;
        _badges[badgeId] = Badge({
            soul: soul,
            kind: kind,
            nodeId: nodeId,
            mintedAt: uint64(block.timestamp),
            exists: true
        });
        hasBadgeKind[soul][k] = true;

        emit BadgeMinted(soul, badgeId, kind, nodeId, block.timestamp);
    }

    function getBadge(uint256 badgeId)
        external
        view
        returns (address soul, BadgeKind kind, uint256 nodeId, uint64 mintedAt)
    {
        Badge storage b = _badges[badgeId];
        if (!b.exists) revert BadgeNotFound();
        return (b.soul, b.kind, b.nodeId, b.mintedAt);
    }

    /// @dev Explicit soul-bound guard if future ERC-721 adapters are added.
    function transferFrom(address, address, uint256) external pure {
        revert SoulBound();
    }
}
