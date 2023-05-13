import './Header.css';
import Logo from "../../assets/hectango-logo-v2.png";
import TextLogo from "../../assets/text-logo.png";

function Header(props) {
    const {account, balance} = props;

    function render() {
        return (
            <div className="Header">
                {_renderLeftSide()}
                {_renderRightSide()}
            </div>
        );
    }

    function _renderLeftSide() {
        return (
            <div className="Header_left">
                <img className="LogoInRow" src={Logo} alt={'hectango-logo'}/>
                <div className="LogoTextInRow_Container">
                    <img className="LogoTextInRow" src={TextLogo} alt={'text-logo'}/>
                </div>
            </div>
        )
    }

    function _renderRightSide() {
        return (
            <div className="Header_right">
                <p className="Text">{_formatAccount(account)}</p>
                <p className="Text"><b className="Text_bold">Balance:</b>{balance}</p>
            </div>
        )
    }

    function _formatAccount(account) {
        const textSize = account.length;

        return `${account.substring(0,5)}...${account.substring(textSize-5,textSize-1)}`
    }

    return render();
}

export default Header;