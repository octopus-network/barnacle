# Solidity contracts for supporting cross-chain transfer of ERC721 tokens in appchain

## Setup

* Deploy `ERC721Vault.sol`.
  * The address that perform the deployment will be the owner of this contract.
* Deploy `ERC721App.sol`.
  * By passing the address of `ERC721Vault` contract and precompile contracts.
  * The address that perform the deployment will be granted the admin role of this contract. This address can be used to grant other roles in the following steps.
* Change the owner of `ERC721Vault` contract to the address of `ERC721App` contract.
  * Call `transferOwnership` of `ERC721Vault` contract by the owner account of this contract.
* Grant role to address(es) that can use `register` function of `ERC721App` contract.
  * Call `grantRole` of `ERC721App` contract by passing the role identifier (`keccak256("REGISTRATION_ROLE")`) and the address.
* Grant role to address(es) that can use `unlock` function of `ERC721App` contract.
  * Call `grantRole` of `ERC721App` contract by passing the role identifier (`keccak256("TOKEN_REDEEM_ROLE")`) and the address.
