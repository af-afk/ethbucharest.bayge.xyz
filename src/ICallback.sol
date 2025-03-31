// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface ICallback {
    function prove(bytes32 hash, uint32 from) external returns (uint32 lower, uint32 upper);
}
