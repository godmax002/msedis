
#[cfg(unix)]
extern crate syslog;

use std::fmt::{Debug, Error, Formatter};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{stderr, stdout, Write};
use std::iter::FromIterator;
use std::path::Path;
use std::process;
use std::sync::mpsc::{channel, Sender};
use std::thread;

enum Output {
    Channel(Sender<Vec<u8>>),
    Stdout,
    Stderr,
    File(File, String),
}

impl Debug for Output {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error>{
        match *self {
            Output::Channel(_) => fmt.write_str("Channel"),
            Output::Stdout => fmt.write_str("Stdout"),
            Output::Stderr => fmt.write_str("Stderr"),
            Output::File(_, ref filename) => fmt.write_fmt(format_args!("File {}", filename)),
        }
    }
}

impl Write for Output {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        match *self {
            Output::Channel(ref v) => {
                v.send(Vec::from_iter(data.iter().cloned())).unwrap();
                Ok(data.len())
            },
            Output::Stdout => stdout().write(data),
            Output::Stderr => stderr().write(data),
            Output::File(ref mut f, _) => f.write(data),
        }

    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Output::Channel(_) => Ok(()),
            Output::Stdout => stdout().flush(),
            Output::Stderr => stderr().flush(),
            Output::File(ref mut f, _) => f.flush(),
        }
    }
}

impl Clone for Output{
    fn clone(&self) -> Self {
        match *self {
            Output::Channel(ref v) => Output::Channel(v.clone()),
            Output::Stdout => Output::Stdout,
            Output::Stderr => Output::Stderr,
            Output::File(_, ref path) => Output::File(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path)
                    .unwrap(),
                path.clone()
            ),
        }
    } 
}

#[derive(PartialEq, Clone, Debug)]
pub enum Level {
    Debug,
    Verbose,
    Notice,
    Warning,
}

impl Level {
    pub fn contains(&self, other: &Level) ->bool {
        match *self {
            Level::Debug => true,
            Level::Verbose => *other != Level::Debug,
            Level::Notice => *other == Level::Notice || *other == Level::Warning,
            Level::Warning => *other == Level::Warning,
        }
    }
}

pub struct Logger {
    tx: Sender<(
        Option<Output>,
        Option<Level>,
        Option<String>,
        Option<Option<Box<syslog::Logger>>>,
        Option<i32>
    )>
}

impl Logger {
    fn create(level: Level, output: Output) -> Logger {
        let (tx, rx) = channel::<(
            Option<Output>,
            Option<Level>,
            Option<String>,
            Option<Option<Box<syslog::Logger>>>,
            Option<i32>
        )>();
        {
            let mut level = level;
            let mut output = output;
            let mut syslog_writer: Option<Box<syslog::Logger>>  = None;
            thread::spawn(move || {
                while let Ok((out, lvl, msg, syslog, code)) = rx.recv() {
                    match (out, lvl, msg, syslog) {
                        (_, Some(lvl), Some(msg), _) => {
                            if level.contains(&lvl) {
                                match write!(output, "{}", format!("{}\n", msg)) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        eprint!("Failed to log {:?} {}", e, msg);
                                    }
                                }
                                if let Some(ref mut w) = syslog_writer {
                                    match w.send_3164(
                                        match lvl {
                                            Level::Debug => syslog::Severity::LOG_DEBUG,
                                            Level::Verbose => syslog::Severity::LOG_INFO,
                                            Level::Notice => syslog::Severity::LOG_NOTICE,
                                            Level::Warning => syslog::Severity::LOG_WARNING,
                                        },
                                        msg.clone()
                                    ) {
                                        Ok(_) => (),
                                        Err(e) => {
                                            eprint!("Failed to log {:?} {}", e, msg);
                                        }
                                    }
                                }
                            }
                        }
                        (_, Some(lvl), _, _) => {
                            level = lvl;
                        }
                        (Some(out), _, _, _) => {
                            output = out;
                        }
                        (_, _, _, Some(syslog)) => {
                            syslog_writer = syslog;
                        }
                        (out, lvl, msg, syslog) => {
                            panic!("Unknown messages {:?}", (out, lvl, msg, syslog.is_some()));
                        }
                    }

                    if let Some(code) = code {
                        process::exit(code);
                    }
                }
            });

        }
        Logger { tx }
    }
    
}
