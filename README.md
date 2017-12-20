# cache_cleaner
This is a small rust program to clear cache files on my system. This is ment only as a tool for
me to learn rust and automate my systems deleting of cache.

Though if you still want to use it procced at your own risk.
There is a basic config in place, but still vary bugy and not working properly

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