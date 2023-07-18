use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
// 89 50 4E 47 0D 0A 1A 0A
const IHDR_HEX: [u8; 4] = [73, 72, 68, 82];
fn main() -> io::Result<()> {
    let f = File::open("pngsuite/basn0g16.png")?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buffer)?;
    if buffer[0..=7] != PNG_SIGNATURE {
        print!("Invalid file please choose a PNG")
    } else {
        buffer = buffer[8..].to_vec();

        let length_ihdr: u32 = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
        buffer.drain(0..4);
        if buffer[0..4] != IHDR_HEX {
            print!("INVALID HEADER ORDER OR HEX")
        } else {
            buffer.drain(0..4);
            let image_height: u32 = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
            let image_width: u32 = u32::from_be_bytes(buffer[4..8].try_into().unwrap());
            buffer.drain(0..8);
            println!("{}", image_height);
            println!("{}", image_width);
        }
    }
    println!("{:?}", buffer);

    Ok(())
}
