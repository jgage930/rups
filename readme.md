## rups
### Rust Univeral Password Safe
A password safe written in rust.

### TODO
- set up arguments in clap


~~- create a password
    - accept command line arguments
    - encrypt password, store in db
    - DONE ~~

- config
    - set up a config file to hold vars
        - path to encrypt key
        - path to db
        - master user?

- crud operations
    ~~ - get single password by id DONE ~~
    - list all passwords DONE
    - search for a password
        - auto complete set up for passwords in db
    - update password
        - option to update one field
        - option to open password in a file and edit
    - delete password by id

- set up admin user and password
    - handle authentication for the entire safe

- session system
    - allow users to login in once and stay logged in for some amount of time

- Tech debt
    - clean up
        - reafactor to use one subcrate for all logic
        - clean up main.rs
        - handler funcs for the different commands
