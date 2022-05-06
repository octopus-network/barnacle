const fs = require('fs');
const path = require('path');
const hre = require('hardhat');
const { open } = require('fs/promises');

require('dotenv').config();

async function main() {
  const networkName = hre.network.name;
  const networkUrl = hre.network.config.url;
  console.log('Deploying to network', networkName, networkUrl);
  
  const accounts = await hre.ethers.getSigners();
  const deployer = accounts[0];
  const accountA = accounts[1];

  console.log("Deploying contracts with the account:", deployer.address);
  console.log("Executor contracts with the account:", accountA.address);
  
  const StringStoreContract = await hre.ethers.getContractFactory("StringStore");
  const contract = await StringStoreContract.deploy();
  await contract.deployed();

  console.log('Contracts deployed!');

  console.log('Deployed StringStore Contract address', contract.address);
  await printToFile(`${networkName}-string-store`, contract.address);

  await testContract(contract, accountA);
}

async function testContract(contract, accountA) {
  const contractWithSigner = contract.connect(accountA);

  const submitStringStoreTx = await contractWithSigner.save("Hello World!");
  await submitStringStoreTx.wait();

  const retrieveStringStoreTx = await contractWithSigner.get();
  console.log('Retrieve String Store: ', retrieveStringStoreTx);
}

async function printToFile(filename, text) {
  const dest = 'deployed-addresses';
  if (!fs.existsSync(dest)) fs.mkdirSync(dest); 
  let fh = await open(path.join(dest, filename), 'w');
  await fh.writeFile(String(text));
  await fh.close();
}

// We recommend this pattern to be able to use
// async/await everywhere and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });