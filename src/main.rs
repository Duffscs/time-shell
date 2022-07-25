use std::collections::HashSet;
use std::io::{self, BufRead};
use std::net::TcpListener;
use std::num::ParseIntError;
use std::process::{exit, Command};

use caps::Capability;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use nix::libc::{gettimeofday, timeval};
use time_shell::lib::{remove_capabilities, Code};

const NEEDED_CAPS: [Capability; 0] = [];
const PORT_FILE: &str = "/etc/time-shell/port";

fn main() {
    #[cfg(debug_assertions)]
    println!("Debugging enabled, consider build using `cargo build --release`");

    if let Err(_) = remove_capabilities(&NEEDED_CAPS) {
        #[cfg(debug_assertions)]
        println!("Capabilities drop failled");
    }

    std::thread::spawn(open_server);
    interactive_cli();
}

fn interactive_cli() {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("line"))
        .for_each(handle_commands);
}

const SET_TIME_PATH: &str = "/usr/bin/set_time";

fn handle_commands(string: String) {
    if string == "" {
        return;
    }
    let input = string.split_whitespace().collect::<Vec<&str>>();
    let command = input[0];
    let arguments = input.into_iter().skip(1).collect::<Vec<&str>>().join(" ");
    match command {
        "time" => println!("{}", time(arguments)),
        "settime" => set_time(arguments),
        "help" => help(),
        "exit" => exit(Code::SUCESS),
        _ => println!("\tcommand not found, try help"),
    }
}

fn help() {
    println!("\ttime [format]   -- print the current time, use strftime format");
    println!("\tsettime [time]  -- set the current time, use DD/MM/YYYY HH:mm format");
    println!("\thelp            -- print this message");
    println!("\texit            -- exit the program");
}

fn time(format: String) -> String {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    unsafe {
        if gettimeofday(&mut tv, std::ptr::null_mut()) != 0 {
            #[cfg(debug_assertions)]
            println!("Exit code : {}", Code::FAILED_GET_TIME);
        }
        let now = chrono::Local.timestamp(tv.tv_sec as i64, (tv.tv_usec * 1000) as u32);
        return now.format(&format).to_string();
    }
}

fn set_time(input: String) {
    let res_dt = parse_string_to_datetime_local(input.clone(), "%d/%m/%Y %H:%M");
    match res_dt {
        Ok(datetime) => exec_binary(SET_TIME_PATH, &datetime.timestamp().to_string()),
        Err(_) => println!("{} : invalid format", input),
    }
}

fn exec_binary(path: &str, arg: &str) {
    // in release mode, code is always 0
    let res = Command::new(path)
        .arg(arg)
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("wait for the end");

    #[cfg(debug_assertions)]
    println!("Exit code : {}", res.code().unwrap());
}

fn parse_string_to_datetime_local(input: String, format: &str) -> Result<DateTime<Local>, i32> {
    let naive_parsed_date = NaiveDateTime::parse_from_str(&input, format);
    if naive_parsed_date.is_err() {
        return Err(Code::PARSE_ERROR);
    }
    let date = Local.from_local_datetime(&naive_parsed_date.unwrap());
    if date.single().is_none() {
        return Err(Code::PARSE_ERROR);
    }
    Ok(date.unwrap())
}

use actix_web::{post, App, HttpServer, Responder};
use std::fs;

#[post("/")]
async fn time_route(format: String) -> impl Responder {
    time(format)
}

#[actix_web::main]
async fn open_server() {
    let addr = "127.0.0.1";
    println!(
        "Server port can be configured using --port or editing {}",
        PORT_FILE
    );
    let default_port;

    if let Some(available_port) = get_an_available_port() {
        default_port = available_port;
    } else {
        println!("No available port found, not starting server");
        return;
    }

    let mut port = get_port(default_port);

    loop {
        if let Ok(server) = HttpServer::new(|| App::new().service(time_route)).bind((addr, port)) {
            println!("Server is listening on http://{}:{}", addr, port);
            server.run().await.unwrap();
            break;
        } else if port == default_port {
            println!("Could not bind to port {}", port);
            break;
        } else {
            println!(
                "Could not bind to port {}, falling back on an available {}",
                port, default_port
            );
            port = default_port;
        }
    }
}

fn get_port(available_port: u16) -> u16 {
    let config_file_port = get_port_from_config_file().unwrap_or(available_port);
    return get_port_from_args().unwrap_or(config_file_port);
}

fn get_port_from_config_file() -> Result<u16, ParseIntError> {
    let read = fs::read_to_string(PORT_FILE).unwrap_or_default();
    return read.trim().parse::<u16>();
}

fn get_port_from_args() -> Option<u16> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--port" {
        return match args[2].parse::<u16>() {
            Ok(port) => Some(port),
            Err(_) => None,
        };
    }
    return None;
}

fn get_an_available_port() -> Option<u16> {
    let range = (1025..65535).collect::<HashSet<u16>>(); // "randomizes port range
    for port in range {
        if let Ok(_l) = TcpListener::bind(("127.0.0.1", port)) {
            return Some(port);
        }
    }
    None
}

// curl -X POST "http://127.0.0.1:8080" -d '%d/%m/%Y %H:%M'
