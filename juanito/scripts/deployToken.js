const hre = require("hardhat");

async function main() {
    const [deployer] = await hre.ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);
  
    console.log("Account balance:", (await deployer.getBalance()).toString());
  
    /* Token */
    const Token = await hre.ethers.getContractFactory("Token");
    const token = await Token.deploy(10000000000000, "Hectango", 18, "HTG");
    await token.deployed();
    console.log("Token address:", token.address);
    
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });