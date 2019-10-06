# Blackjack Design Document
This document details the design and goals for a command-line interface (CLI) implementation of the card game blackjack.

## Language
This implementation of blackjack will be written in Rust. Though there are valid arguments to be made that such a strict typing system can hinder velocity on a relatively simple project such as this, I have chosen Rust based on two main criteria.
- An expressive type system encourages thorough consideration of contracts between application objects, which is useful when bringing together separate components of the game.
- Strong compile-time checks such as for reference lifetime validity and match exhaustion prevent simple, yet time-consuming errors that can still find their way into simple projects.

## Features & Scope

This version of blackjack will place the user against the dealer with no other players (either AI or pass-and-play). Additionally, to reduce the complexity and scope of the project, there will be no betting. Besides calculating payouts and validating user bets, this gets rid of a mechanics such as doubling down and better justifies the absence of others, such as splitting.

In particular, this version of blackjack will be basic, rendering cards to the screen for the user and then asking them to decide between hitting and staying. The dealer will play by the standard dealer algorithm (hit while below 17, then stay).

## Breakdown

The implementation will separate the game logic into clearly defined, well separated components.

### Cards

The most basic unit of game logic in blackjack is the **Card**. Though technically suit is not needed for blackjack, it is included as it gives familiarity to blackjack players who know the game from having played with physical cards and gives a good excuse to use the unicode suit characters.

The card module will also include logic for a **Hand**—which besides encapsulating the structural concept of a collection ([vector]([https://doc.rust-lang.org/std/vec/struct.Vec.html](https://doc.rust-lang.org/std/vec/struct.Vec.html))) of cards also provides functionality such as accepting a new card and calculating the value of the collection of cards according to blackjack rules.

### Deck
Once the notion of a card is defined, building the **Deck** follows naturally. A deck, like a hand, also behaves much like a collection of cards, but also includes a second collection of "discarded" cards, so that the game can make its way through the whole deck before reshuffling. The **Deck** also includes functionality to support randomly shuffling and dealing cards into **Hand**s.

### Agent
So as to avoid confusion with a **Player**, which is the term that will be used to refer to the human player, we use the term **Agent** to refer to a player of blackjack in general. An **Agent** is a contract that defines public interfaces into a hand, and a means of determining whether the implementing object will hit or stay depending on the state of the game. This module then uses this contract to define specific **Player** and **Dealer** objects that obey this contract and take actions based on user input and standard blackjack dealer rules respectively.

### Game

With the above primitives defined, the structural definition of a **Game** object and the functionality that facilitates a cohesive game of blackjack is defined in the game module. This includes the logic that continually asks an **Agent** what action they want to take until they either bust or stay, and also the logic that defines what set of actions occur over the course of a single round.

## CLI Interface

All game information is printed to the console. Often, information is preceded by the name of the **Agent** to which it pertains (either "Player" or "Dealer") to clarify the happenings over the course of a round. The user only ever needs to give input to answer two questions: 
- does the user want to hit or stay?
- does the user want to play another round? 

Both of these questions are multiple choice (they are coincidentally both binary decisions) and are presented to the user through a [dialoguer]([https://github.com/mitsuhiko/dialoguer](https://github.com/mitsuhiko/dialoguer)) select menu, which allows the user to use the arrow keys to highlight the option they desire and then hit return to select it. In particular, we opt for this over a simple textual command line input because it is a simpler user experience and also removes the need for user input validation logic.

## Example Round
```
Dealer showing J♣
Player cards:
Q♠ 5♣
Current value: 15
Pick an action: Hit

Player: hit and got a 3♦
Player: value is 18
Dealer showing J♣
Player cards:
Q♠ 5♣ 3♦
Current value: 18
Pick an action: Stay

Player: stayed at 18
Dealer cards:
Q♠ 5♣ 3♦
Value: 18
Dealer: hit and got a 7♣
Dealer: value is 19
Dealer: stayed at 19
Dealer wins

Would you like to play another round?:
> Yes
  No
```
