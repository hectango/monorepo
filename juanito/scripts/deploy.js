async function main() {
    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);
  
    console.log("Account balance:", (await deployer.getBalance()).toString());
  
    /* Token */
    const Token = await ethers.getContractFactory("Token");
        // Token: uint256 _initialAmount, string memory _tokenName, uint8 _decimalUnits, string  memory _tokenSymbol)
    const token = await Token.deploy(10000000000000, "Hectango", 18, "HTG");
    await token.deployed();
    console.log("Token address:", token.address);
    
    /* FlowRates */
    const FlowRates = await ethers.getContractFactory("FlowRates");
    const rates = await FlowRates.deploy();
    await rates.deployed();
    console.log("FlowRates address:", rates.address);


  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });