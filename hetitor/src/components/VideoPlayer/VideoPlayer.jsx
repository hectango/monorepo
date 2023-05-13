import './VideoPlayer.css';
import {Fragment, useEffect, useRef, useState} from "react";
import ReactHlsPlayer from "react-hls-player";
import {createNewFlow, deleteExistingFlow} from "../../utils/superfluid.js";
import Play from '../../assets/play.png';
import Pause from '../../assets/pause.png';
import LoadingSpinner from "../LoadingSpinner/LoadingSpinner.jsx";

function VideoPlayer(props) {
    const {receiver, flowRate, onFlowCreated} = props;
    const [isVideoPlaying, setIsVideoPlaying] = useState(false)
    const [flow, setFlow] = useState(null);
    const [isTalkingToBlockchain, setIsTalkingToBlockchain] = useState(false);

    const player = useRef(null);

    useEffect(() => {
        if (flow) {
            onFlowCreated(flow);
        }
    }, [flow]);

    function render() {
        return (
            <Fragment>
                {_renderOverlay()}
                {_renderVideoPlayer()}
                {_renderControlButtons()}
            </Fragment>
        )
    }

    function _renderControlButtons() {
        return (
            <div className="Controls">
                {!isVideoPlaying && <img className={'IconButton'} src={Play} onClick={_onPlay}/>}
                {isVideoPlaying && <img className={'IconButton'} src={Pause} onClick={_onPause}/>}
            </div>

        );
    }

    function _renderOverlay() {
        if (isTalkingToBlockchain) {
            return <div className="overlay">
                <LoadingSpinner/>
                <p>Loading the amazing magic of blockchain....</p>
            </div>
        }
    }

    function _renderVideoPlayer() {
        return (
            <div className="VideoContainer">
                <ReactHlsPlayer
                    playerRef={player}
                    src="https://customer-wo7syqqap4g20awy.cloudflarestream.com/0d58d9d181fb619cce31def2509af262/manifest/video.m3u8"
                    width="800px"
                    height="auto"
                />
            </div>
        );
    }

    function _onPlay() {
        setIsTalkingToBlockchain(true);
        setIsVideoPlaying(true);
        createNewFlow(receiver, flowRate).then((flow) => {
            setFlow(flow);
            player.current.play();
            setIsTalkingToBlockchain(false);
        }).catch(console.error)
    }

    function _onPause() {
        player.current.pause();
        setIsVideoPlaying(false);
        setIsTalkingToBlockchain(true);
        deleteExistingFlow(receiver).then(() => {
            setFlow(null);
            setIsTalkingToBlockchain(false)
        }).catch(console.error);
    }

    return render();
}

export default VideoPlayer;