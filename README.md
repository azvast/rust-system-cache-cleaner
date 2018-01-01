# cache_cleaner
This is a small rust program to clear cache files on my system. 
This is ment only as a tool for me to learn rust and automate 
my systems deleting of cache.

Though if you still want to use it procced at your own risk.
There is a basic config in place, but still vary bugy and not 
working properly

The program wont delete system files unless running as root. 
It has a hand full of debug/verbose options avablible.

It will detect the current running user and append its home path
to user files and folders. When running as root the program will 
parse /etc/passwd for all users. Then run the cache cleaing on 
every user account.

Right now the tool will only skip over users that have a / or 
/dev/null for a home path. The reason for this, it because the 
tool needs to run on my uncle's system as well. He has user with
ids less than 1000 and in weird place like /x or /c/down/z, you
get the point. 

Note: 

    I'm building this against the musl target. I'm doing this so 
    generates a static binary file. This is important to me because 
    this program is going to be running when the system starts up 
    and I don't want it to have to load its dependencies.

Depends:

    I'm building this against rust 1.22.1.

to build:

    git clone https://github.com/BearzRobotics/cache_cleaner.git
    cd cache_cleaner
    cargo run

to install:
    Root must have access to rustc and cargo. So if you installed rust via rustup,
    you also have to do that for your root account. If you apt installed or eopkg installed
    rust you'll be just fine.

    sudo make install