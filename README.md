# cache_cleaner
This is a small rust program to clear cache files on my system. 
This is ment only as a tool for me to learn rust and automate 
my systems deleting of cache.

Though if you still want to use it procced at your own risk.
There is a basic config in place.

The program wont delete system files unless running as root. 
It has a hand full of debug/verbose options avablible.

It will detect the current running user and append its home path
to user files and folders. When running as root the program will 
parse /etc/passwd for all users. Then run the cache cleaing on 
every user account. - On windows this feature is not implemented yet.

Right now the tool will only skip over users that have a /, and a few
others like /dev/null for a home path. The reason for this, it because the 
tool needs to run on my uncle's system as well. He has user with
ids less than 1000 and in weird place like /x or /c/down/z, you
get the point. 

For the parser I'm using a handful of xml files.

Depends:

    I'm building this against the latest stabel rust. 

to build:

    git clone https://github.com/BearzRobotics/cache_cleaner.git
    cd cache_cleaner
    cargo run

to install:
    Root must have access to rustc and cargo. So if you installed rust via rustup,
    you also have to do that for your root account. If you apt installed or eopkg installed
    rust you'll be just fine.

    sudo make install

## To Do:

-[x] implement basic xml layout 

-[] read events from files 

-[] create backends for the xml events 

-[] get admin support of windows working 

-[] find a way to preduce installer for windows. 
