use std::io;
use std::io::Write;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut prefix: Option<&str> = None;
    for line in input.lines() {
        prefix = Some(match prefix {
            None => line,
            Some(old_prefix) => {
                let common_len = old_prefix.bytes().zip(line.bytes()).take_while(|(x, y)| x == y).count();
                &old_prefix[..common_len]
            }
        })
    }

    let prefix = match prefix {
        Some(prefix) => prefix,
        None => {
            // Empty input
            return Ok(());
        }
    };

    let prefix = {
        let mut prefix = String::from(prefix);
        if prefix.ends_with('.') {
            prefix.pop();
        }
        prefix
    };

    let writer = io::stdout();
    let writer = writer.lock();
    let mut writer = io::BufWriter::new(writer);

    writer.write_all(prefix.as_bytes())?;
    writer.write_all(b"\n")?;

    for line in input.lines() {
        writer.write_all(b"    ")?;
        writer.write_all(line[prefix.len()..].as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
