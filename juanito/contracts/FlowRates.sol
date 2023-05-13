// https://eips.ethereum.org/EIPS/eip-20
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;


contract FlowRates {

    // Platform settings
    uint256 public platformRate;
    // owner => video => rate (gwei/second)
    mapping (address => mapping (string => uint256)) public rates;
    address private _owner;

    modifier onlyOwner() {
        require(_owner == msg.sender, "Oops! owners only");
        _;
    }

    event VideoAdded(address indexed owner, string indexed video, uint256 rate);
    event PlatformNewRate(address indexed caller, uint256 previousRate, uint256 NewRate);

    constructor(uint256 defaultRate) {
        _owner = msg.sender;
        platformRate = defaultRate;
    }

    function uploadVideo( string memory video, uint256 rate) public {
        rates[msg.sender][video] = rate;
        emit VideoAdded(msg.sender, video, rate);
    }
    
    function getRate(address owner, string memory video) public view returns (uint256) {
        return rates[owner][video];
    }

    function setPlatformRate(uint256 newRate) public onlyOwner {
        uint256 previous = platformRate;
        platformRate = newRate;
        emit PlatformNewRate(msg.sender, previous, platformRate);
    }
}