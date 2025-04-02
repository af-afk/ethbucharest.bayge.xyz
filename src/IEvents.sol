// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface IEvents {
    event Deployed(address indexed deployment);

    event Registered(
        address indexed addr,
        address indexed recipient,
        string info
    );

    event Checked(
        address indexed addr,
        address indexed points,
        uint64 indexed gas,
        bytes32 word
    );

    event NewWinner(
        address indexed addr,
        address indexed points,
        uint64 indexed gas
    );

    event CancelTookPlace(address indexed victim);

    event Concluded();

    event WinnerPaidOut(
        address indexed leader,
        uint256 indexed amount
    );
}
