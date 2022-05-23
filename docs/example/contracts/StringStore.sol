//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

contract StringStore {
  string store;

  function save(string memory _store) external {
    store = _store;
  }

  function get() public view returns (string memory _store) {
    _store = store;
  }   
}