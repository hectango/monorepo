const hre = require("hardhat");

async function main() {

  const [loader] = await hre.ethers.getSigners();
  const provider = await new ethers.providers.JsonRpcProvider('https://rpc.ankr.com/polygon_mumbai');
  let nonceavo = await provider.getTransactionCount(loader.address);

  console.log("Nonce:", nonceavo);

    console.log("Initial video load with Viewer account:", loader.address);
    // let tempNonce = provider.getTransactionCount(loader.address);
    // console.log("Initial nonce: ", tempNonce);
    console.log("Account balance:", (await loader.getBalance()).toString());
    /* FlowRates load */
    //const deployedFlowRates = "0x081C1383Db2411E2a8Fb5EDf9aD48fC8F1E610F3";
    const deployedFlowRates = "0x900Ed131ac712f60353305d076C0A50058239A86";
    
    console.log("FlowRates address:", deployedFlowRates);
    const FlowRates = await hre.ethers.getContractFactory("FlowRates");
    const ratesInstance = await FlowRates.attach(deployedFlowRates);
    // Video 1
    const videoID1 = "0d58d9d181fb619cce31def2509af262";
    const videoRate1 = 5787037037037; // wei per second
    console.log("Video ID1:", videoID1);
    console.log("Video rate1:", videoRate1);
    await ratesInstance.uploadVideo(videoID1, videoRate1, {nonce: ++nonceavo});
    
    // Video 2
    const videoID2 = "bacb61daab7ebcfdacf58cb158bb34a2";
    const videoRate2 = 15787037037037; // wei per second
    console.log("Video ID2:", videoID2);
    console.log("Video rate2:", videoRate2);
    await ratesInstance.uploadVideo(videoID2, videoRate2, {nonce: ++nonceavo});
    
    // Video 3
    const videoID3 = "8862758380f187fa168a44fb971a96bd";
    const videoRate3 = 115787037037037; // wei per second
    console.log("Video ID3:", videoID3);
    console.log("Video rate3:", videoRate3);
    await ratesInstance.uploadVideo(videoID3, videoRate3, {nonce: ++nonceavo});

    console.log("FlowRates loaded!");
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });