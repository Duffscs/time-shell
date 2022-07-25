# Time-shell

Time-shell is a secure code project realized in rust during my month-long course in Prague at the Czech Technical University.

# Instruction

Develop a Network Clock (NC) application with the following characteristics:

* NC is run as a standard application. It displays the current date and time to the interactive (logged-on) user; the user may specify the exact format of the displayed value by setting a format string interactively. It’s acceptable to only update the display on request.
* Additionally, NC listens for communications on a TCP port number defined in a configuration file (or a registry key). A remote user can connect to this port and request the current date and time in a specified format.
* The interactive user (but not a remote one) may also set the date/time.
* Note that since the application is accessible from the internet, there are many potential attackers waiting to exploit any bug. For this reason, the application should be written with security in mind; particularly, it will use as low privileges as possible.

# My work

I wrote the programs in rust because this langage is memory safe and allow to handle the input easily.

## Communication

For the communication I chose the HTTP protocol, because the protocol handle buffer overflow and size of message automatically, so it's safer than using tcp and our own communication way. 

## Data execution Prevention

Refering [rust documentation], the DEP is automaticly enabled by the rust compiler:

```
The Rust compiler supports non-executable memory regions, and enables it by default since its initial release, version 0.1 (2012-01-20)[21], [22], but has regressed since then[23]–[25], and enforced by default since version 1.8.0 (2016-04-14)[25].
```

[rust documentation]: https://doc.rust-lang.org/rustc/exploit-mitigations.html#non-executable-memory-regions

# Usage 

How to run time-shell:
```
cargo build --release
cd target/release/
./time-shell
```

```
help
    time [format]   -- print the current time, use strftime format
    settime [time]  -- set the current time, use DD/MM/YYYY HH:mm format
    help            -- print this message
    exit            -- exit the program
```

The server can be tested with the following command:

```
curl -X POST "http://127.0.0.1:8080" -d '%d/%m/%Y %H:%M' 
```

To use settime command, the sys_time capabilities should be granted to settime and it should be placed in /usr/bin.
```
sudo setcap cap_sys_time+ep ./settime
mv settime /usr/bin/
```

Server port can be configured using --port or editing /etc/time-shell/port

- /etc/time-shell is a directory owned by root with 744 permissions.
- /etc/time-shell/port is a file owned by root with 644 permissions.



if settime doesn't work, try to disable set-ntp :
```
sudo timedatectl set-ntp 0
```

## Contributor :
- https://github.com/Duffscs