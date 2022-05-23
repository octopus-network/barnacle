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
  
  const ERC20Contract = await hre.ethers.getContractFactory("MockErc20Token");
  const erc20 = await ERC20Contract.deploy();
  await erc20.deployed();

  /**
   * Transfer some ERC20s to accountA
   * */
  const transferTx = await erc20.transfer(accountA.address, "80000000000000000000");
  await transferTx.wait();

  const EscrowContract = await hre.ethers.getContractFactory("Escrow");
  const contract = await EscrowContract.deploy(erc20.address);
  await contract.deployed();

  console.log('Contracts deployed!');

  console.log('Deployed ERC20 contract address', erc20.address);
  await printToFile(`${networkName}-erc20`, erc20.address);

  console.log('Deployed Escrow Contract address', contract.address);
  await printToFile(`${networkName}-escrow`, contract.address);

  const erc20WithSigner = erc20.connect(accountA);
  const approveTx = await erc20WithSigner.approve(contract.address, "60000000000000000000");
  await approveTx.wait();

  console.log('Approved smart contract!');

  await testContract(contract, erc20, accountA);
}

async function testContract(contract, erc20, accountA) {
  const contractWithSigner = contract.connect(accountA);

  console.log('Executor Account Balance: ', (await hre.ethers.provider.getBalance(accountA.address)).toString());
  console.log('Executor ERC20 Account Balance: ', (await erc20.balanceOf(accountA.address)).toString());

  const submitEscrowTx = await contractWithSigner.submitEscrow("30000000000000000000");
  await submitEscrowTx.wait();
  console.log('Executor ERC20 Account Balance: ', (await erc20.balanceOf(accountA.address)).toString());

  const retrieveEscrowTx = await contractWithSigner.retrieveEscrow();
  await retrieveEscrowTx.wait();
  console.log('Executor ERC20 Account Balance: ', (await erc20.balanceOf(accountA.address)).toString());
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