use honywell_ean_scanner::ean_code;
use honywell_ean_scanner::ean_code::EANCode;
use serialport::Error;
use std::{io::Read, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Error> {
    //You have to choose right rs232 port
    let mut port = serialport::new("COM4", 115_200)
        .timeout(Duration::from_millis(10))
        .open_native()?;

    let mut buf: Vec<u8> = vec![0; 32];

    loop {
        let prefix;
        let code;
        match port.read(&mut buf) {
            Ok(len) => prefix = ean_code::parse_sequence(len, &buf),
            Err(_) => continue,
        };
        match port.read(&mut buf) {
            Ok(len) => code = ean_code::parse_sequence(len, &buf),
            Err(_) => continue,
        };

        match EANCode::new(prefix, code) {
            Some(ean) => println!("\n{:#?}", ean),
            None => {}
        }
        buf = vec![0; 32];
    }
}
