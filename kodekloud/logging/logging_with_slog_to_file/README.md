# Notes on using slog to log to a file

While this is in the Kodekloud section, it wasn't actually covered in the course (logging to a file). We looked at log and slog usage, but only to the console and I just got curious, and with a little help from the docs:

https://docs.rs/slog/latest/slog/#where-to-start

And some AI assistance creating the missing 'logs' directory, I as able to kermudgeon a script together to log to a log file.

## Kodekloud logging coverage

The rust course on Kodekloud covered logging to the console with log and slog (see the other two directories).
It didnt cover logging to a file, specifically, so I did my own research and while this isn't the best example, it works as an MVP*.  It uses slog, but instead of printing to the console, writes to a log file.

## Improvements

I could improve the logging functionality further by perhaps having the logging code in a separate logging module or library crate, which would theoretically make logging trivial to add to any rust project by reusing code. See Backyard or Restaurant under trpl (The Rust Programming Language) for examples of modular designs.

## TODO LIST:

- Add a new cargo project but with a logging module, it needs at least two interfaces:
  - One to set the log directory path
  - One to set the logfile name

- Maybe other things (that may or may not be taken care of by the slog macros):
  - debug! logging
  - info! logging
  - warn! logging
  - error! logging

* Minimum Viable Product 
