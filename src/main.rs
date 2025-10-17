use windows::{
    Win32::{
        Foundation::ERROR_BUFFER_OVERFLOW,
        NetworkManagement::IpHelper::{
            GetAdaptersAddresses, GAA_FLAG_INCLUDE_PREFIX, IP_ADAPTER_ADDRESSES_LH,
        },
        System::Console::CONSOLE_MODE,
    },
};

use std::{
    io::{self, Write},
};
use hostname;
use arboard::Clipboard;
use colored::*;

// IEEE 802.11 interface type (Wi-Fi)
const IF_TYPE_IEEE80211: u32 = 71;

/// Checks if an adapter is likely virtual based on its description
fn is_virtual_adapter(description: &str) -> bool {
    let desc_lower = description.to_lowercase();
    [
        "virtual",
        "vmware",
        "hyper-v",
        "tap",
        "loopback",
        "vpn",
        "container",
        "bluetooth",
        "kernel debug",
        "zerotier",
    ]
    .iter()
    .any(|keyword| desc_lower.contains(keyword))
}

/// Converts a Windows `PWSTR` (UTF-16 pointer) to a Rust `String`
unsafe fn pwstr_to_string(pwstr: windows::core::PWSTR) -> String {
    if pwstr.is_null() {
        return "(No Description)".to_string();
    }
    let len = (0..).take_while(|&i| *pwstr.0.add(i) != 0).count();
    let slice = std::slice::from_raw_parts(pwstr.0, len);
    String::from_utf16_lossy(slice)
}

#[cfg(windows)]
fn enable_ansi_support() {
    use windows::Win32::System::Console::{
        GetConsoleMode, SetConsoleMode, GetStdHandle,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE,
    };
    use windows::Win32::Foundation::HANDLE;

    unsafe {

        let std_out = GetStdHandle(STD_OUTPUT_HANDLE).unwrap();
        let mut mode: CONSOLE_MODE = CONSOLE_MODE(0);

        // Fix: use is_ok() instead of as_bool()
        if GetConsoleMode(std_out, &mut mode).is_ok() {
            let new_mode = CONSOLE_MODE(mode.0 | ENABLE_VIRTUAL_TERMINAL_PROCESSING.0);
            let _ = unsafe { SetConsoleMode(std_out, new_mode) };
        }
    }
}


fn main() {
    #[cfg(windows)]
    enable_ansi_support();
    // ASCII Art Banner
    println!(
        r#"
 ___ _____ ___   _____           _  
|_ _| ____|_ _| |_   _|__   ___ | |___  
 | ||  _|  | |    | |/ _ \ / _ \| / __|  
 | || |___ | |    | | (_) | (_) | \__ \  
|___|_____|___|   |_|\___/ \___/|_|___/  

getmacaddress v0.1.0
Copyright (C) 2025 PT. Indonesia Epson Industry

A lightweight utility for retrieving hostname and MAC address information.
"#
    );

    // Print Hostname
    match hostname::get() {
        Ok(name) => println!("Hostname   : {}\n", name.to_string_lossy().blue().bold()),
        Err(e) => eprintln!("Failed to get hostname: {}\n", e),
    }

    unsafe {
        let mut buffer_length: u32 = 0;
        let flags = GAA_FLAG_INCLUDE_PREFIX;

        // First call to get required buffer size
        let mut result = GetAdaptersAddresses(
            0,
            flags,
            None,
            None,
            &mut buffer_length,
        );

        let mut adapter_addresses = Vec::new();

        if result == ERROR_BUFFER_OVERFLOW.0 {
            adapter_addresses = vec![0u8; buffer_length as usize];

            result = GetAdaptersAddresses(
                0,
                flags,
                None,
                Some(adapter_addresses.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
                &mut buffer_length,
            );
        }

        if result != 0 {
            eprintln!("GetAdaptersAddresses failed with error code: {}", result);
            return;
        }

        let mut adapter = adapter_addresses.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;

        println!("Wi-Fi Interfaces:\n");

        let mut copied = false;

        while !adapter.is_null() {
            let adapter_ref = &*adapter;

            // Only show Wi-Fi adapters
            if adapter_ref.IfType == IF_TYPE_IEEE80211 {
                let desc = pwstr_to_string(adapter_ref.Description);
                let is_virtual = is_virtual_adapter(&desc);

                // Convert UTF-16 description to Rust String
                // let desc = if !adapter_ref.Description.is_null() {
                //     let wide = adapter_ref.Description;
                //     let len = (0..)
                //         .take_while(|&i| *wide.0.add(i) != 0)
                //         .count();
                //     let slice = std::slice::from_raw_parts(wide.0, len);
                //     String::from_utf16_lossy(slice)
                // } else {
                //     "(No Description)".to_string()
                // };

                // Format MAC address
                let mac = (0..adapter_ref.PhysicalAddressLength as usize)
                    .map(|i| format!("{:02X}", adapter_ref.PhysicalAddress[i]))
                    .collect::<Vec<_>>()
                    .join("-");

                println!("Description: {}", desc);
                if !is_virtual {
                    print!("MAC Address: {}", mac.blue().bold());
                } else {
                    println!("MAC Address: {}\n", mac.dimmed());
                }
                // Copy first physical Wi-Fi MAC to clipboard
                if !is_virtual && !copied {
                    match Clipboard::new() {
                        Ok(mut clipboard) => {
                            if clipboard.set_text(mac.clone()).is_ok() {
                                println!("{} {}", " ✔".green(), "MAC address copied to clipboard. You can paste it anywhere.\n");
                                copied = true;
                            } else {
                                eprintln!("  ✖  Failed to copy MAC address to clipboard.\n");
                            }
                        }
                        Err(e) => eprintln!("✖ Clipboard error: {e}"),
                    }
                }
            }

            adapter = adapter_ref.Next;
        }
        
        if !copied {
            println!("No physical Wi-Fi adapter found to copy.");
        }
    }

    // Wait for user to press Enter before exiting
    print!("Hit Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
