# microdb - yet another tiny SQLlite-like database

MicroDB is a simple database implemented in C++ that supports basic SQL commands such as `insert` and `select`. It uses a basic B-Tree structure for data storage and is designed as a learning project to explore file I/O, memory management, and tree-based data structures. Best way to learn anything is by doing.

## Features

- **Basic SQL Commands:**  
  - `insert <id> <username> <email>`  
  - `select`

- **Meta-commands:**  
  - `.exit` to close the database.  
  - `.btree` to print the current B-Tree structure.  
  - `.constants` to print configuration constants.

- **Persistent Storage:**  
  Data is stored in a file, and the database retains data between sessions.

## Dependencies

- **C++11 or higher**
- **CMake (minimum version 3.10)**
- **POSIX Libraries:** Used for file I/O and other system operations.
- **Catch2:** A header-only C++ testing framework (fetched automatically by CMake).

## Building the Project

To build the project on macOS (or any system with CMake):

1. Open a terminal in the repository's root directory.
2. Create and enter a build directory:

 ```bash
 mkdir build
 cd build
```

3. Configure the project with CMake:

`cmake ..`

 4. Build the project:

`cmake --build .`

This will generate two executables:
 • db — The main MicroDB database binary.
 • tests — The test executable for running automated tests.

Running the Database

To run the database interactively, execute the db binary with a database filename as an argument:

`./db mydatabase.db`

At the db > prompt you can type commands such as:
 • Insert a row:

`insert 1 user1 <person1@example.com>`

 • Retrieve all rows:

`select`

 • Other meta-commands:

`.btree`
`.constants`
`.exit`

Running the Tests

The project uses Catch2 for automated testing. You can run the tests in one of two ways:

Using the Test Executable (easiest)

From the build directory, run:

`./tests`

Using CTest

From the build directory, run:

`ctest`

The tests simulate user input by writing commands to a temporary file and piping that input to the db binary. The output is then checked against expected values.

This project is inspired by various educational resources on building simple databases. It serves as a hands-on exploration of low-level file and memory management, as well as tree data structures.
