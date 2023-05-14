import { ethers } from "ethers";
import contractABI from '../abis/fdaix.json';
import flowRatesABIS from '../abis/FlowRates.json';

export async function connectWallet() {
    try {
        const { ethereum } = window;

        if (!ethereum) {
            return;
        }
        const accounts = await ethereum.request({
            method: "eth_requestAccounts"
        });
        console.log("Connected", accounts[0]);
        //console.log(0/1);
        return accounts[0];
        // Setup listener! This is for the case where a user comes to our site
        // and connected their wallet for the first time.
        // setupEventListener()
    } catch (error) {
        console.log(error);

        throw new Error("Error connecting the wallet");
    }
};

export async function checkIfWalletIsConnected () {
    console.log("checking if wallet is connected");
    const { ethereum } = window;

    if (!ethereum) {
        console.log("Make sure you have metamask!");
        return;
    } else {
        console.warn("We have the ethereum object", ethereum);
    }

    const accounts = await window.ethereum.request({ method: "eth_accounts" });
    const chain = await window.ethereum.request({ method: "eth_chainId" });
    let chainId = chain;
    console.log("chain ID:", chain);
    console.log("global Chain Id:", chainId);
    if (accounts.length !== 0) {
        console.log("Found an authorized account:", accounts[0]);

        return accounts[0];
    }

    throw new Error("No authorized account found");
};

export async function getBalance(account) {
    return formatBalance(await window.ethereum.request({
        method: "eth_getBalance",
        params: [account, "latest"],
    }));
}

function formatBalance(rawBalance) {
    const balance = (parseInt(rawBalance) / 1000000000000000000).toFixed(2)
    return balance
}

export async function getBalanceToken(contractAddress) {
    await ethereum.request({ method: 'eth_requestAccounts' });

    //Now you need its provider
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    //To get signer and its address use function below:
    const signer = provider.getSigner();
    const signerAddress = await signer.getAddress();

    const contract = await new ethers.Contract(contractAddress, contractABI.result, signer);

    const balance = await contract.balanceOf('0x51159613973463Fd808718bb1D86056E16bd12E3');
    const formatedBalance = ethers.utils.formatUnits(balance, 18)

    return parseFloat(formatedBalance).toFixed(3);
}

export async function getFlowRate(contractAddress, ownerAddress, videoId) {
    await ethereum.request({ method: 'eth_requestAccounts' });

    //Now you need its provider
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    //To get signer and its address use function below:
    const signer = provider.getSigner();

    const contract = await new ethers.Contract(contractAddress, flowRatesABIS.abi, signer);

    const rate = await contract.getRate(ownerAddress, videoId);
    const formattedRate = ethers.utils.formatUnits(rate, 0);
    console.log(formattedRate);

    return formattedRate

}