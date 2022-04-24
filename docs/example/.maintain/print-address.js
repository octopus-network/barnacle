const hre = require('hardhat');

require('dotenv').config();

async function main() {
    const accounts = await hre.ethers.getSigners();

    console.log("Deployer private key account address:", accounts[0].address);
    console.log("Executor private key account address:", accounts[1].address);
}

// We recommend this pattern to be able to use
// async/await everywhere and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });