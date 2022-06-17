### Status

[![Build Status](https://travis-ci.org/BearzRobotics/cache_cleaner.png)](https://travis-ci.org/BearzRobotics/cache_cleaner)

# system cache cleaner

This is a small rust program to clear cache files on my system.
This is ment only as a tool for me to learn rust and automate
my systems deleting of cache.

Though if you still want to use it procced at your own risk.
There is a basic config in place.

The program wont delete system files unless running as root.
It has a handfull of debug/verbose options avablible.

It will detect the current running user and append its home path
to user files and folders. When running as root the program will
parse /etc/passwd for all users. Then run the cache cleaing on
every user account. - On windows this feature is not implemented yet.

Right now the tool will only skip over users that have a /, and a few
others like /dev/null for a home path. The reason for this, it because the
tool needs to run on my uncle's system as well. He has user with
ids less than 1000 and in weird place like /x or /c/down/z, you
get the point.

# Depends:

    I'm building this against the latest stabel rust.

# to build:

    git clone https://github.com/azvast/rust-system-cache-cleaner.git
    cd cache_cleaner
    cargo run

## To Do:

- [x] create basic crawler config file
- [x] implement the parser for the crawler file
- [ ] create backends for the config events
- [ ] get admin support of windows working
- [ ] find a way to preduce installer for windows.
