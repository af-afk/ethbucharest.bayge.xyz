// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity 0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract ETHBucharestHackathonToken is ERC20, Ownable, ERC20Permit {
    constructor()
        ERC20("ETH Bucharest Hackathon Token", "EBHT")
        Ownable(msg.sender)
        ERC20Permit("ETH Bucharest Hackathon Token")
    {}

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
