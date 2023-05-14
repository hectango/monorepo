require("@nomicfoundation/hardhat-toolbox");
require('dotenv').config()

const { ALCHEMY_API_KEY, GNOSIS_PRIVATE_KEY, HARRY_PRIVATE_KEY, MUMBAI_PRIVATE_KEY } = process.env;

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.18",
  networks: {
    mumbai: {
      url: `https://rpc.ankr.com/polygon_mumbai`,
      accounts: [MUMBAI_PRIVATE_KEY],
      gasPrice: 20000000000
    }, 
    harry: {
      url: `https://rpc.ankr.com/polygon_mumbai`,
      accounts: [HARRY_PRIVATE_KEY],
      gasPrice: 40000000000
    }, 
    gnosis: {
      url: `https://rpc.gnosischain.com`,
      accounts: [GNOSIS_PRIVATE_KEY],
      gasPrice: 40000000000
    }, 
    optimismgoerli: {
      url: `https://opt-goerli.g.alchemy.com/v2/${ALCHEMY_API_KEY}`,
      accounts: [MUMBAI_PRIVATE_KEY],
      gasPrice: 20000000000
    }
  }
};
