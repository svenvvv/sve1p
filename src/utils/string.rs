use std::io::{Error, Write};

pub fn write_escaped_string<T: Write>(writer: &mut T, string: &String) -> Result<(), Error> {
    for ch in string.bytes() {
        if ch == '"' as u8 {
            writer.write("\\".as_bytes())?;
        };
        writer.write_all(&[ch])?;
    }
    Ok(())
}

pub fn unescape_string(string: &str) -> String {
    // Pretty lazy, TODO if this breaks :)
    string.chars().filter(|c| *c != '\\').collect()
}
