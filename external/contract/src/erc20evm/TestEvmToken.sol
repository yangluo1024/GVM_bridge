// Copyright (C) 2021 Cycan Technologies
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pragma solidity ^0.8.0;

import  "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestEvmToken is ERC20 {

    constructor () ERC20("TestEvmToken", "TET") public {
        _mint(msg.sender, 1000000000000*1e18);
    }
	
	function _callWasmC(string memory input) internal returns (string memory) {
		uint inputLen = bytes(input).length + 32;  //First 32bytes is string len prefix
		bytes memory outbytes = new bytes(1024);
		uint gasdata = gasleft();
		assembly {
			if iszero(delegatecall(gasdata, 0x05, input, inputLen, outbytes, 1024)) {
				revert(0,0)
			}
		}
		return string(outbytes);
	}
	
	function evmCallWasm(bytes32 bob, uint256 value, bytes32 contractid) public returns (string memory) {
		
		bytes memory input1 = bytes('{"VM":"wasm", "Account":"0x');
		input1 = _bytesConcat(input1, bytes(_bytes32tohex(contractid)));
		input1 = _bytesConcat(input1, bytes('", "Fun": "transfer", "InputType": ["accountid","u128"], "InputValue": ["0x'));
		input1 = _bytesConcat(input1, bytes(_bytes32tohex(bob)));
		input1 = _bytesConcat(input1, bytes('","'));
		input1 = _bytesConcat(input1, bytes(_uint2str10(value)));
		input1 = _bytesConcat(input1, bytes('"], "OutputType":[["enum"],["0","2"],["0"]]}'));
		
		//string input = '{"VM":"wasm", "Account":"0x' + _bytes32tohex(contractid) + '", "Fun": "transfer", "InputType": ["accountid","u128"], 
		//"InputValue": ["0x' + _bytes32tohex(bob) + '","'+ _uint2str10(value) + '",], "OutputType":[["enum"],["0","2"],["0"]]}';
		
		return _callWasmC(string(input1));
	}
	
	function _bytes32tohex(bytes32 b) internal pure returns (string memory) {
		bytes memory bytesString = new bytes(64);
		for (uint i = 0; i< b.length; i++) {
			bytesString[i*2] = _byte2hex(uint8(b[i]) / 16);
			bytesString[i*2 + 1] = _byte2hex(uint8(b[i]) % 16);
		}
		return string(bytesString);
	}
	
	function _byte2hex(uint8 b) internal pure returns (bytes1) {
		require(b < 16, "byte2hex -- b must < 16");
		if(b < 10){
			return  bytes1(b + 0x30);
		}
		else {
			return bytes1(b - 9 + 0x60 );
		}
	}
	
	function _uint2str10(uint v) internal pure returns (string memory) {
		if( v == 0) return "0";
		uint v1 = v;
        uint strlen = 0;
        while(v1 != 0) {
			v1 /= 10;
			strlen++;
		}
		bytes memory str = new bytes(strlen);
	    while(v != 0 && strlen > 0) {
		    strlen--;
			str[strlen] = bytes1(uint8(v%10) + 0x30);
			v /= 10;
		}
		
		return string(str);
	}
	
	 function _bytesConcat(bytes memory _a, bytes memory _b) internal pure returns (bytes memory){
		 bytes memory ret = new bytes(_a.length + _b.length);
		 for(uint i=0; i<_a.length; i++) ret[i] = _a[i];
		 for(uint i=0; i< _b.length; i++) ret[i+_a.length] = _b[i];
		 
		 return ret;
	 }
	
}
