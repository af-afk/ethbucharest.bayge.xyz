// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface IProver {
    function register(address _contract, address points, string memory info) external;

    function tokenAddr() external view returns (address);

    function prove(bytes32 hash, uint32 start) external view returns (uint32 lower, uint32 upper);

    function check(address _contract, bytes32 word, address points) external returns (uint64);

    function cancel(address victim) external;

    function conclude() external;

    function tokenAmountOwed() external view returns (uint256);

    function currentLeaderSolution() external view returns (uint64 gas, address pointsRecipient);
}
