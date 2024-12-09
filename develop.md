# Reindeer Hunt Project Documentation

## lib.rs

This file contains the main entry point for the Tauri application. It initializes the Tauri builder and registers various plugins and commands.

### Functions
- **run**: Initializes the Tauri application, registers plugins for file system, shell, and dialog functionalities, and sets up the command handlers for various functionalities provided by the `filemanager` and `manager` modules.

## main.rs

This file contains the main function that starts the Tauri application.

### Functions
- **main**: Calls the `run` function from the `reindeer_hunt_lib` module to start the Tauri application.

## filemanager.rs

This file contains functions related to file management, such as saving and opening configuration and cache files, and importing data from files.

### Functions
- **save_times**: Saves the provided JSON content to the configuration file.
- **open_times**: Opens the configuration file and returns its content as JSON. If the file does not exist, it creates a default configuration.
- **import**: Imports student data from a specified file and returns it as JSON.
- **open**: Opens a specified file and returns its content as JSON.
- **save_cache**: Saves the provided JSON content to the cache file.
- **open_cache**: Opens the cache file and returns its content as JSON. If the file does not exist, it creates a default cache.

## manager.rs

This file contains functions related to managing student data, such as adding, removing, and updating students, as well as handling their statuses and pairings.

### Functions
- **remove_student**: Removes a student from the provided JSON content based on their name, last name, and homeroom.
- **remove_lost**: Removes students who have lost from the provided JSON content and resets their statuses.
- **loss**: Updates the status of a student to "loss" based on their name, last name, and homeroom.
- **win**: Updates the status of a student to "win" based on their name, last name, and homeroom.
- **reset_status**: Resets the status of a student to "undecided" based on their name, last name, and homeroom.
- **pair_students**: Pairs students randomly and updates their pair information in the provided JSON content.
- **coin_toss**: Performs a coin toss to determine the win/loss status of students.
- **single_toss**: Performs a coin toss for a single student to determine their win/loss status.
- **next_round**: Prepares the next round by performing a coin toss for all students, resetting their statuses, pairing them, and sorting them.
- **sort_students**: Sorts students based on their homeroom, last name, and first name.
- **add_student**: Adds a new student to the provided JSON content.
- **update_notes**: Updates the notes of a student based on their name, last name, and homeroom.
- **end_program**: Ends the program by exiting the process.

## App.jsx

This file contains the main React component for the application. It manages the state and handles various actions related to students and their statuses.

### Functions
- **App**: The main React component that initializes state variables and defines various functions to handle student-related actions such as adding, removing, updating, and pairing students, as well as handling their statuses and notes.
- **handleStudentClick**: Handles the click event on a student, setting the state variables with the student's information.
- **quit**: Saves the current state to cache and ends the program.
- **incrementButtonState**: Increments the button state, cycling through predefined states.
- **openTimes**: Opens and sets the hunt times from the configuration.
- **saveTimes**: Saves the current hunt times to the configuration.
- **open_cache**: Opens and sets the student data from the cache.
- **save_cache**: Saves the current student data to the cache.
- **addStudent**: Adds a new student to the list and saves the updated list to the cache.
- **saveStudent**: Updates the notes and status of a student and saves the changes to the cache.
- **openDia**: Opens a file dialog to select a CSV file and sets the path.
- **removeLost**: Removes students who have lost from the list and saves the updated list to the cache.
- **studentWin**: Updates a student's status to "win" and saves the changes to the cache.
- **studentLoss**: Updates a student's status to "loss" and saves the changes to the cache.
- **resetStatus**: Resets a student's status to "undecided" and saves the changes to the cache.
- **importDia**: Opens a file dialog to select a CSV file, imports student data from the file, and saves the updated list to the cache.
- **sortStudents**: Sorts the students and saves the sorted list to the cache.
- **removeStudent**: Removes a student from the list and saves the updated list to the cache.
- **OpenJSON**: Opens a file dialog to select a JSON file, loads student data from the file, and saves the updated list to the cache.
- **savehunts**: Saves the current student data to a JSON file.
- **nextRound**: Prepares the next round by randomizing, removing lost students, pairing students, and saving the updated list to the cache.
- **pair**: Pairs students randomly and saves the updated list to the cache.
- **singleToss**: Performs a coin toss for a single student and saves the updated list to the cache.
- **updateNotes**: Updates the notes of a student and saves the changes to the cache.
- **randomize**: Randomizes the win/loss status of students and saves the changes to the cache.
- **genpermits**: Generates permits for students and saves them as a PDF file.

