// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

library AStructs {
  /**
   * @dev Enum representing various categories of earnings within the platform.
   * This enhances code readability by replacing numeric codes with descriptive names, making the contract logic clearer and easier to understand.
   */
  enum EarningCategory {
    Win, // Earnings from a successful bet or stake.
    Commission, // Commission earnings distributed from the platform to users.
    VaultFee, // Fees collected and stored in the vault.
    Donation // Donations received and recorded within the platform.
  }

  /**
   * @dev Enum representing the different states an event can be in during its lifecycle.
   * This enum provides clear and descriptive labels for the event states, replacing plain numbers with meaningful names.
   */
  enum EventState {
    NotInitialized, // Event has not been properly initialized yet.
    Open, // Staking is currently allowed for the event, but the event has not started.
    Live, // The event is currently active and ongoing.
    Ended, // The event has concluded, but payouts have not been fully processed.
    Finalized // The event is fully closed, and all payouts have been processed.
  }

  /**
   * @dev Enum representing the different stages or steps an event can go through.
   * This enum adds clarity to the event's progression from being opened to being fully paid out.
   */
  enum Step {
    Opened, // The event is open for staking.
    Closed, // The event has ended, and no more staking is allowed.
    Paid // The event has been fully paid out.
  }

  /**
   * @dev Structure representing a player's stake in an event.
   * This structure holds the amount staked and the team the player has bet on.
   */
  struct Stake {
    uint256 amount; // The total amount of tokens staked by the player.
    uint8 team; // The team the player is betting on: 1 for Team A, 2 for Team B.
  }

  /**
   * @dev Structure representing an event for betting.
   * This structure includes all necessary details for managing the event, including stakes, players, and the event's status.
   */
  struct Event {
    bytes8 eventIdBytes; // Unique identifier for the event in bytes8 format.
    uint256 startDate; // The start date and time of the event.
    address[] players; // List of players who have placed stakes in the event.
    mapping(address => Stake) stakes; // Mapping of player addresses to their respective stakes.
    mapping(address => bool) stakeFinalized; // Mapping to track whether a player's stake has been finalized and paid out.
    uint256[2] total; // Total stakes for each team: index 0 for Team A, index 1 for Team B.
    int8 winner; // The winner of the event: 1 for Team A, 2 for Team B, -2 for a tie, -1 for no result yet, -3 for event canceled.
    uint8 sport; // Identifier representing the sport associated with the event.
    uint256 playersPaid; // Number of players who have been paid out.
    bool active; // Indicates whether the event is currently open for participation.
    bool closed; // Indicates whether the event has ended.
    bool paid; // Indicates whether all payouts for the event have been processed.
  }

  /**
   * @dev Data Transfer Object (DTO) for an event, used to transfer event data between contracts or to external systems.
   * This structure is useful for providing detailed event information in a single object.
   */
  struct EventDTO {
    string eventId; // Unique identifier for the event as a string.
    uint256 startDate; // Start date and time of the event.
    uint8 sport; // Identifier representing the sport associated with the event.
    uint256 total_A; // Total stakes placed on Team A.
    uint256 total_B; // Total stakes placed on Team B.
    uint256 total; // Combined total stakes for both teams.
    int8 winner; // The winner of the event: 1 for Team A, 2 for Team B, -2 for a tie, -1 for no result yet, -3 for event canceled.
    Stake playerStake; // Details of the player's stake in the event.
    bool active; // Indicates whether the event is currently open for participation.
    bool closed; // Indicates whether the event has ended.
    bool paid; // Indicates whether all payouts for the event have been processed.
  }

  /**
   * @dev Structure representing a player's data within the platform.
   * This structure includes details about the player's activity, level, and commission earnings.
   */
  struct Player {
    bytes8[] activeEvents; // Array of event IDs in which the player is currently participating.
    bytes8[] closedEvents; // Array of event IDs for events that the player participated in and that are now closed.
    uint32 level; // The player's current level, representing their experience or skill within the platform.
    uint256 claimedCommissionsByPlayer; // Total amount of commissions claimed by the player.
    uint256 lastCommissionPerTokenForPlayer; // The last recorded commission per token for the player, used to calculate unclaimed commissions.
  }

  /**
   * @dev Structure to hold a summary of the player's data for external view.
   * This structure provides a concise overview of the player's current status and balances.
   */
  struct PlayerSummary {
    uint32 level; // The player's current level.
    uint256 ethBalance; // The player's current ETH balance.
    uint256 atonBalance; // The player's current ATON balance.
    uint256 unclaimedCommission; // The amount of commission the player has not yet claimed.
    uint256 claimedCommission; // The total amount of commission the player has claimed.
  }

  /**
   * @dev Function to populate an event structure with initial values.
   * This function is used when creating a new event to ensure all necessary fields are correctly initialized.
   * @param e The event structure to populate.
   * @param eventIdBytes The unique identifier for the event in bytes8 format.
   * @param _startDate The start date and time for the event.
   * @param _sport The identifier for the sport associated with the event.
   */
  function populateEvent(Event storage e, bytes8 eventIdBytes, uint256 _startDate, uint8 _sport) internal {
    e.eventIdBytes = eventIdBytes;
    e.startDate = _startDate;
    e.active = true;
    e.closed = false;
    e.paid = false;
    e.winner = -1; // Initialize with no result yet
    e.sport = _sport;
  }
}
// This software is provided "as is", without warranty of any kind, express or implied, 
// including but not limited to the warranties of merchantability, fitness for a particular purpose, 
// and noninfringement. In no event shall the authors or copyright holders be liable for any claim, 
// damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, 
// out of, or in connection with the software or the use or other dealings in the software.
