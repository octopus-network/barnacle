// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0 <0.9.0;

/**
 * @title Pallet OctopusAppchain Interface
 *
 * The interface through which solidity contracts will interact with pallet-octopus-appchain
 * Address :    0x0000000000000000000000000000000000000803
 */

interface OctopusAppchain {
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
	 * burn asset
     * Selector: 0xe1a6d74c
	 *
	 * @param receiver_id The receiver address in near
	 * @param amount The amount to cross 
	 * @param asset_id The id of asset to cross 
	 *
     */
    function burn_asset(uint32 asset_id, uint128 amount, bytes memory receiver_id) external;


    /**
	 * lock nft 
     * Selector: 0x6ad31c32 
	 *
	 * @param receiver_id The receiver address in near
	 * @param instance The instance id of nft 
	 * @param class The class id of nft 
	 *
     */
    function lock_nft(uint128 class, uint128 instance, bytes memory receiver_id) external;
}
