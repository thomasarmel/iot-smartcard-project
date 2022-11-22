fn main() {
    let smart_card = iot_smartcard_client_rs::smart_card::SmartCard::new().unwrap();

    // Select applet A0 00 00 00 62 03 01 0C 06 01 02
    let select_applet_apdu_command = apdu::command::select_file(0x04, 0x00, vec![0xA0, 0x00, 0x00, 0x00, 0x62, 0x03, 0x01, 0x0C, 0x06, 0x01, 0x02]);
    let select_applet_apdu_response = smart_card.send_apdu_command(select_applet_apdu_command.into()).unwrap();
    assert_eq!((0x90, 0x00), select_applet_apdu_response.trailer); // OK

    // Send hello command
    let hello_apdu_command_check_size = apdu::Command::new(0x00, 0x10, 0x00, 0x00);
    let hello_apdu_response_check_size = smart_card.send_apdu_command(hello_apdu_command_check_size).unwrap();
    assert_eq!(0x6C, hello_apdu_response_check_size.trailer.0); // Wrong length

    let correct_hello_apdu_response_expected_length = hello_apdu_response_check_size.trailer.1;
    let hello_apdu_command = apdu::Command::new_with_le(0x00, 0x10, 0x00, 0x00, correct_hello_apdu_response_expected_length as u16);
    let hello_apdu_response = smart_card.send_apdu_command(hello_apdu_command).unwrap();
    assert_eq!((0x90, 0x00), hello_apdu_response.trailer); // OK
    assert_eq!(b"Hello robert", hello_apdu_response.payload.as_slice());
}
