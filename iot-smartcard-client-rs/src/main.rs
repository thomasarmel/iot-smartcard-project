use std::io::{stdin, stdout, Write};
use ibig::{ibig, modular::ModuloRing, ubig, UBig};

use iot_smartcard_client_rs::smart_card_commands::SmartCardCommands;

fn vec8_to_u32(input: &[u8]) -> u32 {
    input.iter().fold(0, |accum, v| (accum << 8) + *v as u32)
}

fn shell() {
    let smart_card_commands = SmartCardCommands::new().unwrap();
    let mut _exponent: u32 = 65537;
    let mut _modulus = ubig!(0b010001);

    smart_card_commands.select_applet();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut command = input.split_whitespace().next().unwrap();

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
                let exponent = vec8_to_u32(&public_key[2..2 + exponent_size as usize]);
                let modulus_size = vec8_to_u32(&public_key[2 + exponent_size as usize..2 + exponent_size as usize + 2]);
                let modulus = vec8_to_u32(&public_key[2 + exponent_size as usize + 2..2 + exponent_size as usize + 2 + modulus_size as usize]);
                println!("exponent size: {}", exponent_size);
                println!("exponent: {}", exponent);
                println!("modulus size: {}", modulus_size);
                println!("modulus: {}", modulus);
                _exponent = exponent.clone();
                _modulus = UBig::from(modulus);
            },
            "gsign" => {
                println!("Please enter the message you want to sign:");
                let mut message = String::new();
                stdin().read_line(&mut message).unwrap();
                let message = message.trim();
                let response = smart_card_commands.ask_for_signature(message);
                let signature = smart_card_commands.fetch_signature(response);
                let signature_to_int = vec8_to_u32(&signature);
                let message_to_int = vec8_to_u32(message.as_bytes());
                let modulus_temp = _modulus.clone();
                let modulus_temp_2 = _modulus.clone();
                let ring = ModuloRing::new(&UBig::from(modulus_temp));
                // println!("verify signature: {}", ModuloRing::new(&ubig!(modulus_temp_2)).from(signature_to_int).pow_signed(&ibig!(_exponent)) == ring.from(message_to_int));
                println!("signature: {:?}", signature);
                println!("decrypted signature: {}", ModuloRing::new(&modulus_temp_2).from(signature_to_int).pow_signed(&ibig!(65537)));
                println!("message: {}", ring.from(message_to_int));
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
