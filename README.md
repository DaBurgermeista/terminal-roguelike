# Terminal Roguelike

A turn-based, text-based roguelike written in Rust.

The game is a text-adventure dungeon crawler with roguelike-inspired systems. Players explore through room descriptions and typed commands rather than a graphical or ASCII map.

The focus is on turn-based exploration, combat, procedural dungeon content, items, inventory, risk, and replayability.

## Project Goals

- Build a playable single-player dungeon crawler in Rust.
- Use turn-based actions.
- Use typed text commands rather than real-time input.
- Generate dungeon content randomly.
- Include monsters, combat, items, inventory, and multiple dungeon levels.
- Keep the game runnable with one command:

### Milestone 1: Project Foundation
- [x] Create Rust projects with cargo
- [x] Create GitHub repository
- [x] Add `README.md`
- [x] Add `.gitignore`
- [x] Configure stable Rust toolchain
- [x] configure `cargo run` works in google shell

### Milestone 2: Early Movement Prototype

- [x] Render a static ASCII dungeon prototype
- [x] Store player position separately from the map
- [x] Add typed movement commands
- [x] Prevent movement through walls
- [x] Add basic game messages
- [x] Confirm stable line-based input in Cloud Shell

> The ASCII map was a learning prototype. The planned game will use text descriptions and typed commands rather than a displayed map.

### Milestone 3: Text Adventure Foundation

- [x] Remove the ASCII-map interface
- [x] Create a `Room` data structure
- [x] Create a player location represented by a room
- [x] Create a named starting room
- [x] Add a room description
- [ ] Add exits between rooms
- [ ] Add north, south, east, and west movement commands
- [ ] Display available exits
- [ ] Reject movement where no exit exists
- [ ] Add a `look` command
- [ ] Add a `help` command
- [ ] Create three connected hand-written rooms

### Milestone 4: Command System

- [ ] Create a dedicated command parser
- [ ] Accept short movement commands: `n`, `s`, `e`, `w`
- [ ] Accept long movement commands: `north`, `south`, `east`, `west`
- [ ] Add a `status` command
- [ ] Add an `inventory` command
- [ ] Handle unknown commands clearly
- [ ] Add command aliases where useful

### Milestone 5: Player, Turns, and Status

- [ ] Create a `Player` struct
- [ ] Add player name
- [ ] Add hit points
- [ ] Add maximum hit points
- [ ] Add attack and defense values
- [ ] Add a turn counter
- [ ] Make each valid player action consume one turn
- [ ] Display player status

### Milestone 6: Combat

- [ ] Create an `Enemy` struct
- [ ] Add one enemy type: goblin
- [ ] Add enemy health and attack values
- [ ] Add an `attack` command
- [ ] Add basic hit and damage calculations
- [ ] Add enemy actions after player turns
- [ ] Add combat messages
- [ ] Add enemy death
- [ ] Add player death and game-over behavior

### Milestone 7: Items and Inventory

- [ ] Create an `Item` struct
- [ ] Add items to rooms
- [ ] Add a healing potion
- [ ] Add a `take` command
- [ ] Add inventory storage
- [ ] Add a `use` command
- [ ] Add item descriptions
- [ ] Add item effects
- [ ] Add enemy loot drops

### Milestone 8: Dungeon Generation

- [ ] Create randomly generated room names and descriptions
- [ ] Generate connected rooms
- [ ] Generate exits between rooms
- [ ] Ensure every generated room can be reached
- [ ] Place enemies randomly
- [ ] Place items randomly
- [ ] Add a dungeon entrance
- [ ] Add stairs or an exit to the next dungeon level
- [ ] Add dungeon depth

### Milestone 9: Progression and Replayability

- [ ] Add multiple enemy types
- [ ] Add stronger enemies at deeper dungeon levels
- [ ] Add experience points
- [ ] Add player levels
- [ ] Add improved player statistics
- [ ] Add equipment
- [ ] Add randomized loot
- [ ] Add victory or end-game conditions
- [ ] Add restart-after-death behavior

### Milestone 10: Saving and Polish

- [ ] Choose a save-file format
- [ ] Save player state
- [ ] Save dungeon state
- [ ] Add a `save` command
- [ ] Add load-on-start behavior
- [ ] Handle invalid save files safely
- [ ] Improve text descriptions
- [ ] Improve help text
- [ ] Add tests for movement, combat, and inventory
- [ ] Balance enemy and item values
- [ ] Write final setup and play instructions

## Run

```bash
cargo run
