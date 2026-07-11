// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title Vortex369 — Eternal Solar Kingdom Genesis + Scalar Node Registry
/// @notice Anchors the 0.000999 ETH sacrifice, 369/999 harmonics, and Tesla 369 scalar lattice.
/// @dev Additive only: sealRitual(string) + getGenesis() ABI preserved for solarking dry-run.
contract Vortex369 {
    uint8 public constant PHASE_MAX = 9;
    uint16 public constant HARMONIC_PERIOD = 1296;
    uint8 public constant MAX_SHELLS = 6;

    bytes32 public immutable genesisTxHash;
    uint256 public immutable genesisBlock;
    address public immutable sacrificeTo;

    uint256 public harmonic369;
    uint256 public harmonic999;

    struct ScalarNode {
        address sealer;
        uint8 phase;
        uint16 shellCoherence;
        uint32 harmonicIndex;
        bytes32 sealHash;
        uint64 activatedAt;
        bool active;
    }

    uint256 public nextNodeId = 1;
    mapping(uint256 => ScalarNode) private _nodes;

    event RitualSealed(address indexed sealer, string idm, uint256 harmonic369, uint256 harmonic999);
    event Legacy99Activated(bytes32 indexed genesisTxHash, uint256 blockNumber);

    event ScalarNodeActivated(
        address indexed sealer,
        uint256 indexed nodeId,
        uint8 phase,
        bytes32 sealHash
    );
    event LatticeHarmonic(
        uint256 indexed nodeId,
        uint256 shift,
        uint8 phase,
        uint256 timestamp
    );
    event LibationOffered(address indexed offerer, string target, uint256 h369);
    event RainbowVortex(address indexed sealer, uint8 intensity, uint256 h999);
    event FieldConfirm(address indexed sealer, string kind, bytes32 noteHash);

    error InvalidPhase();
    error InvalidShells();
    error InvalidIntensity();
    error NodeNotActive();
    error HarmonicIndexOOB();

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

    /// @notice Existing ritual seal — ABI stable for solarking seal --dry-run
    function sealRitual(string calldata idm) external {
        _bumpHarmonics();
        emit RitualSealed(msg.sender, idm, harmonic369, harmonic999);
    }

    function getGenesis()
        external
        view
        returns (bytes32 txHash, uint256 blockNumber, address to, uint256 h369, uint256 h999)
    {
        return (genesisTxHash, genesisBlock, sacrificeTo, harmonic369, harmonic999);
    }

    /// @notice Activate a scalar node (Tesla 369 lattice). Phase clamped to 1–9.
    function activateScalarNode(
        uint8 initialPhase,
        bytes32 sealHash,
        uint16 shellCoherence,
        uint32 harmonicIndex
    ) external returns (uint256 nodeId) {
        if (initialPhase < 1 || initialPhase > PHASE_MAX) revert InvalidPhase();
        if (shellCoherence > MAX_SHELLS) revert InvalidShells();
        if (harmonicIndex >= HARMONIC_PERIOD) revert HarmonicIndexOOB();

        nodeId = nextNodeId++;
        _nodes[nodeId] = ScalarNode({
            sealer: msg.sender,
            phase: initialPhase,
            shellCoherence: shellCoherence,
            harmonicIndex: harmonicIndex,
            sealHash: sealHash,
            activatedAt: uint64(block.timestamp),
            active: true
        });

        _bumpHarmonics();
        emit ScalarNodeActivated(msg.sender, nodeId, initialPhase, sealHash);
        emit LatticeHarmonic(nodeId, harmonicIndex, initialPhase, block.timestamp);
    }

    /// @notice Advance phase 1→9 then wrap to 1; bump harmonicIndex mod 1296.
    function advancePhase(uint256 nodeId) external {
        ScalarNode storage n = _nodes[nodeId];
        if (!n.active) revert NodeNotActive();

        uint8 next = n.phase >= PHASE_MAX ? 1 : n.phase + 1;
        n.phase = next;
        n.harmonicIndex = uint32((uint256(n.harmonicIndex) + 3) % HARMONIC_PERIOD);

        emit LatticeHarmonic(nodeId, n.harmonicIndex, n.phase, block.timestamp);
    }

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
        )
    {
        ScalarNode storage n = _nodes[nodeId];
        return (
            n.sealer,
            n.phase,
            n.shellCoherence,
            n.harmonicIndex,
            n.sealHash,
            n.activatedAt,
            n.active
        );
    }

    function isNodeActive(uint256 nodeId) external view returns (bool) {
        return _nodes[nodeId].active;
    }

    function offerLibation(string calldata target) external {
        _bumpHarmonics();
        emit LibationOffered(msg.sender, target, harmonic369);
    }

    function sealRainbow(uint8 intensity) external {
        if (intensity < 1 || intensity > PHASE_MAX) revert InvalidIntensity();
        _bumpHarmonics();
        emit RainbowVortex(msg.sender, intensity, harmonic999);
    }

    function sealConfirm(string calldata kind, bytes32 noteHash) external {
        emit FieldConfirm(msg.sender, kind, noteHash);
    }

    function _bumpHarmonics() internal {
        harmonic369 += 1;
        if (harmonic369 % 3 == 0) {
            harmonic999 += 1;
        }
    }
}
