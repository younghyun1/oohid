# oohid
Personal project - make a nice GUI UUID generator using Iced and Rayon on Rust. CLI prototype first.

Currently multithreaded using Rayon's auto threadpool. Pretty good, generates a million a second depending on arguments.

Command line arguments:

--help: displays clap help

--check: checks results for dupes. unlikely, but yeah

-c: # of uuids to gen. it's all v4

-f: formatting options. please refer to --help

--verbose: outputs checking and runtime result