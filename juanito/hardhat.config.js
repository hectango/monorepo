require("@nomicfoundation/hardhat-toolbox");
require('dotenv').config()

const { HARRY_PRIVATE_KEY, MUMBAI_PRIVATE_KEY } = process.env;

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
      gasPrice: 20000000000
    }
  }
};
