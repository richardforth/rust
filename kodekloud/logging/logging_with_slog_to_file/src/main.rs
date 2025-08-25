//#[macro_use]

//extern crate slog;
//extern crate slog_term;
//extern crate slog_async;

use std::fs::OpenOptions;
use slog::Drain;
use slog::o;

// log levels
use slog::debug;
use slog::info;
use slog::warn;
use slog::error;

// for log directory check / creation
use std::fs;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
   let log_path = "logs/app.log"; // if 'logs' directory doesnt exist, causes panic on open
   let log_dir = Path::new("logs");
   // TODO: Check if log dir exists, if not create it
   if !log_dir.exists() {
       // create_dir_all will create intermediate directories if needed
       fs::create_dir_all(log_dir)?;
       println!("Created log directory: {:?}", log_dir);
   } else {
       println!("Log directory already exists: {:?}", log_dir);
   }
   let file = OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(log_path)
      .unwrap();

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());

    info!(log, "Application started"; "version" => "1.0.0");
    let name = String::from("Richard");
    debug!(log, "Variable set"; "name" => name.clone());
    let message = format!("Hello, {}.", name.clone());
    println!("{}", message);
    debug!(log, "Printed message to screen"; "message" => message.clone());
    warn!(log, "This is a warning message";);
    error!(log, "This is an error message";);
    info!(log, "Application Finished"; "version" => "1.0.0");
  
    Ok(())
}
