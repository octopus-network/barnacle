//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockErc20Token is ERC20 {
  constructor() ERC20("MockErc20Token", "DAI") {
    _mint(msg.sender, 100000000 * 10 ** decimals());
  }
}