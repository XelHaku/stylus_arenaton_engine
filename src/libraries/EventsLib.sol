// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

library EventsLib {

    /**
     * @dev Logs a player action such as staking or earning.
     *
     * @param eventIdIndexed - Indexed unique identifier for filtering.
     * @param playerIndexed - Indexed player address for filtering.
     * @param amount - The amount of ATON tokens involved.
     * @param actionType - Type of action (0 = Stake, 1 = Earnings).
     */
    event PlayerAction(
        string indexed eventIdIndexed,
        address indexed playerIndexed,
        uint256 amount,
        uint8 actionType
    );

    /**
     * @dev Logs a state change of an event, such as Opened, Closed, or Finalized.
     *
     * @param eventIdIndexed - Indexed unique identifier for filtering.
     * @param sportOrWinner - Sport type or winner (0 if not applicable).
     * @param eventType - Type of event (0 = Opened, 1 = Closed, 2 = Finalized).
     */
    event EventStateChanged(
        string indexed eventIdIndexed,
        int8 sportOrWinner,
        uint8 eventType
    );

    /**
     * @dev Logs a player token swap.
     *
     * @param playerIndexed - Indexed player address for filtering.
     * @param amount - Amount of `tokenOut` received by the player.
     */
    event Swap(address indexed playerIndexed, uint256 amount);

    /**
     * @dev Logs the update of accumulated commission in ATON.
     *
     * @param newCommissionATON - New commission added.
     * @param accumulatedCommissionPerToken - Cumulative commission per token.
     * @param totalCommissionInATON - Total accumulated commission.
     */
    event Accumulate(uint256 newCommissionATON, uint256 accumulatedCommissionPerToken, uint256 totalCommissionInATON);

    /**
     * @dev Logs donations in ATON.
     *
     * @param donor - Address of the donor.
     * @param amount - Amount donated in ATON.
     */
    event ATONDonated(address indexed donor, uint256 amount);
}

// This software is provided "as is", without warranty of any kind, express or implied, 
// including but not limited to the warranties of merchantability, fitness for a particular purpose, 
// and noninfringement. In no event shall the authors or copyright holders be liable for any claim, 
// damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, 
// out of, or in connection with the software or the use or other dealings in the software.
