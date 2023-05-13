import './LandingPage.css';
import {Fragment, useEffect, useState} from "react";
import {useNavigate} from 'react-router-dom';
import {checkIfWalletIsConnected, connectWallet} from "../../utils/wallet.js";
import Logo from '../../assets/hectango-logo-v2.png';
import TextLogo from '../../assets/text-logo.png';

function LandingPage() {
    const [errorMessage, setErrorMessage] = useState(null);
    const [isConnectingToWallet, setIsConnectingToWallet] = useState(true);
    const [currentAccount, setCurrentAccount] = useState();
    const navigate = useNavigate();

    useEffect(() => {
        setIsConnectingToWallet(false);
        if (currentAccount) {
            setIsConnectingToWallet(false);
            setErrorMessage(null);
            navigate('/my-videos');
            localStorage.setItem('account', currentAccount);
        }
    }, [currentAccount]);

    useEffect(() => {
        checkIfWalletIsConnected().then((account) => {
            setCurrentAccount(account);
        }).catch(() => setIsConnectingToWallet(false));
    }, []);

    function render() {
        return (
            <div className={"Container"}>
                {_renderErrorMessage()}
                {_renderLogo()}
                {_renderConnectButton()}
            </div>
        )
    }

    function _renderLogo() {
        return (
            <Fragment>
                <img className="Logo" src={Logo} alt={'hectango-logo'}/>
                <img className="LogoText" src={TextLogo} alt={'text-logo'}/>
            </Fragment>
        );
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
            <button className="css-button-sliding-to-left--blue" onClick={_onConnectToWalletHandler}>Connect to your Wallet</button>
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

export default LandingPage;