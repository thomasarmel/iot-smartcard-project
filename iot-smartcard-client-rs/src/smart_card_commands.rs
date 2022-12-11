use crate::smart_card::SmartCard;

#[allow(dead_code)]
pub struct SmartCardCommands {
    smart_card: SmartCard,
}

#[allow(dead_code)]
impl SmartCardCommands {
    pub fn new() -> Result<SmartCardCommands, anyhow::Error> {
        let smart_card = SmartCard::new().unwrap();

        Ok(Self {
            smart_card
        })
    }

    /// Select applet
    pub fn select_applet(&self) {
        // Select applet A0 00 00 00 62 03 01 0C 06 01 02
        let select_applet_apdu_command = apdu::command::select_file(0x04, 0x00, vec![0xA0, 0x00, 0x00, 0x00, 0x62, 0x03, 0x01, 0x0C, 0x06, 0x01, 0x02]);
        let select_applet_apdu_response = self.smart_card.send_apdu_command(select_applet_apdu_command.into()).unwrap();
        assert_eq!((0x90, 0x00), select_applet_apdu_response.trailer); // OK
    }

    /// Fake hello command, returns expected response size
    pub fn send_hello_world(&self) -> u8 {
        let hello_apdu_command_check_size = apdu::Command::new(0x00, 0x10, 0x00, 0x00);
        let hello_apdu_response_check_size = self.smart_card.send_apdu_command(hello_apdu_command_check_size).unwrap();
        assert_eq!(0x6C, hello_apdu_response_check_size.trailer.0); // Wrong length
        hello_apdu_response_check_size.trailer.1
    }

    /// Actual hello command, response size has to be provided (see send_hello_world)
    /// Returns the response
    pub fn send_hello_world_with_correct_size(&self, hello_apdu_response_check_size_trailer_1: u8) {
        let hello_apdu_command = apdu::Command::new_with_le(0x00, 0x10, 0x00, 0x00, hello_apdu_response_check_size_trailer_1 as u16);
        let hello_apdu_response = self.smart_card.send_apdu_command(hello_apdu_command).unwrap();
        assert_eq!((0x90, 0x00), hello_apdu_response.trailer); // OK
        assert_eq!(b"Hello robert", hello_apdu_response.payload.as_slice()); // OK
    }

    /// Authentication command
    /// Returns expected response size
    /// 00 20 00 00 04 0n 0n 0n 0n
    pub fn authenticate(&self, s: &str) -> u8 {
        // let v = string_to_vec_hex(s);
        let mut v = s.to_string().into_bytes();
        v.iter_mut().for_each(|x| *x = *x - 48);

        let auth_apdu_command = apdu::Command::new_with_payload(0x00, 0x20, 0x00, 0x00, v); // vec![0x00, 0x00, 0x00, 0x00]);
        let auth_apdu_response = self.smart_card.send_apdu_command(auth_apdu_command).unwrap();
        assert_eq!(0x61, auth_apdu_response.trailer.0); // OK
        // println!("{:?}", auth_apdu_response);
        auth_apdu_response.trailer.1
    }

    /// Get auth status. Expected response size has to be provided (see authenticate)
    /// A0 C0 00 00 0n
    pub fn get_authentication_status(&self, auth_apdu_response_trailer_1: u8) {
        let get_auth_status_apdu_command = apdu::Command::new_with_le(0xA0, 0xC0, 0x00, 0x00, auth_apdu_response_trailer_1 as u16);
        let get_auth_status_apdu_response = self.smart_card.send_apdu_command(get_auth_status_apdu_command).unwrap();
        assert_eq!((0x90, 0x00), get_auth_status_apdu_response.trailer); // OK
        assert_eq!(vec![0x4F, 0x4B], get_auth_status_apdu_response.payload); // OK
    }

    /// Change pin -> nnnn
    /// Returns expected response size
    /// Note: also lock the card
    /// 00 22 00 00 04 01 02 03 04
    pub fn change_pin(&self, s: &str) -> u8 {
        // let v = string_to_vec_hex(s);
        let mut v = s.to_string().into_bytes();
        v.iter_mut().for_each(|x| *x = *x - 48);

        let change_pin_apdu_command = apdu::Command::new_with_payload(0x00, 0x22, 0x00, 0x00, v); // vec![0x01, 0x02, 0x03, 0x04]);
        let change_pin_apdu_response = self.smart_card.send_apdu_command(change_pin_apdu_command).unwrap();
        assert_eq!(0x6C, change_pin_apdu_response.trailer.0); // OK
        change_pin_apdu_response.trailer.1
    }

