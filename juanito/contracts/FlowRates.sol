// https://eips.ethereum.org/EIPS/eip-20
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;


contract FlowRates {
    // owner => video => rate (gwei/second)
    mapping (address => mapping (string => uint256)) public rates;

    event VideoAdded(address indexed owner, string indexed video, uint256 rate);

    function uploadVideo( string memory video, uint256 rate) public {
        rates[msg.sender][video] = rate;
        emit VideoAdded(msg.sender, video, rate);
    }
    
    function getRate(address owner, string memory video) public view returns (uint256) {
        return rates[owner][video];
    }

}