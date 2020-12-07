use std::io::Read;

use anyhow::Result;

pub fn input<T, F>(mut reader: impl Read, transform: F) -> Result<T>
where
    F: Fn(String) -> Result<T>,
{
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    transform(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_input() -> Result<()> {
        let r = input(Cursor::new("Hello, World!"), |s| Ok(s))?;

        assert_eq!(r, "Hello, World!");

        Ok(())
    }

    #[test]
    fn test_input_2() -> Result<()> {
        let r = input(Cursor::new("Hello, World!"), |s| {
            Ok(s.split(" ").map(|e| e.to_string()).collect::<Vec<String>>())
        })?;

        assert_eq!(r, &["Hello,", "World!"]);

        Ok(())
    }
}
