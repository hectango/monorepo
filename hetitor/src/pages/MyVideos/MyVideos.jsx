import './MyVideos.css';
import {Fragment, useEffect, useState} from "react";
import VideoPlayer from "../../components/VideoPlayer/VideoPlayer.jsx";
import Header from "../../components/Header/Header.jsx";
import {getBalanceToken} from "../../utils/wallet.js";

const receiver = '0x2819Db886a1C12C74Edf3514F831dfA00bFc101F';
const flowRate = "5787037037037";

function MyVideos() {
    const [balance, setBalance] = useState(0);

    useEffect(() => {
        getBalanceToken('0x5D8B4C2554aeB7e86F387B4d6c00Ac33499Ed01f').then();
    }, []);

    function render() {
        return (
            <Fragment>
                {_showHeader()}
                {_showBalance()}
                {_renderVideoViewArea()}
            </Fragment>
        )
    }

    function _showBalance() {
        return (
            <div>{balance}</div>
        )
    }

    function _showHeader() {
        return (
            <Header account={localStorage.getItem('account')}  balance={'0.01'}/>
        )
    }

    function _renderVideoViewArea() {
        return (
            <VideoPlayer
                receiver={receiver}
                flowRate={flowRate}
                onFlowCreated={(flow) => {
                    console.log(flow)
                }}
            />
        )
    }


    return render();
}

export default MyVideos;