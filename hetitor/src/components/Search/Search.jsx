import './Search.css';
import {useEffect, useState} from "react";

function Search(props) {
    const {onSelectResult} = props;

    const [searchText, setSearchText] = useState('');
    const [isSearching, setIsSearching] = useState(false);
    const [results, setResults] = useState([]);

    useEffect(() => {
        if (searchText !== '') {
            setResults([
                {
                    videoId: '0d58d9d181fb619cce31def2509af262',
                    text: 'Video 1',
                    videoUrl: 'https://customer-wo7syqqap4g20awy.cloudflarestream.com/0d58d9d181fb619cce31def2509af262/manifest/video.m3u8',
                    ownerAddress: '0x358A4567d62b6632169BBAdfA0884aB56b315c24',
                },
                {
                    videoId: 'bacb61daab7ebcfdacf58cb158bb34a2',
                    text: 'Video 2',
                    videoUrl: 'https://customer-wo7syqqap4g20awy.cloudflarestream.com/bacb61daab7ebcfdacf58cb158bb34a2/manifest/video.m3u8',
                    ownerAddress: '0x358A4567d62b6632169BBAdfA0884aB56b315c24',
                },
                {
                    videoId: '8862758380f187fa168a44fb971a96bd',
                    text: 'Video 3',
                    videoUrl: 'https://customer-wo7syqqap4g20awy.cloudflarestream.com/8862758380f187fa168a44fb971a96bd/manifest/video.m3u8',
                    ownerAddress: '0x358A4567d62b6632169BBAdfA0884aB56b315c24',
                }
            ])
        }
    }, [searchText]);

    function render() {
        return (
            <div className="Search_Container">
                <input type={"search"} className="Search_Input" onChange={_onSearchChangeHandler} value={searchText}/>
                {_renderResults()}
            </div>
        );
    }

    function _renderResults() {
        if (results.length > 0) {
            return (
                <div className={"Search_Results_Container"}>
                    {results.map((result) => {
                        return (
                            <div className="ResultItem" onClick={() => {
                                onSelectResult(result)
                                setResults([]);
                            }}>
                                <b className="ResultItem_videoText">{result.text}</b>
                            </div>
                        );
                    })}
                </div>
            )
        }
    }

    function _onSearchChangeHandler(e) {
        setSearchText(e.target.value);
    }

    return render();
}

export default Search;