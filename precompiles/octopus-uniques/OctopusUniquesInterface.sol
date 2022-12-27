// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

/**
 * @title Pallet uniques Interface
 *
 * The interface through which solidity contracts will interact with pallet-uniques
 * Address :    0x0000000000000000000000000000000000000805
 */
interface OctopusUniques {
	/**
	 * create nft collection in uniques pallet
	 * Selector:0x12d69575 
	 * 
	 * @param collection The collection of nft
	 * @param admin The admin of this collection. 
	 */
    function create_collection(uint256 collection, address admin) external returns (bool);

	/**
	 * mint nft in uniques pallet
	 * Selector:0xe7d3fe6b 
	 * 
	 * @param collection The collection of nft
	 * @param item The item of nft
	 * @param owner The owner of nft
	 * 
	 */
    function mint(uint256 collection, uint256 item, address owner) external returns (bool);
	

	/**
	 * Set the metadata for an item
	 * Selector: 0x527a0032 
	 * 
	 * @param collection The collection of nft
	 * @param item The item of nft
	 * @param uri The token uri of nft
	 * 
	 */
	function set_metadata(uint256 collection, uint256 item, string memory uri) external returns (bool);

	/**
	 * burn nft in uniques pallet
	 * Selector: 0xb390c0ab
	 * 
	 * @param collection The collection of nft
	 * @param item The item of nft
	 * 
	 */
    function burn(uint256 collection, uint256 item) external returns (bool);
}
