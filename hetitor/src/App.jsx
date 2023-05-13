import "./App.css";

import {Fragment, useEffect, useRef, useState} from 'react';
import {checkIfWalletIsConnected, connectWallet} from "./utils/wallet";
import {createNewFlow, deleteExistingFlow} from "./utils/superfluid";
import ReactHlsPlayer from 'react-hls-player';

const receiver = '0x2819Db886a1C12C74Edf3514F831dfA00bFc101F';

const flowRate = "5787037037037";

/*
    return () => {
      window.ethereum?.removeListener('accountsChanged', refreshAccounts)
      window.ethereum?.removeListener("chainChanged", refreshChain)
}*/

function App() {
    const [flow, setFlow] = useState(null);
    const [isLoading, setLoading] = useState(false);
    const [errorMessage, setErrorMessage] = useState(null);
    const [isConnectingToWallet, setIsConnectingToWallet] = useState(true);
    const [currentAccount, setCurrentAccount] = useState();

    const player = useRef(null);

    useEffect(() => {
        checkIfWalletIsConnected().then((account) => {
            setCurrentAccount(account);
        }).catch(() => setIsConnectingToWallet(false));
    }, []);

    useEffect(() => {
        setIsConnectingToWallet(false);
        setErrorMessage(null);
    }, [currentAccount]);

    useEffect(() =>{
        setLoading(true);
        deleteExistingFlow(receiver).then(() => setLoading(false)).catch(console.error);
    }, []);

    function _onPlay() {
        setLoading(true);
        createNewFlow(receiver, flowRate).then((flow) => {
            setFlow(flow);
            player.current.play();
            setLoading(false);
        }).catch(console.error)
    }

    function _onPause() {
        player.current.pause();
        setLoading(true);
        deleteExistingFlow(receiver).then(() => {
            setFlow(null);
            setLoading(false)
        }).catch(console.error);
    }

    function render() {
        return (
            <Fragment>
                {_renderOverlay()}
                {_renderErrorMessage()}
                {_renderConnectButton()}
                {_renderVideoPlayer()}
                <button onClick={_onPlay}>Play</button>
                <button onClick={_onPause}>Stop</button>
            </Fragment>
        );
    }

    function _renderOverlay() {
        if (isLoading) {
            return <div className={"overlay"}>
                <p>We are communicating with the blockchain.....</p>
            </div>
        }
    }

    function _renderErrorMessage() {
        return (<div>{errorMessage}</div>)
    }

    function _renderConnectButton() {
        if(isConnectingToWallet) {
            return (
                <div>Connecting....</div>
            );
        }

        if(currentAccount) {
            return (
                <div>Connected to metamask</div>
            )
        }

        return (
            <button onClick={_onConnectToWalletHandler}>Connect to your Wallet</button>
        );
    }

    function _renderVideoPlayer() {
        return (
            <Fragment>
                <ReactHlsPlayer
                    playerRef={player}
                    src="https://customer-wo7syqqap4g20awy.cloudflarestream.com/0d58d9d181fb619cce31def2509af262/manifest/video.m3u8"
                    width={200}
                    height={200}
                />
            </Fragment>
        );
    }

    async function _onConnectToWalletHandler() {
        setIsConnectingToWallet(true);

        try {
            const account = await connectWallet();

            setCurrentAccount(account);
        } catch (e) {
            setIsConnectingToWallet(false);
            setErrorMessage('Error connecting to the wallet');
        }

    }
    return render();
}

export default App;
