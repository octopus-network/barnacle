// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

/**
 * @title Pallet Session Interface
 *
 * The interface through which solidity contracts will interact with pallet-session
 * Address :    0x0000000000000000000000000000000000000804
 */

interface OctopusSession {
    /**
	 * set keys 
     * Selector: 0xd8be245a
	 *
	 * @param keys The session key will set
	 * @param proof The proof to set 
	 *
     */
    function set_keys(bytes memory keys, bytes memory proof) external;
}