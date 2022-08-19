## Minigrep Project
A Rust built `grep` alternative.

### Build
>`cargo build`

### Or Run
>`cargo run SEARCH_TERM FILENAME`

Example:
>`cargo run soul test.txt`

### Symlink for MacOS
Run the following after running the Build command: 
>`ln -s /Path-to-project-root/target/debug/minigrep /usr/local/bin`

Then run you can run the following from anywhere:
>`minigrep SEARCH_TERM FILENAME`