    /// Get change pin status
    /// Expected response size has to be provided (see change_pin)
    /// A0 C0 00 00 02
    pub fn get_change_pin_status(&self, change_pin_apdu_response_trailer_1: u8) {
        let get_change_pin_status_apdu_command = apdu::Command::new_with_le(0xA0, 0xC0, 0x00, 0x00, change_pin_apdu_response_trailer_1 as u16);
        let get_change_pin_status_apdu_response = self.smart_card.send_apdu_command(get_change_pin_status_apdu_command).unwrap();
        assert_eq!((0x6E, 0x00), get_change_pin_status_apdu_response.trailer); // OK
    }

    /// Fake get public key
    /// Returns expected response size
    /// 00 30 00 00
    pub fn get_public_key(&self) -> u8 {
        let get_public_key_apdu_command = apdu::Command::new(0x00, 0x30, 0x00, 0x00);
        let get_public_key_apdu_response_check_size = self.smart_card.send_apdu_command(get_public_key_apdu_command).unwrap();
        assert_eq!(0x6C, get_public_key_apdu_response_check_size.trailer.0); // Wrong length
        get_public_key_apdu_response_check_size.trailer.1
    }

    /// Actual get public key
    /// Expected response size has to be provided (see get_public_key)
    /// 00 30 00 00 nn
    pub fn get_actual_public_key(&self, get_public_key_apdu_response_check_size: u8) -> Vec<u8> {
        // print!("{:?}", get_public_key_apdu_response_check_size.trailer.1);
        let get_public_key_apdu_command = apdu::Command::new_with_le(0x00, 0x30, 0x00, 0x00, get_public_key_apdu_response_check_size as u16);
        let get_public_key_apdu_response = self.smart_card.send_apdu_command(get_public_key_apdu_command).unwrap();
        assert_eq!((0x90, 0x00), get_public_key_apdu_response.trailer); // OK
        get_public_key_apdu_response.payload
    }

    /// Ask for signing &str
    /// Returns expected response size
    /// 00 31 00 00 nn nn ....
    pub fn ask_for_signature(&self, s: &str) -> u8 {
        let ask_for_signing_apdu_command = apdu::Command::new_with_payload(0x00, 0x31, 0x00, 0x00, s.to_string().into_bytes());// vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
        let ask_for_signing_apdu_response = self.smart_card.send_apdu_command(ask_for_signing_apdu_command).unwrap();
        assert_eq!(0x61, ask_for_signing_apdu_response.trailer.0); // OK
        ask_for_signing_apdu_response.trailer.1
    }

    /// Fetch signature
    /// Expected response size has to be provided (see ask_for_signature)
    /// A0 C0 00 00 nn
    pub fn fetch_signature(&self, ask_for_signing_apdu_response_trailer_1: u8) -> Vec<u8> {
        let fetch_signature_apdu_command = apdu::Command::new_with_le(0xA0, 0xC0, 0x00, 0x00, ask_for_signing_apdu_response_trailer_1 as u16);
        let fetch_signature_apdu_response = self.smart_card.send_apdu_command(fetch_signature_apdu_command).unwrap();
        assert_eq!((0x90, 0x00), fetch_signature_apdu_response.trailer); // OK
        fetch_signature_apdu_response.payload
    }

    /// Logout
    /// 00 21 00 00
    pub fn logout(&self) -> u8 {
        let logout_apdu_command = apdu::Command::new(0x00, 0x21, 0x00, 0x00);
        let logout_apdu_response = self.smart_card.send_apdu_command(logout_apdu_command).unwrap();
        assert_eq!(0x6C, logout_apdu_response.trailer.0); // Wrong length
        logout_apdu_response.trailer.1
    }

    /// Get logout status
    /// 00 21 00 00 02
    pub fn get_logout_status(&self, logout_apdu_response_trailer_1: u8) {
        let get_logout_status_apdu_command = apdu::Command::new_with_le(0x00, 0x21, 0x00, 0x00, logout_apdu_response_trailer_1 as u16);
        let get_logout_status_apdu_response = self.smart_card.send_apdu_command(get_logout_status_apdu_command).unwrap();
        assert_eq!((0x90, 0x00), get_logout_status_apdu_response.trailer); // OK
        assert_eq!(vec![0x4F, 0x4B], get_logout_status_apdu_response.payload); // OK
    }
}
