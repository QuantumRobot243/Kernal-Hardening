mod hardening;
mod secrets;
mod shutdown;

use hardening::{memory, dump, signals, anti_debug, input};
use secrets::secret::Secret;
use std::panic;
use obfstr::obfstr;

fn main() {
    panic::set_hook(Box::new(|info| {
        eprintln!("\n{}", obfstr!("[CRITICAL] Panic detected!"));
        shutdown::secure_shutdown();
    }));

    anti_debug::block_debugger();
    memory::lock_memory();
    dump::disable_core_dumps();
    signals::install_signal_handlers();

    let _my_password = Secret::new();

    println!("{}", obfstr!("H-shell active. Enter Your Input..."));

    loop {
        let command = input::secure_prompt(obfstr!("H-Shell $> "));
        if command == obfstr!("status") {
            println!("{}", obfstr!("[*] Memory: LOCKED"));
            println!("{}", obfstr!("[*] Ptrace: BLOCKED"));
            println!("{}", obfstr!("[*] Dumps: DISABLED"));
        }
        else if command == obfstr!("secrets") {
            println!("{}", obfstr!("[*] Secrets are loaded and hot."));
        }
        else if command == obfstr!("exit") || command == obfstr!("quit") {
            println!("{}", obfstr!("Exiting..."));
            shutdown::secure_shutdown();
        }
        else if command.is_empty() {
            continue;
        }
        else {
            println!("{}", obfstr!("Unknown command. This event has been logged."));
        }

        drop(command);
    }
}
