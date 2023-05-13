const hre = require("hardhat");

async function main() {
    const [loader] = await hre.ethers.getSigners();

    console.log("Initial video load with account:", loader.address);
  
    console.log("Account balance:", (await loader.getBalance()).toString());
  
    /* FlowRates load */
    //const deployedFlowRates = "0x081C1383Db2411E2a8Fb5EDf9aD48fC8F1E610F3";
    const deployedFlowRates = "0x900Ed131ac712f60353305d076C0A50058239A86";
    const videoID = "0d58d9d181fb619cce31def2509af262";
    const rate = 5787037037037; // wei per second

    console.log("FlowRates address:", deployedFlowRates);
    console.log("VideoID:", videoID);
    console.log("Video rate:", rate);
    const FlowRates = await hre.ethers.getContractFactory("FlowRates");
    const ratesInstance = await FlowRates.attach(deployedFlowRates);
    await ratesInstance.uploadVideo(videoID, rate);
    console.log("FlowRates loaded!");
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });