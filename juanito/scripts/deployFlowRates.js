const hre = require("hardhat");

async function main() {
    const [deployer] = await hre.ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);
  
    console.log("Account balance:", (await deployer.getBalance()).toString());
  
    /* FlowRates */
    const defaultPlatformRate = 1000;
    const FlowRates = await hre.ethers.getContractFactory("FlowRates");
    const rates = await FlowRates.deploy(defaultPlatformRate);
    console.log("FlowRates address:", rates.address);
    await rates.deployed();

  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });