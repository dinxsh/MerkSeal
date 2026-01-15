// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console} from "forge-std/Script.sol";
import {MerkleBatchRegistry} from "../MerkleBatchRegistry.sol";

/**
 * @title Deploy Script for MerkleBatchRegistry
 * @notice Foundry deployment script for Mantle testnet/mainnet
 * @dev Run with: forge script script/Deploy.s.sol --rpc-url $MANTLE_RPC_URL --broadcast --private-key $PRIVATE_KEY
 */
contract DeployScript is Script {
    function run() external {
        vm.startBroadcast();

        MerkleBatchRegistry registry = new MerkleBatchRegistry();

        vm.stopBroadcast();

        // Log deployment info
        console.log("==============================================");
        console.log("MerkleBatchRegistry deployed to:", address(registry));
        console.log("==============================================");
        console.log("Save this address to your .env file:");
        console.log("MERKLE_BATCH_REGISTRY_ADDRESS=%s", address(registry));
        console.log("==============================================");
    }
}
