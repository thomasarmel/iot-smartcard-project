use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::str::from_utf8;
use iot_smartcard_client_rs::{smart_card_commands::SmartCardCommands, rsa_public_key::RSAPublicKey};

fn vec8_to_u32(input: &[u8]) -> u32 {
    input.iter().fold(0, |accum, v| (accum << 8) + *v as u32)
}

/// Wait for user command
fn shell() {
    let smart_card_commands = SmartCardCommands::new().unwrap();

    let mut rsa_public_key : Option<RSAPublicKey> = None;

    smart_card_commands.select_applet();

    println!("Hello world!");
    println!("To see the documentation of the different commands, type 'help' :)");

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
                println!("Please enter your new PIN:");
                let mut new_pin = String::new();
                stdin().read_line(&mut new_pin).unwrap();
                let new_pin = new_pin.trim();
                let response = smart_card_commands.change_pin(new_pin);
                smart_card_commands.get_change_pin_status(response);
            },
            "gpubk" => {
                let response = smart_card_commands.get_public_key();
                let public_key = smart_card_commands.get_actual_public_key(response);
                println!("Received public key: {:02X?}", public_key);
                let exponent_size = vec8_to_u32(&public_key[0..2]);
                let modulus_size = vec8_to_u32(&public_key[2 + exponent_size as usize..2 + exponent_size as usize + 2]);
                rsa_public_key = Some(RSAPublicKey::new(
                    &public_key[2..2 + exponent_size as usize],
                    &public_key[2 + exponent_size as usize + 2..2 + exponent_size as usize + 2 + modulus_size as usize]
                ));
                println!("{}", rsa_public_key.as_ref().unwrap());
            },
            "gsign" => {
                if rsa_public_key.is_none() {
                    println!("Please get the public key first!");
                    continue;
                }
                println!("Please enter the message you want to sign:");
                let mut message_to_sign = String::new();
                stdin().read_line(&mut message_to_sign).unwrap();
                let message_to_sign = message_to_sign.trim();
                let signature_size = smart_card_commands.ask_for_signature(message_to_sign);
                let signature = smart_card_commands.fetch_signature(signature_size);
                println!("Received signature: {:02X?}", signature);
                println!("Checking signature...");
                if rsa_public_key.as_ref().unwrap().check_signature(message_to_sign.as_bytes(), &signature) {
                    println!("The signature is valid!");
                } else {
                    println!("The signature is invalid!");
                }
                println!("Signature: {:02X?}", signature);
                println!("{}", std::env::current_dir().unwrap().display())
            }
            "gsign_file" => {
                println!("Please enter the path to the file you want to sign (example: ./src/messages/hello.txt):");
                let mut path = String::new();
                stdin().read_line(&mut path).unwrap();
                let path = path.trim();
                let path = Path::new(&path);
                println!("{}", path.display());
                let message_to_sign = fs::read_to_string(path).unwrap();
                let message_to_sign = message_to_sign.trim();
                let signature_size = smart_card_commands.ask_for_signature(message_to_sign);
                let signature = smart_card_commands.fetch_signature(signature_size);
                println!("Received signature: {:02X?}", signature);
                println!("Checking signature...");

                let b = match rsa_public_key.as_ref().unwrap().check_signature(message_to_sign.as_bytes(), &signature) {
                    true => {
                        println!("The signature is valid!");
                        true
                    },
                    false => {
                        println!("The signature is invalid!");
                        false
                    },
                };

                let signature_decimal = signature.iter().map(|&b| format!("{:?}", b)).collect::<Vec<String>>().join("");
                let signature_hexadecimal = hex::encode(&signature);
                let signature_decrypted = rsa_public_key.as_ref().unwrap().decrypt_signature(message_to_sign.as_bytes(), &signature);
                let signature_decrypted_utf8 = from_utf8(&*signature_decrypted).iter().map(|&b| format!("{:?}", b)).collect::<Vec<String>>().join("");
                // remove quotation marks
                let signature_decrypted_utf8 = signature_decrypted_utf8.replace("\"", "");

                if b {
                    println!("Writing signature, message and result to {}_signature_valid.txt file...", path.file_stem().unwrap().to_str().unwrap());
                        fs::write(format!("./src/results/{}{}", path.file_stem().unwrap().to_str().unwrap(), "_signature_valid.txt"), format!("decimal signature: {}\nhexadecimal signature: {}\ndecrypted signature: {}\noriginal message: {}\nvalid signature: {}", signature_decimal, signature_hexadecimal, signature_decrypted_utf8, message_to_sign, b)).expect(format!("Unable to write {}_signature.txt file", path.file_stem().unwrap().to_str().unwrap()).as_str());
                } else {
                    println!("Writing signature, message and result to {}_signature_invalid.txt file...", path.file_stem().unwrap().to_str().unwrap());
                        fs::write(format!("./src/results/{}{}", path.file_stem().unwrap().to_str().unwrap(), "_signature_invalid.txt"), format!("decimal signature: {}\nhexadecimal signature: {}\ndecrypted signature: {}\noriginal message: {}\nvalid signature: {}", signature_decimal, signature_hexadecimal, signature_decrypted_utf8, message_to_sign, b)).expect(format!("Unable to write {}_signature.txt file", path.file_stem().unwrap().to_str().unwrap()).as_str());
                }

                println!("Done!");
                println!("When tou want to see the file(s), type 'exit' :)");
            }
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
                println!("gsign_file - Gets the signature of a message in a file and store the results in another file.");
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
