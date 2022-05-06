require("@nomiclabs/hardhat-ethers");
require('dotenv').config();

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
module.exports = {
  solidity: "0.8.4",
  networks: {
    barnacle: {
      url: process.env.BARNACLE_RPC_URL,
      chainId: 1008,
      accounts: [process.env.BARNACLE_DEPLOYER_PRIVATE_KEY, process.env.BARNACLE_ACCOUNT_PRIVATE_KEY]
    },
  }
};
