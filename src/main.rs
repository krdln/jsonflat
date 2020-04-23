use serde_json::Value;
use std::io;

struct StringStack<'a> {
    prev_len: usize,
    buffer: &'a mut String,
}

impl<'a> StringStack<'a> {
    fn new(buffer: &'a mut String) -> Self {
        StringStack {
            prev_len: buffer.len(),
            buffer,
        }
    }
}

impl Drop for StringStack<'_> {
    fn drop(&mut self) {
        self.buffer.truncate(self.prev_len);
    }
}

fn flatten(prefix: &mut String, json: &Value, w: &mut impl io::Write) -> io::Result<()> {
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
    let json_reader = serde_json::Deserializer::from_reader(stdin);

    let writer = io::stdout();
    let writer = writer.lock();
    let mut writer = io::BufWriter::new(writer);

    for json in json_reader.into_iter() {
        let json = json?;
        flatten(&mut String::new(), &json, &mut writer)?;
    }

    Ok(())
}
