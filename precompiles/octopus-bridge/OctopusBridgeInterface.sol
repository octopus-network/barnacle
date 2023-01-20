// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0 <0.9.0;

/**
 * @title Pallet OctopusAppchain Interface
 *
 * The interface through which solidity contracts will interact with pallet-octopus-bridge
 * Address :    0x0000000000000000000000000000000000000803
 */

interface OctopusBridge {
    /**
	 * lock native token 
     * Selector: 0xd8f7c836
	 *
	 * @param receiver_id The receiver address in near
	 * @param amount The amount to cross 
	 *
     */
    function lock(uint256 amount, bytes memory receiver_id) external;


    /**
	 * burn nep141 
     * Selector: 0x749186c9
	 *
	 * @param receiver_id The receiver address in near
	 * @param amount The amount to cross 
	 * @param asset_id The id of asset to cross 
	 *
     */
    function burn_nep141(uint32 asset_id, uint256 amount, bytes memory receiver_id) external;


    /**
	 * lock nonfungible 
     * Selector: 0x7c1978c0
	 *
	 * @param receiver_id The receiver address in near
	 * @param item The item id of nft 
	 * @param collection The collection id of nft 
	 *
     */
    function lock_nonfungible(uint256 collection, uint256 item, bytes memory receiver_id) external;
	
}