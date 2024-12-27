//                     _.-'-._
//                  _.'       '-.
//              _.-'   _.   .    '-._
//           _.'   _.eEEE   EEe..    '-._
//       _.-'   _.eEE* EE   EE`*EEe._    '-.
//    _.'   _.eEEE'  . EE   EE .  `*EEe._   '-
//    |   eEEP*'_.eEE' EP   YE  Ee._ `'*EE.   |
//    |   EE  .eEEEE' AV  .. VA.'EEEEe.  EE   |
//    |   EE |EEEEP  AV  /  \ VA.'*E***--**---'._     .------------.    .----------._          /\       .------------.     _.--------._    .-----------._
//    |   EE |EEEP  EEe./    \eEE. E|   _  ___   '    '------------'    |  .......   .        /  \      '----.  .----'    |   ______   .   |   .......   .
//    |   EE |EEP AVVEE/  /\  \EEEA |  |_EE___|   )   .----------- .    |  |      |  |       / /\ \          |  |         |  |      |  |   |  |       |  |
//    |   EE |EP AV  `   /EE\  \ 'EA|            .    '------------'    |  |      |  |      / /  \ \         |  |         |  |      |  |   |  |       |  |
//    |   EE ' _AV   /  /EE|"   \ `E|  |-ee-\   \     .------------.    |  |      |  |     / /  --' \        |  |         |  '------'  .   |  |       |  |
//    |   EE.eEEP   /__/*EE|_____\  '--|.EE  '---'.   '------------'    '--'      '--'    /-/   -----\       '--'          '..........'    '--'       '--'
//    |   EEP            EEE          `'*EE   |
//    |   *   _.eEEEEEEEEEEEEEEEEEEE._   `*   |
//    |     <EEE<  .eeeeeeeeeeeee. `>EEE>     |
//    '-._   `*EEe. `'*EEEEEEE*' _.eEEP'   _.-'
//        `-._   `"Ee._ `*E*'_.eEEP'   _.-'
//            `-.   `*EEe._.eEE*'   _.'
//               `-._   `*V*'   _.-'
//                   '-_     _-'
//                      '-.-'

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

import "./libraries/AStructs.sol";
import "./libraries/Tools.sol";
import "./libraries/EventsLib.sol";

