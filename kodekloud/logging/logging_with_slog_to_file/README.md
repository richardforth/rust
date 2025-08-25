# Notes on using slog to log to a file

While this is in the Kodekloud section, it wasnt actually covered in the course. I just got curious, and with a little help from the docs:

https://docs.rs/slog/latest/slog/#where-to-start

And some AI assistance creating the missing 'logs' directory, I as able to kermudgeon a script together to log to a log file.

## Kodekloud logging coverage

The rust course on Kodekloud covered logging to the console with log and slog (see the other two directories).
It didnt cover logging to a file, so I did my own research and while this isnt the best example, it works as an MVP.*

## Improvements

I could improve the logging functionality by perhaps having the logging code in a separate logging module or library crate, which would theoreticallky make logging trivial to add to any rust project by reusing code.



* Minimum Viable Product 
