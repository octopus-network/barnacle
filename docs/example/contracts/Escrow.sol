//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract Escrow {
  IERC20 public _token;

  constructor(address ERC20Address) {
    deposit_count = 0;
    _token = IERC20(ERC20Address);
  }

  uint256 deposit_count;

  function retrieveEscrow() external {
    require(deposit_count != 0, "Escrow deposit is zero.");

    require(_token.transfer(msg.sender, deposit_count), "Escrow withdrawal failed!");
    
    deposit_count = 0;
  }

  function submitEscrow(uint256 amount) external {
    require(amount != 0, "Escrow amount cannot be zero.");

    require(_token.transferFrom(msg.sender, address(this), amount), "Escrow deposit failed!");

    deposit_count += amount;
  }   
}