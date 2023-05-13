require("@nomicfoundation/hardhat-toolbox");
require('dotenv').config()

const { MUMBAI_PRIVATE_KEY } = process.env;

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.18",
  networks: {
    mumbai: {
      url: `https://rpc.ankr.com/polygon_mumbai`,
      accounts: [MUMBAI_PRIVATE_KEY]
    }
  }
};
