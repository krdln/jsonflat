mod utils;

use serde_json::Value;
use std::io::{self, Write};

fn flatten(prefix: &mut String, json: &Value, w: &mut impl io::Write) -> io::Result<()> {
    use utils::StringStack;
    match json {
        Value::Object(map) => {
            if map.is_empty() {
                writeln!(w, "{}: {{}}", prefix)?;
            }
            let prefix = StringStack::new(prefix);
            if let Some(name) = map.get("name").and_then(Value::as_str) {
                prefix.buffer.push_str("(name=");
                prefix.buffer.push_str(name);
                prefix.buffer.push_str(")");
            }
            for (k, v) in map {
                let prefix = StringStack::new(prefix.buffer);
                prefix.buffer.push_str(".");
                prefix.buffer.push_str(k);
                flatten(prefix.buffer, v, w)?;
            }
        }
        Value::Array(items) => {
            if items.is_empty() {
                writeln!(w, "{}: []", prefix)?;
            }
            for (i, item) in items.iter().enumerate() {
                let prefix = StringStack::new(prefix);
                use std::fmt::Write;
                write!(prefix.buffer, "[{}]", i).unwrap();
                flatten(prefix.buffer, item, w)?;
            }
        }
        scalar => {
            writeln!(w, "{}: {}", prefix, scalar)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = io::BufReader::new(stdin);
    let mut stdin = utils::RewindBuffer::new(stdin);

    let writer = io::stdout();
    let writer = writer.lock();
    let mut writer = io::BufWriter::new(writer);

    let mut line = String::new();
    while { line.clear(); stdin.read_line(&mut line)? != 0 } {
        stdin.forget_past();
        if let Some(brace_pos) = line.find('{') {
            stdin.unread(line[brace_pos..].as_bytes());
            match serde_json::Deserializer::from_reader(&mut stdin).into_iter().next() {
                Some(Ok(json)) => {
                    stdin.forget_past();
                    flatten(&mut line[..brace_pos].into(), &json, &mut writer)?;

                    // Read the remaining ("after-json") part of line
                    let mut line_ending = String::new();
                    stdin.read_line(&mut line_ending)?;
                    // Retype the line (hiding json) if there's anything interesting at the end
                    if line_ending.trim() != "" {
                        writer.write_all(line[..brace_pos].as_bytes())?;
                        writer.write_all("{…}".as_bytes())?;
                        writer.write_all(line_ending.as_bytes())?;
                    }
                    writer.flush()?;
                }
                Some(Err(e)) if e.is_io() => return Err(e.into()),
                Some(Err(_)) | None => {
                    stdin.rewind();
                    writer.write_all(line.as_bytes())?;
                    writer.flush()?;
                    // Consume the remaning part of line, which is now part of rewind buffer
                    line.clear();
                    stdin.read_line(&mut line)?;
                }
            }
        } else {
            writer.write_all(line.as_bytes())?;
            writer.flush()?;
        }
    }

    Ok(())
}
