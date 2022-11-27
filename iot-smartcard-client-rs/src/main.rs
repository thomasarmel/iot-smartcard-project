use std::io::{stdin, stdout, Write};
use ibig::{modular::ModuloRing, UBig};
use iot_smartcard_client_rs::smart_card_commands::SmartCardCommands;

fn vec8_to_u32(input: &[u8]) -> u32 {
    input.iter().fold(0, |accum, v| (accum << 8) + *v as u32)
}

fn shell() {
    let smart_card_commands = SmartCardCommands::new().unwrap();
    let mut _exponent: UBig = UBig::from(65537 as u32);
    let mut _modulus = UBig::from(0 as u32);

    smart_card_commands.select_applet();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.split_whitespace().next().unwrap();

        match command {
            "send_hello_world" => {
                let response = smart_card_commands.send_hello_world();
                smart_card_commands.send_hello_world_with_correct_size(response);
            },
            "auth" => {
                println!("Please enter your PIN:");
                let mut pin = String::new();
                stdin().read_line(&mut pin).unwrap();
                let pin = pin.trim();
                let response = smart_card_commands.authenticate(pin);
                smart_card_commands.get_authentication_status(response);
            },
            "chpin" => {
                /*println!("Please enter your old PIN:");
                let mut old_pin = String::new();
                stdin().read_line(&mut old_pin).unwrap();
                let old_pin = old_pin.trim();*/
                println!("Please enter your new PIN:");
                let mut new_pin = String::new();
                stdin().read_line(&mut new_pin).unwrap();
                let new_pin = new_pin.trim();
                let response = smart_card_commands.change_pin(/*old_pin, */new_pin);
                smart_card_commands.get_change_pin_status(response);
            },
            "gpubk" => {
                let response = smart_card_commands.get_public_key();
                let public_key = smart_card_commands.get_actual_public_key(response);
                println!("public key: {:x?}", public_key);
                let exponent_size = vec8_to_u32(&public_key[0..2]);
                let modulus_size = vec8_to_u32(&public_key[2 + exponent_size as usize..2 + exponent_size as usize + 2]);
                _exponent = UBig::from_be_bytes(&public_key[2..2 + exponent_size as usize]);
                _modulus = UBig::from_be_bytes(&public_key[2 + exponent_size as usize + 2..2 + exponent_size as usize + 2 + modulus_size as usize]);
                println!("exponent size: {}", exponent_size);
                println!("exponent: {}", _exponent);
                println!("modulus size: {}", modulus_size);
                println!("modulus: {}", _modulus);
            },
            "gsign" => {
                println!("Please enter the message you want to sign:");
                let mut message_to_sign = String::new();
                stdin().read_line(&mut message_to_sign).unwrap();
                let message_to_sign = message_to_sign.trim();
                let message_size = message_to_sign.len();
                let signature_size = smart_card_commands.ask_for_signature(message_to_sign);
                let signature = UBig::from_be_bytes(&*smart_card_commands.fetch_signature(signature_size));
                let ring = ModuloRing::new(&_modulus);
                println!("received signature: {:?}", signature);
                let decrypted_signature = ring.from(&signature).pow(&_exponent).residue().to_be_bytes();
                // Remove padding
                let decrypted_signature = std::str::from_utf8(&decrypted_signature[decrypted_signature.len() - message_size..]).unwrap();
                println!("decrypted signature: {}", decrypted_signature);
                if decrypted_signature == message_to_sign {
                    println!("Signature is valid!");
                } else {
                    println!("Signature is invalid!");
                }
            },
            "logout" => {
                let response = smart_card_commands.logout();
                smart_card_commands.get_logout_status(response);
            },
            "help" => {
                println!("Available commands:");
                println!("send_hello_world - Sends a hello world message to the smart card.");
                println!("auth - Authenticates the user with the smart card.");
                println!("chpin - Changes the PIN of the user.");
                println!("gpubk - Gets the public key of the user.");
                println!("gsign - Gets the signature of a message.");
                println!("logout - Logs out the user.");
                println!("help - Prints this help message.");
                println!("exit - Exits the shell.");
            },
            "exit" => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Unknown command");
            }
        }
    }
}

fn main() {
    shell();
}
