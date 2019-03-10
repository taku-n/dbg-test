use std::error::Error;
use std::ffi::*;
use std::io::prelude::*;
use std::net::TcpStream;

const ADDR_PORT: &str = "127.0.0.1:9841";

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(ADDR_PORT)?;
    let c_string = CString::new("hello")?;

    //stream.write(&[0x41, 0x42, 0x43, 0x00])?;
    stream.write(c_string.as_bytes_with_nul())?;

    Ok(())
}
