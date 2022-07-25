extern crate nix;
use caps::Capability;
use nix::libc::{settimeofday, suseconds_t, time_t, timeval};
use std::process::exit;

const NEEDED_CAPS: [Capability; 1] = [Capability::CAP_SYS_TIME];

use time_shell::lib::{remove_capabilities, Code};

fn main() {
    if let Err(_) = remove_capabilities(&NEEDED_CAPS) {
        #[cfg(debug_assertions)]
        println!("Capabilities drop failled");
    }
    let args = parse_args();

    let tv = timeval {
        tv_sec: args.time as time_t,
        tv_usec: 0 as suseconds_t,
    };
    
    unsafe {
        if settimeofday(&tv, std::ptr::null_mut()) != 0 {
            exit(Code::FAILED_SET_TIME);
        }
    }
}

struct Args {
    time: i64,
}

fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        exit(Code::INVALID_ARGUMENT);
    }

    if let Ok(timestamp) = args[1].parse::<i64>() {
        Args { time: timestamp }
    } else {
        exit(Code::INVALID_ARGUMENT);
    }
}