contract Arenaton is ERC20, ReentrancyGuard {
  uint256 private premium = 200000;
  uint256 constant pct_denom = 10000000;

  // Mapping for storing event and player data
  mapping(bytes8 => AStructs.Event) private events;
  mapping(address => AStructs.Player) private players;

  // Array for tracking active events
  bytes8[] private activeEvents;
  bytes8[] private closedEvents;

  // Represents the total accumulated commission per token
  uint256 private accumulatedCommissionPerToken;

  // Stores the total commission in ATON
  uint256 private totalCommissionInATON;

  address private owner;
  mapping(address => bool) private authorizedAddresses;

  // Constructor to initialize the contract with a list of addresses
  // First address in the list is the owner, and the rest are authorized addresses
  constructor(address[] memory _addresses) ERC20("Arenaton", "ATON") {
    require(_addresses.length > 0, "At least one address must be provided");

    // Set the first address as the owner
    owner = _addresses[0];

    // Set the subsequent addresses as authorized addresses
    for (uint256 i = 1; i < _addresses.length; i++) {
      authorizedAddresses[_addresses[i]] = true;
    }
  }

  // ░█████╗░██╗░░░██╗████████╗██╗░░██╗░█████╗░██████╗░██╗███████╗░█████╗░████████╗██╗░█████╗░███╗░░██╗░██████╗
  // ██╔══██╗██║░░░██║╚══██╔══╝██║░░██║██╔══██╗██╔══██╗██║╚════██║██╔══██╗╚══██╔══╝██║██╔══██╗████╗░██║██╔════╝
  // ███████║██║░░░██║░░░██║░░░███████║██║░░██║██████╔╝██║░░███╔═╝███████║░░░██║░░░██║██║░░██║██╔██╗██║╚█████╗░
  // ██╔══██║██║░░░██║░░░██║░░░██╔══██║██║░░██║██╔══██╗██║██╔══╝░░██╔══██║░░░██║░░░██║██║░░██║██║╚████║░╚═══██╗
  // ██║░░██║╚██████╔╝░░░██║░░░██║░░██║╚█████╔╝██║░░██║██║███████╗██║░░██║░░░██║░░░██║╚█████╔╝██║░╚███║██████╔╝
  // ╚═╝░░╚═╝░╚═════╝░░░░╚═╝░░░╚═╝░░╚═╝░╚════╝░╚═╝░░╚═╝╚═╝╚══════╝╚═╝░░╚═╝░░░╚═╝░░░╚═╝░╚════╝░╚═╝░░╚══╝╚═════╝░

  modifier onlyOwner() {
    require(msg.sender == owner, "Caller is not the owner");
    _;
  }

  modifier onlyAuthorized() {
    require(authorizedAddresses[msg.sender], "Caller is not authorized");
    _;
  }

  /**
   * @dev Adds or removes an authorized address.
   * @param authorizedAddress The address to be added or removed.
   */
  function setAuthorizedAddress(address authorizedAddress) external onlyOwner {
    authorizedAddresses[authorizedAddress] = !authorizedAddresses[authorizedAddress];
  }

  /**
   * @dev Checks if an address is authorized.
   * @param authorizedAddress The address to check.
   * @return bool True if the address is authorized, false otherwise.
   */
  function isAuthorized(address authorizedAddress) external view returns (bool) {
    return authorizedAddresses[authorizedAddress];
  }

  // ███████╗██╗░░░██╗███████╗███╗░░██╗████████╗░██████╗
  // ██╔════╝██║░░░██║██╔════╝████╗░██║╚══██╔══╝██╔════╝
  // █████╗░░╚██╗░██╔╝█████╗░░██╔██╗██║░░░██║░░░╚█████╗░
  // ██╔══╝░░░╚████╔╝░██╔══╝░░██║╚████║░░░██║░░░░╚═══██╗
  // ███████╗░░╚██╔╝░░███████╗██║░╚███║░░░██║░░░██████╔╝
  // ╚══════╝░░░╚═╝░░░╚══════╝╚═╝░░╚══╝░░░╚═╝░░░╚═════╝░

  /**
   * @dev Adds a new event to the platform.
   * @param _eventId The unique identifier for the event.
   * @param _startDate The start date of the event.
   * @param _sport The sport associated with the event.
   */
  function addEvent(string memory _eventId, uint256 _startDate, uint8 _sport) external onlyAuthorized {
    bytes8 eid = Tools._stringToBytes8(_eventId); // Convert event ID to bytes8 format

    // Validate event parameters
    require(_startDate > block.timestamp && !events[eid].closed && !events[eid].active, "Event invalid");

    // Populate the event struct with the provided details
    AStructs.populateEvent(events[eid], eid, _startDate, _sport);
    activeEvents.push(eid);

    // Emit an event indicating the event creation
    emit EventsLib.EventStateChanged(_eventId, int8(_sport), 0); // 0 represents the "Opened" event type
  }

  // External function to allow a player to stake with ETH or ATON on a specific event
  /**
   * @dev Allows a player to stake with ETH or ATON on a specific event.
   * @param _eventId The unique identifier for the event.
   * @param _amountATON The amount of ATON tokens to stake. If 0, the stake is in ETH.
   * @param _team The team to stake on.
   * @param isGasless Whether the staking should be gasless (true) or not (false).
   * @param _player The player who is staking (only relevant for gasless staking).
   */
  function stake(
    string memory _eventId,
    uint256 _amountATON,
    uint8 _team,
    bool isGasless,
    address _player
  ) external payable nonReentrant {
    bool isETH = msg.value > 0;
    require(isETH || _amountATON > 0, "Cannot stake 0 value");

    address staker = isGasless ? _player : msg.sender;
    if (isGasless) {
      require(authorizedAddresses[msg.sender], "Not authorized to stake");
    }
    require(msg.sender != address(this), "Cannot stake from contract");

    if (!isETH) {
      _distributeTransfer(staker, address(this), _amountATON); // Ensure transfer is correctly defined
    }

    _stake(_eventId, isETH ? msg.value : 0, isETH ? 0 : _amountATON, _team, staker);
  }

  /**
   * @dev Internal function to handle the staking logic in an event.
   * This function allows players to stake both ETH and ATON (an ERC20 token) on a team in a given event.
   * The staked amounts are added to the event's total pool, and players can stake either ETH, ATON, or both.
   *
   * @param _eventId The unique identifier for the event.
   * @param _amountETH The amount of ETH (in wei) the player is staking. Set to 0 if no ETH is staked.
   * @param _amountATON The amount of ATON (ERC20 tokens) the player is staking. Set to 0 if no ATON is staked.
   * @param _team The team the player is staking on. Valid values are 1 or 2 (representing different teams).
   * @param _player The address of the player who is staking.
   */
  function _stake(
    string memory _eventId,
    uint256 _amountETH,
    uint256 _amountATON,
    uint8 _team,
    address _player
  ) internal {
    // Convert event ID from string to bytes8 format for internal use
    bytes8 eid = Tools._stringToBytes8(_eventId);

    // Validate event status and parameters:
    // - Event must be active, not closed, and the start date must be in the future.
    // - Player must stake on a valid team (either team 1 or team 2).
    require(
      events[eid].active &&
        !events[eid].closed &&
        events[eid].startDate > block.timestamp &&
        (_team == 1 || _team == 2),
      "Invalid event or team"
    );

    // If the player is staking ETH, mint the equivalent amount of ATON tokens to the contract
    if (_amountETH > 0) {
      _mint(address(this), _amountETH); // Ensure the _mint function properly handles ETH-to-ATON conversion and minting
    }

    // Calculate the total stake by adding the ETH and ATON amounts
    uint256 amount = _amountETH + _amountATON;

    // Retrieve the current event and the player's stake within the event
    AStructs.Event storage currEvent = events[eid];
    AStructs.Stake storage playerStake = currEvent.stakes[_player];

    // If this is the player's first stake in the event, initialize their stake
    if (playerStake.amount == 0) {
      currEvent.players.push(_player); // Add player to the event's player list
      playerStake.amount = amount; // Set their stake amount
      playerStake.team = _team; // Set the team the player is staking on
      players[_player].activeEvents.push(eid); // Track the player's active events
    } else {
      // If the player has already staked, ensure they are staking on the same team
      require(playerStake.team == _team, "Cannot stake on different teams");
      playerStake.amount += amount; // Increase their total stake
    }

    // Add the stake to the total for the selected team
    currEvent.total[_team - 1] += amount;

    // Emit the PlayerAction event to signal that a stake has been added
    // The actionType `0` signifies that the action is a stake addition
    emit EventsLib.PlayerAction(_eventId, _player, amount, 0);
  }

  /**
   * @dev Closes an event, records the winner, and distributes payouts to the players in batches.
   * This function updates the event's status, validates the winner, calculates and accumulates the commission if applicable,
   * and pays the players in batches.
   *
   * @param eventId The unique identifier of the event.
   * @param _winner The winning team (1 for Team 1, 2 for Team 2, 0 for a draw, -1 for cancellation).
   * @param _batchSize The number of players to process in each batch for payouts.
   */
  function terminateEvent(string memory eventId, int8 _winner, uint8 _batchSize) external onlyAuthorized {
    bytes8 eventIdBytes = Tools._stringToBytes8(eventId);
    AStructs.Event storage eventDetail = events[eventIdBytes];

    // Ensure the event is valid for termination and is not already closed
    require(!eventDetail.closed || !eventDetail.paid, "Event already closed");
    require(eventDetail.startDate < block.timestamp, "Event not yet eligible for termination");
    require(_winner >= -1 && _winner <= 2, "Invalid winner value");

    uint256 totalStakedAmount = eventDetail.total[0] + eventDetail.total[1];
    uint256 commission = (totalStakedAmount * premium) / pct_denom;
    bool waiveCommission = true;

    if (eventDetail.players.length > 1) {
      waiveCommission = false;
    }

    // Close the event and record the winner if it is not already closed
    if (!eventDetail.closed) {
      eventDetail.winner = _winner;
      eventDetail.closed = true;
      _removeEvent(eventIdBytes, activeEvents);

      if (!waiveCommission) {
        _accumulateCommission(commission); // Accumulate commission here
      }

      closedEvents.push(eventIdBytes);
      emit EventsLib.EventStateChanged(eventId, _winner, 1); // 1: EventClosed
    } else {
      // Handle payouts
      if (eventDetail.players.length == 0) {
        eventDetail.paid = true; // Mark as paid if no players
      } else if (eventDetail.players.length == 1) {
        // Handle single player payout
        address player = eventDetail.players[0];
        eventDetail.stakeFinalized[player] = true;

        distributeAndFinalize(
          player,
          totalStakedAmount,
          commission,
          waiveCommission,
          eventIdBytes,
          eventDetail.winner,
          eventDetail,
          eventDetail.total
        );

        eventDetail.paid = true;
      } else {
        // Process payouts in batches for multiple players
        uint256 playersProcessed = 0;
        while (playersProcessed < _batchSize && eventDetail.playersPaid < eventDetail.players.length) {
          address player = eventDetail.players[eventDetail.playersPaid];

          if (eventDetail.stakes[player].amount > 0 && !eventDetail.stakeFinalized[player]) {
            distributeAndFinalize(
              player,
              totalStakedAmount,
              commission,
              waiveCommission,
              eventIdBytes,
              eventDetail.winner,
              eventDetail,
              eventDetail.total
            );
          }

          eventDetail.playersPaid++;
          playersProcessed++;
        }

        // Mark event as fully paid if all players have been processed
        if (eventDetail.playersPaid >= eventDetail.players.length) {
          eventDetail.paid = true;
        }
      }

      // Optionally emit another event when payouts are fully processed
      if (eventDetail.paid) {
        emit EventsLib.EventStateChanged(eventId, _winner, 2); // 2: EventPaid
      }
    }
  }

  /**
   * @dev Finalizes the stake and distributes rewards to a player.
   * @param player The address of the player.
   * @param totalStake The total stake of the event.
   * @param commission The total commission calculated in terminateEvent.
   * @param waiveCommission Whether the commission should be waived.
   * @param eventIdBytes The event ID in bytes8 format.
   * @param winner The winner of the event (1, 2, 0 for tie, -1 for cancel).
   * @param eventDetail The event details.
   * @param teamStakes The stakes for both teams.
   */
  function distributeAndFinalize(
    address player,
    uint256 totalStake,
    uint256 commission,
    bool waiveCommission,
    bytes8 eventIdBytes,
    int8 winner,
    AStructs.Event storage eventDetail,
    uint256[2] memory teamStakes
  ) private {
    eventDetail.stakeFinalized[player] = true;
    _removeEvent(eventIdBytes, players[player].activeEvents);
    players[player].closedEvents.push(eventIdBytes);

    uint256 playerStake = eventDetail.stakes[player].amount;
    uint256 playerShare = 0;

    // Check if the player is on the winning team
    if (
      (winner == 1 && int8(eventDetail.stakes[player].team) == winner) ||
      (winner == 2 && int8(eventDetail.stakes[player].team) == winner)
    ) {
      players[player].level += 3;
      playerShare = (playerStake * totalStake) / teamStakes[uint8(winner - 1)];
    }
    // In case of a tie
    else if (winner == 0) {
      players[player].level += 2;

      uint8 playerTeam = uint8(eventDetail.stakes[player].team);
      uint8 opposingTeamIndex = playerTeam == 1 ? 1 : 0;

      if (teamStakes[opposingTeamIndex] > 0) {
        playerShare = (playerStake * teamStakes[opposingTeamIndex]) / teamStakes[playerTeam - 1];
      } else {
        playerShare = playerStake;
      }
    }
    // In case of a cancelled event
    else if (winner == -1) {
      players[player].level += 1;
      playerShare = playerStake;
      waiveCommission = true;
    } else {
      return;
    }

    // Apply commission if not waived
    if (!waiveCommission) {
      uint256 fee = (playerShare * commission) / totalStake; // Calculate player's share of commission
      playerShare -= fee;
      // Do not accumulate the fee again
    }

    _distributeTransfer(address(this), player, playerShare);
  }

  /**
   * @dev Removes an event from the specified list of events.
   * @param eventIdBytes The event ID in bytes8 format.
   * @param eventList The list of events (active , closed, player active).
   */
  function _removeEvent(bytes8 eventIdBytes, bytes8[] storage eventList) internal {
    uint256 length = eventList.length;
    uint256 indexToRemove = length;
    for (uint256 i = 0; i < length; i++) {
      if (eventList[i] == eventIdBytes) {
        indexToRemove = i;
        break;
      }
    }

    require(indexToRemove < length, "Event not found");

    eventList[indexToRemove] = eventList[length - 1];
    eventList.pop();
  }

  // ░██████╗░██╗░░░██╗███████╗██████╗░██╗███████╗░██████╗
  // ██╔═══██╗██║░░░██║██╔════╝██╔══██╗██║██╔════╝██╔════╝
  // ██║██╗██║██║░░░██║█████╗░░██████╔╝██║█████╗░░╚█████╗░
  // ╚██████╔╝██║░░░██║██╔══╝░░██╔══██╗██║██╔══╝░░░╚═══██╗
  // ░╚═██╔═╝░╚██████╔╝███████╗██║░░██║██║███████╗██████╔╝
  // ░░░╚═╝░░░░╚═════╝░╚══════╝╚═╝░░╚═╝╚═╝╚══════╝╚═════╝░

  /**
   * @dev Retrieves a summary of a single player's data and includes global commission data,
   * as well as a batch of event IDs (either active or closed).
   * @param playerAddress The address of the player.
   * @return summary A PlayerSummary struct containing the player's summary data.
   * @return totalCommission The total commission in ATON.
   * @return accumulatedCommission The accumulated commission per token.
   */
  function playerSummary(
    address playerAddress
  )
    external
    view
    returns (AStructs.PlayerSummary memory summary, uint256 totalCommission, uint256 accumulatedCommission)
  {
    AStructs.Player storage player = players[playerAddress];

    // Populate the player's summary
    summary = AStructs.PlayerSummary({
      level: player.level, // Player's current level
      ethBalance: playerAddress.balance, // Player's ETH balance
      atonBalance: balanceOf(playerAddress), // Player's ATON token balance
      unclaimedCommission: _playerCommission(playerAddress), // Player's unclaimed commission
      claimedCommission: player.claimedCommissionsByPlayer // Player's total claimed commission
    });

    // Assign the global data to the return values
    totalCommission = totalCommissionInATON;
    accumulatedCommission = accumulatedCommissionPerToken;

    // Return the player's summary along with the global commission data
    return (summary, totalCommission, accumulatedCommission);
  }

  function getPlayerEvents(
    address playerAddress,
    uint8 sport,
    bool active,
    uint256 size,
    uint256 pageNo
  ) external view returns (AStructs.EventDTO[] memory) {
    AStructs.Player storage player = players[playerAddress];

    // Determine whether to retrieve active or closed events
    bytes8[] storage eventList = active ? player.activeEvents : player.closedEvents;

    uint256 totalEvents = eventList.length;

    // Calculate start index based on pageNo and size
    uint256 startIndex = (pageNo - 1) * size;

    // Calculate the number of events to return based on available events
    uint256 endIndex = startIndex + size;
    if (endIndex > totalEvents) {
      endIndex = totalEvents;
    }

    // Filter and retrieve events matching the sport condition
    return _filterAndGetEvents(eventList, playerAddress, sport, startIndex, endIndex);
  }

  function _filterAndGetEvents(
    bytes8[] storage eventList,
    address playerAddress,
    uint8 sport,
    uint256 startIndex,
    uint256 endIndex
  ) internal view returns (AStructs.EventDTO[] memory) {
    // Initialize a temporary array to store filtered events
    AStructs.EventDTO[] memory tempEvents = new AStructs.EventDTO[](endIndex - startIndex);
    uint256 count = 0;

    // Populate the tempEvents array with event details that match the sport filter
    for (uint256 i = startIndex; i < endIndex; i++) {
      AStructs.EventDTO memory eventDTO = _getEventDTO(eventList[i], playerAddress);
      // Check if the event matches the specified sport or if sport < 0 (which means return all)
      if (eventDTO.sport == sport || sport == 0) {
        tempEvents[count] = eventDTO;
        count++;
      }
    }

    // Create a final array with the exact size needed to store the filtered events
    AStructs.EventDTO[] memory finalEventsDTO = new AStructs.EventDTO[](count);
    for (uint256 i = 0; i < count; i++) {
      finalEventsDTO[i] = tempEvents[i];
    }

    return finalEventsDTO;
  }

  /**
   * @dev Retrieves the details of a specific event based on the event ID.
   * @param _eventId The event ID to retrieve.
   * @param _player The player's address (optional, used for filtering stakes).
   * @return The details of the specified event.
   */
  function getEventDTO(address _player, string memory _eventId) external view returns (AStructs.EventDTO memory) {
    bytes8 eventIdBytes = Tools._stringToBytes8(_eventId);

    AStructs.EventDTO memory eventDTO = _getEventDTO(eventIdBytes, _player);

    if (bytes(eventDTO.eventId).length == 0) {
      return
        AStructs.EventDTO({
          eventId: "",
          startDate: 0,
          sport: 0,
          total_A: 0,
          total_B: 0,
          total: 0,
          winner: 0,
          playerStake: AStructs.Stake({ amount: 0, team: 0 }),
          active: false,
          closed: false,
          paid: false
        });
    }

    return eventDTO;
  }

  // Function to retrieve multiple events
  /**
   * @dev Retrieves a list of events based on the provided filters.
   * @param _sport The sport identifier for filtering events.
   * @param _step The status of the events to retrieve (Opened, Closed, Paid).
   * @param _player The player's address (optional, used for filtering stakes).
   * @return A list of event details.
   */
  function getEvents(
    uint8 _sport,
    AStructs.Step _step,
    address _player
  ) external view returns (AStructs.EventDTO[] memory) {
    return _listEvents(_sport, _step, _player); // Assuming _listEvents is defined elsewhere
  }

  /**
   * @dev Internal function to retrieve events based on custom criteria.
   * @param _sport The sport identifier.
   * @param _step Indicates the status of the events to retrieve (Opened, Closed, Paid).
   * @param _player The player's address (optional, used for filtering stakes).
   * @return A list of event details based on the criteria.
   */
  function _listEvents(
    uint8 _sport,
    AStructs.Step _step,
    address _player
  ) internal view returns (AStructs.EventDTO[] memory) {
    bool isActive = (_step == AStructs.Step.Opened);
    bool isClosable = (_step == AStructs.Step.Closed);

    bytes8[] storage eventList = isActive ? activeEvents : closedEvents;
    AStructs.EventDTO[] memory tempEventsDTO = new AStructs.EventDTO[](eventList.length);
    uint256 count = 0;

    for (uint256 i = 0; i < eventList.length; i++) {
      AStructs.EventDTO memory currentEvent = _getEventDTO(eventList[i], _player);

      bool sportMatch = currentEvent.sport == (_sport) || _sport == 0;
      bool closableMatch = isClosable
        ? (isActive ? currentEvent.startDate < block.timestamp : currentEvent.closed && !currentEvent.paid)
        : true;

      if (sportMatch && closableMatch) {
        tempEventsDTO[count] = currentEvent;
        count++;
      }
    }

    AStructs.EventDTO[] memory finalEventsDTO = new AStructs.EventDTO[](count);
    for (uint256 i = 0; i < count; i++) {
      finalEventsDTO[i] = tempEventsDTO[i];
    }

    return finalEventsDTO;
  }

  /**
   * @dev Internal function to get detailed information about an event.
   * @param eventIdBytes The event ID in bytes8 format.
   * @param _player The player's address.
   * @return The event details.
   */
  function _getEventDTO(bytes8 eventIdBytes, address _player) internal view returns (AStructs.EventDTO memory) {
    // Check if the event exists, if not, return an empty event
    AStructs.Event storage eventDetails = events[eventIdBytes];
    if (eventDetails.startDate == 0) {
      // Return an empty EventDTO structure when the event does not exist
      return
        AStructs.EventDTO({
          eventId: "",
          startDate: 0,
          sport: 0,
          total_A: 0,
          total_B: 0,
          total: 0,
          winner: 0,
          playerStake: AStructs.Stake({ amount: 0, team: 0 }),
          active: false,
          closed: false,
          paid: false
        });
    }

    // Check if the player has a stake in the event
    AStructs.Stake memory playerStake = eventDetails.stakes[_player];

    // If the player does not have any stakes, initialize the stake to zero values
    if (playerStake.amount == 0 && playerStake.team == 0) {
      playerStake = AStructs.Stake({ amount: 0, team: 0 });
    }

    // Assemble the EventDTO structure with all event details
    AStructs.EventDTO memory eventDTO = AStructs.EventDTO({
      eventId: Tools._bytes8ToString(eventDetails.eventIdBytes),
      startDate: eventDetails.startDate,
      sport: eventDetails.sport,
      total_A: eventDetails.total[0],
      total_B: eventDetails.total[1],
      total: eventDetails.total[0] + eventDetails.total[1],
      winner: eventDetails.winner,
      playerStake: playerStake,
      active: eventDetails.active,
      closed: eventDetails.closed,
      paid: eventDetails.paid
    });

    return eventDTO;
  }

  // ░█████╗░░█████╗░███╗░░░███╗███╗░░░███╗██╗░██████╗░██████╗██╗░█████╗░███╗░░██╗
  // ██╔══██╗██╔══██╗████╗░████║████╗░████║██║██╔════╝██╔════╝██║██╔══██╗████╗░██║
  // ██║░░╚═╝██║░░██║██╔████╔██║██╔████╔██║██║╚█████╗░╚█████╗░██║██║░░██║██╔██╗██║
  // ██║░░██╗██║░░██║██║╚██╔╝██║██║╚██╔╝██║██║░╚═══██╗░╚═══██╗██║██║░░██║██║╚████║
  // ╚█████╔╝╚█████╔╝██║░╚═╝░██║██║░╚═╝░██║██║██████╔╝██████╔╝██║╚█████╔╝██║░╚███║
  // ░╚════╝░░╚════╝░╚═╝░░░░░╚═╝╚═╝░░░░░╚═╝╚═╝╚═════╝░╚═════╝░╚═╝░╚════╝░╚═╝░░╚══╝

  // ░██████╗██╗░░██╗░█████╗░██████╗░██╗███╗░░██╗░██████╗░
  // ██╔════╝██║░░██║██╔══██╗██╔══██╗██║████╗░██║██╔════╝░
  // ╚█████╗░███████║███████║██████╔╝██║██╔██╗██║██║░░██╗░
  // ░╚═══██╗██╔══██║██╔══██║██╔══██╗██║██║╚████║██║░░╚██╗
  // ██████╔╝██║░░██║██║░░██║██║░░██║██║██║░╚███║╚██████╔╝
  // ╚═════╝░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░╚═╝╚═╝╚═╝░░╚══╝░╚═════╝░
  /**
   * @dev Accumulates commission generated from swaps and stores it as ATON tokens.
   * @param newCommissionATON The commission amount in ATON tokens.
   * @notice The commission per token is updated based on the total supply of ATON tokens.
   * @dev Assumes `totalSupply()` is non-zero, otherwise results in a no-op.
   */
  function _accumulateCommission(uint256 newCommissionATON) internal {
    uint256 totalSupplyTokens = totalSupply();

    // Ensure no division by zero
    if (totalSupplyTokens > 0) {
      accumulatedCommissionPerToken += (newCommissionATON * (10 ** decimals())) / totalSupplyTokens;
      totalCommissionInATON += newCommissionATON;
      emit EventsLib.Accumulate(newCommissionATON, accumulatedCommissionPerToken, totalCommissionInATON);
    }
  }

  /**
   * @dev Distributes accumulated commission to a specified player based on their ATON token holdings.
   * The distribution ensures players receive their share of profits from the accumulated commission.
   * @param player Address of the player receiving the commission.
   * @notice The commission is transferred to the player or the owner if `player` is the contract itself.
   */
  function _distributeCommission(address player) internal {
    uint256 unclaimedCommission = _playerCommission(player);

    if (unclaimedCommission > 0) {
      if (player == address(this)) {
        // Transfer commission to the owner when the contract itself is the player
        super._transfer(address(this), owner, unclaimedCommission);
        players[address(this)].claimedCommissionsByPlayer += unclaimedCommission;
      } else {
        // Transfer commission directly to the player
        super._transfer(address(this), player, unclaimedCommission);
        players[player].claimedCommissionsByPlayer += unclaimedCommission;
      }
    }

    // Update player's last known commission per token to the current accumulated value
    players[player].lastCommissionPerTokenForPlayer = accumulatedCommissionPerToken;
  }

  /**
   * @dev Computes the unclaimed commission for a specified player based on their ATON token holdings.
   * @param player Address of the player.
   * @return unclaimedCommission The amount of ATON tokens the player can claim as commission.
   * @notice The calculation is based on the difference between the global accumulated commission per token
   * and the player's last recorded commission per token, scaled by the player's ATON holdings and adjusted by `pct_denom` for precision.
   */
  function _playerCommission(address player) internal view returns (uint256) {
    // Calculate the difference in commission per token since the last update for the player
    uint256 owedPerToken = accumulatedCommissionPerToken - players[player].lastCommissionPerTokenForPlayer;

    // Calculate the total commission owed to the player, scaling by `pct_denom` to maintain precision
    uint256 unclaimedCommission = (balanceOf(player) * owedPerToken * pct_denom) / (10 ** decimals());

    // Return the unclaimed commission, adjusted by `pct_denom`, or 0 if no commission is owed
    return unclaimedCommission > 0 ? unclaimedCommission / pct_denom : 0;
  }

  /**
   * @dev Allows a player to donate ATON tokens to the contract. The donated amount is converted to the
   * total commission pool, which can then be distributed to ATON holders.
   * @notice The function requires the transaction to include Ether. The Ether is converted into ATON
   * and credited to the contract, increasing the total ATON supply.
   */
  function donateATON() external payable {
    uint256 amount = msg.value;

    // Ensure the transaction includes some Ether to donate
    require(amount > 0, "Must send some Ether");

    // Mint an equivalent amount of ATON tokens to the contract address
    _mint(address(this), amount); // Ensure _mint is correctly defined

    // Add the donated amount to the total accumulated commission
    _accumulateCommission(amount);

    // Emit an event indicating that ATON tokens have been donated to the contract
    emit EventsLib.ATONDonated(msg.sender, amount);
  }

  /**
   * @dev Transfers ATON tokens from the sender to the specified recipient.
   * Before executing the transfer, any unclaimed commissions are distributed to both the sender and the recipient.
   * @param to The address of the recipient who will receive the ATON tokens.
   * @param value The amount of ATON tokens to transfer.
   * @return A boolean value indicating whether the operation succeeded.
   * @notice The transfer function ensures that all pending commissions are distributed before any token movement occurs.
   */
  function transfer(address to, uint256 value) public virtual override returns (bool) {
    // Distribute unclaimed commissions to both the sender and the recipient before the transfer
    _distributeTransfer(msg.sender, to, value);

    return true;
  }

  /**
   * @dev Handles the transfer of ATON tokens and the distribution of any unclaimed commissions to the involved addresses.
   * This internal function first ensures that both the sender and recipient receive any commissions owed to them
   * before completing the transfer of tokens.
   * @param from The address of the sender transferring ATON tokens.
   * @param to The address of the recipient receiving ATON tokens.
   * @param amount The amount of ATON tokens to transfer.
   * @notice The function ensures that the owner receives any commissions when the contract address is involved.
   */
  function _distributeTransfer(address from, address to, uint256 amount) internal {
    // Distribute any unclaimed commission to the owner if the contract is the sender or recipient
    if (from == address(this) || to == address(this)) {
      _distributeCommission(owner);
    }

    // Distribute unclaimed commission to the sender
    _distributeCommission(from);

    // Distribute unclaimed commission to the recipient
    _distributeCommission(to);

    // Execute the transfer of ATON tokens from the sender to the recipient
    super._transfer(from, to, amount);
  }

  // ░██████╗░██╗░░░░░░░██╗░█████╗░██████╗░
  // ██╔════╝░██║░░██╗░░██║██╔══██╗██╔══██╗
  // ╚█████╗░░╚██╗████╗██╔╝███████║██████╔╝
  // ░╚═══██╗░░████╔═████║░██╔══██║██╔═══╝░
  // ██████╔╝░░╚██╔╝░╚██╔╝░██║░░██║██║░░░░░
  // ╚═════╝░░░░╚═╝░░░╚═╝░░╚═╝░░╚═╝╚═╝░░░░░

  /**
   * @dev Swaps ATON tokens for ETH at a 1:1 ratio.
   * @param _amountAton The amount of ATON tokens to swap.
   * @return success True if the swap was successful.
   */
  function swap(uint256 _amountAton) external nonReentrant returns (bool success) {
    require(_amountAton > 0, "Swap amount must be greater than zero");
    require(balanceOf(msg.sender) >= _amountAton, "Insufficient ATON balance");
    require(address(this).balance >= _amountAton, "Contract has insufficient ETH balance");

    // Step 1: Transfer ATON tokens to the contract
    _distributeTransfer(msg.sender, address(this), _amountAton);

    // Step 2: Burn the ATON tokens from the contract to maintain the 1:1 swap mechanism
    _burn(address(this), _amountAton);

    // Step 3: Transfer Ether to the sender (after state changes)
    (bool sent, ) = msg.sender.call{ value: _amountAton }("");
    require(sent, "Failed to send ETH");

    // Emit the swap event after successful transfer
    emit EventsLib.Swap(msg.sender, _amountAton);

    return true;
  }
}

// This software is provided "as is", without warranty of any kind, express or implied,
// including but not limited to the warranties of merchantability, fitness for a particular purpose,
// and noninfringement. In no event shall the authors or copyright holders be liable for any claim,
// damages, or other liability, whether in an action of contract, tort, or otherwise, arising from,
// out of, or in connection with the software or the use or other dealings in the software.
