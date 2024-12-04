# Project Documentation

## Overview
This project is a Tauri application that manages a reindeer hunt game. The application allows users to import, save, and manage student data, pair students for the game, and handle game rounds.

## File Descriptions

### `lib.rs`
This file is the entry point for the Tauri application. It initializes the Tauri builder, adds necessary plugins, and sets up the command handlers for various functionalities provided by the `filemanager` and `manager` modules.

### `filemanager.rs`
This module handles file operations such as importing, saving, and opening files. It provides the following commands:
- `import`: Reads student data from a CSV file and converts it to JSON format.
- `save`: Saves JSON data to a specified file.
- `open`: Opens a file and reads its contents as a string.

### `manager.rs`
This module manages the core game logic for the reindeer hunt. It provides the following commands:
- `remove_student`: Removes a student from the list.
- `loss`: Marks a student as having lost a round.
- `win`: Marks a student as having won a round.
- `pair_students`: Pairs students randomly for the game.
- `coin_toss`: Simulates a coin toss to determine the outcome of a round for each student.
- `next_round`: Prepares the students for the next round and pairs them again.
- `sort_students`: Sorts students by room, last name, and first name.

### `main.rs`
This file contains the main function, which serves as the entry point for the application. It calls the `run` function from the `reindeer_hunt_lib` module to start the Tauri application.
