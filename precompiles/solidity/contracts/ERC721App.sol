// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/IERC721Metadata.sol";
import "../octopus-bridge/OctopusBridgeInterface.sol";
import "../octopus-uniques/OctopusUniquesInterface.sol";
import "./ERC721Vault.sol";

contract ERC721App is AccessControl {
    bytes32 public constant REGISTRATION_ROLE = keccak256("REGISTRATION_ROLE");
    bytes32 public constant TOKEN_REDEEM_ROLE = keccak256("TOKEN_REDEEM_ROLE");

    ERC721Vault public immutable vault;
    OctopusBridge public immutable octopusBridge;
    OctopusUniques public immutable octopusUniques;

    // Registered ERC721 tokens.
    // The value is collection id in OctopusUniques.
    mapping(address => uint256) public tokens;

    event Registered(
        address token,
        uint256 collectionId,
        address admin
    );

    event Locked(
        address token,
        address sender,
        uint256 tokenId,
        bytes recipient
    );

    event Unlocked(
        address token,
        bytes32 sender,
        address recipient,
        uint256 tokenId
    );

    constructor(
        ERC721Vault _erc721vault,
        OctopusBridge _octopusBridge,
        OctopusUniques _octopusUniques
    ) {
        vault = _erc721vault;
        octopusBridge = _octopusBridge;
        octopusUniques = _octopusUniques;
        //
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _setRoleAdmin(REGISTRATION_ROLE, DEFAULT_ADMIN_ROLE);
        _setRoleAdmin(TOKEN_REDEEM_ROLE, DEFAULT_ADMIN_ROLE);
    }

    /**
     * @dev Register an ERC721 token contract in this app, also enable cross-chain transfer.
     *
     * The caller must have REGISTRATION_ROLE's role.
     */
    function register(
        address _token,
        uint256 _collectionId,
        address _admin
    ) public onlyRole(REGISTRATION_ROLE) {
        require(
            octopusUniques.create_collection(_collectionId, _admin),
            "Failed to create collection in OctopusUniques."
        );
        tokens[_token] = _collectionId;

        emit Registered(_token, _collectionId, _admin);
    }

    /**
     * @dev Lock an ERC721 token in vault contract, and start cross-chain transfer of the token.
     *
     * The ERC721 token contract must be registered before calling this function.
     * The caller must be the owner of the ERC721 token,
     * and must have approved the token to vault contract before calling this function.
     */
    function lock(
        address _token,
        uint256 _tokenId,
        bytes calldata _recipient
    ) external {
        require(tokens[_token] != 0, "The contract is not registered.");

        vault.deposit(msg.sender, _token, _tokenId);

        require(
            octopusUniques.mint(tokens[_token], _tokenId, address(vault)),
            "Failed to mint token in OctopusUniques."
        );

        require(
            octopusUniques.set_metadata(
                tokens[_token],
                _tokenId,
                IERC721Metadata(_token).tokenURI(_tokenId)
            ),
            "Failed to set metadata of the token in OctopusUniques."
        );

        octopusBridge.lock_nonfungible(tokens[_token], _tokenId, _recipient);

        emit Locked(_token, msg.sender, _tokenId, _recipient);
    }

    /**
     * @dev Unlock an ERC721 token in vault contract and transfer it to the given recipient.
     *
     * The caller must have TOKEN_REDEEM_ROLE's role.
     */
    function unlock(
        address _token,
        bytes32 _sender,
        address _recipient,
        uint256 _tokenId
    ) external onlyRole(TOKEN_REDEEM_ROLE) {
        octopusUniques.burn(tokens[_token], _tokenId);

        vault.withdraw(_recipient, _token, _tokenId);

        emit Unlocked(_token, _sender, _recipient, _tokenId);
    }
}
