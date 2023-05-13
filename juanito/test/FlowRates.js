const { loadFixture } = require("@nomicfoundation/hardhat-network-helpers");
const { expect } = require("chai");

describe("FlowRates contract", function () {
  async function deployRatesFixture() {
    const FlowRates = await ethers.getContractFactory("FlowRates");
    const [owner, addr1, addr2] = await ethers.getSigners();

    const rates = await FlowRates.deploy();

    await rates.deployed();

    // Fixtures can return anything you consider useful for your tests
    return { FlowRates, rates, owner, addr1, addr2 };
  }

  it("Should assign the total supply of tokens to the owner", async function () {
    const { rates, addr1 } = await loadFixture(deployRatesFixture);
    const videoID = "0d58d9d181fb619cce31def2509af262";
    const videoRate = 1234;
    await rates.connect(addr1).uploadVideo(videoID, videoRate);
    const rate = await rates.getRate(addr1.address, videoID);
    expect(rate).to.equal(videoRate);
  });

  /*
  it("Should transfer tokens between accounts", async function () {
    const { hardhatToken, owner, addr1, addr2 } = await loadFixture(
      deployTokenFixture
    );

    // Transfer 50 tokens from owner to addr1
    await expect(
      hardhatToken.transfer(addr1.address, 50)
    ).to.changeTokenBalances(hardhatToken, [owner, addr1], [-50, 50]);

    // Transfer 50 tokens from addr1 to addr2
    // We use .connect(signer) to send a transaction from another account
    await expect(
      hardhatToken.connect(addr1).transfer(addr2.address, 50)
    ).to.changeTokenBalances(hardhatToken, [addr1, addr2], [-50, 50]);
  });
  */
});