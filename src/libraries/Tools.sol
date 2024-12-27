// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;
import './AStructs.sol';

library Tools {
    /**
     * @dev Converts a string to a bytes8 value.
     * @param source The input string to be converted.
     * @return result The bytes8 representation of the input string.
     * Internal function, not meant to be called directly.
     */
    function _stringToBytes8(string memory source) internal pure returns (bytes8 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
    }

    /**
     * @dev Converts a bytes8 value to a string.
     * @param x The input bytes8 value to be converted.
     * @return string The string representation of the input bytes8 value.
     * Internal function, not meant to be called directly.
     */
    function _bytes8ToString(bytes8 x) internal pure returns (string memory) {
        bytes memory bytesString = new bytes(8);
        for (uint256 i = 0; i < 8; i++) {
            bytesString[i] = x[i];
        }
        return string(bytesString);
    }
}


    // const bytes8EventId = ethers.utils.formatBytes32String(_eventId).slice(0, 18); // Slice for bytes8



// This software is provided "as is", without warranty of any kind, express or implied, 
// including but not limited to the warranties of merchantability, fitness for a particular purpose, 
// and noninfringement. In no event shall the authors or copyright holders be liable for any claim, 
// damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, 
// out of, or in connection with the software or the use or other dealings in the software.
