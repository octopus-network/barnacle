// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

/// @title ERC721 Vault
/// @notice Holds ERC721 Tokens on behalf of ERC721App.
contract ERC721Vault is Ownable {

    /// @dev Emitted when an ERC721 token is deposited.
    /// @param account The address of the ERC721App contract.
    /// @param sender The address of the sender.
    /// @param token The address of the ERC721 token.
    /// @param tokenId The token id being deposited.
    event Deposit(
        address account,
        address sender,
        address token,
        uint256 tokenId
    );

    /// @dev Emitted when an ERC721 token is withdrawn.
    /// @param account The address of the ERC721App contract.
    /// @param recipient The address of the receiver.
    /// @param token The address of the ERC721 token.
    /// @param tokenId The token id being withdrawn.
    event Withdraw(
        address account,
        address recipient,
        address token,
        uint256 tokenId
    );

    /// @dev Accepts an ERC721 Token from the caller.
    /// @param _sender The address of the sender.
    /// @param _token The address of the Token.
    /// @param _tokenId The token id being deposited.
    function deposit(
        address _sender,
        address _token,
        uint256 _tokenId
    ) external onlyOwner {
        IERC721(_token).safeTransferFrom(_sender, address(this), _tokenId);
        emit Deposit(msg.sender, _sender, _token, _tokenId);
    }

    /// @dev Returns an ERC721 token to the receipient.
    /// @param _recipient The address of the receiver.
    /// @param _token The address of the Token.
    /// @param _tokenId The token id being deposited.
    function withdraw(
        address _recipient,
        address _token,
        uint256 _tokenId
    ) external onlyOwner {
        IERC721(_token).safeTransferFrom(address(this), _recipient, _tokenId);
        emit Withdraw(msg.sender, _recipient, _token, _tokenId);
    }
}
