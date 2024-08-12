# xkcdpasswd.rs
Honestly, this is just a simple implementation of [xkcd #936](https://xkcd.com/936/) about password strength. It's mainly intended as a project to help me get my feet wet with Rust.

Project inspired by [Zi-SH's xkpass.io](https://github.com/Zi-SH/xkpass-io), although as a "clean room" reimplementation. 

# Installing

Simply run `cargo run` while in the directory.

# Credits

The word list has been taken from [Josh Kaufman's 10,000 most common English words](https://github.com/first20hours/google-10000-english) list, based off Google's Trillion Word Corpus.

I've also reused Zi-SH's filtering on the list to get rid of individual letters, albeit to > 1 letter than 0.

`$ curl https://raw.githubusercontent.com/first20hours/google-10000-english/master/google-10000-english-no-swears.txt | awk 'length($0) > 2' | sort`