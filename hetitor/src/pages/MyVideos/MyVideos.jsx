import './MyVideos.css';
import {Fragment, useEffect, useState} from "react";
import VideoPlayer from "../../components/VideoPlayer/VideoPlayer.jsx";
import Header from "../../components/Header/Header.jsx";
import {getBalanceToken} from "../../utils/wallet.js";
import Search from "../../components/Search/Search.jsx";

const receiver = '0x2819Db886a1C12C74Edf3514F831dfA00bFc101F';

function MyVideos() {
    const [balance, setBalance] = useState(0);
    const [selectedVideoInfo, setSelectedVideoInfo] = useState(null);

    useEffect(() => {
        setInterval(() => {
            getBalanceToken('0x5D8B4C2554aeB7e86F387B4d6c00Ac33499Ed01f')
                .then((balanceValue) => setBalance(balanceValue));
        }, 1000);
    }, []);

    function render() {
        return (
            <Fragment>
                {_showHeader()}
                {_showSearchComponent()}
                {_renderVideoViewArea()}
            </Fragment>
        )
    }

    function _showHeader() {
        return (
            <Header account={localStorage.getItem('account')}  balance={balance}/>
        )
    }

    function _showSearchComponent() {
        return (
            <Search onSelectResult={_onSelectResult}/>
        )
    }

    function _renderVideoViewArea() {
        if (selectedVideoInfo) {
            return (
                <VideoPlayer
                    receiver={receiver}
                    videoInfo={selectedVideoInfo}
                    onFlowCreated={(flow) => {
                        console.log(flow)
                    }}
                />
            )
        }
    }

    function _onSelectResult(videoInfo) {
        setSelectedVideoInfo(videoInfo);
    }


    return render();
}

export default MyVideos;