# Docs

See docs/explanation.md for a description of the program and how it works.

# How to run

The program is built in rust, so requires a rust installation. See https://www.rust-lang.org/ for installation instructions. This will come with the package manager 'Cargo'.

Build the program with `$ cargo build --release`

Run the executable: `$ ./target/release/asgn1.exe > output.txt`
(or leave output pipe blank for the program to print to terminal)

The program accepts a single line of text through stdin as the search query.

## File setup

You can change the program to parse a document collection to create an inverted index by changing the MODE variable (in ./src/main.rs) to Mode::Indexer.

Please also change the WSJ_PATH variable at the top of ./src/lib.rs to point to the document collection being used.
This is important for parsing as well as searching, as the DOC_IDs are read from that original collection file.

### Inverted index location

The index folder contains all files and subfolders required for searching. Place the index folder (with everything inside) in the rust root directory (at the same level as the readme.md).
This is stored on the department server (`/home/nicta609/courses/431/asgn1/index`), or can be created by running the program in indexer mode in roughly 70 seconds.
