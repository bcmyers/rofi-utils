use std::borrow::Cow;
use std::io::{self, Write};
use std::process::{Command, Stdio};

use failure::Error;

const CONFIG: &[(&str, &str)] = &[("background", "~/bin/background")];

fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    let _ = args.next();

    match args.next() {
        // First call
        None => {
            for (key, _) in CONFIG.iter() {
                println!("{}", key)
            }
            Ok(())
        }
        // Second call
        Some(key) => {
            let value = CONFIG
                .iter()
                .filter(|(k, _)| *k == key)
                .map(|(_, v)| v)
                .nth(0)
                .unwrap();
            let (mut binary, arg): (Cow<str>, _) = match value.find(' ') {
                Some(index) => {
                    let binary = &value[0..index];
                    let arg = &value[index + 1..];
                    (binary.into(), Some(arg))
                }
                None => ((*value).into(), None),
            };
            if binary.starts_with("~/") {
                let home_dir = dirs::home_dir().unwrap();
                let path = home_dir.join(&binary[2..]);
                binary = path.to_str().unwrap().to_string().into();
            }
            let mut command = Command::new(binary.as_ref());
            command.stderr(Stdio::piped()).stdout(Stdio::piped());
            if let Some(arg) = arg {
                command.arg(arg);
            }
            let output = command.output()?;
            let mut stderr = io::stderr();
            stderr.write_all(&output.stderr)?;
            stderr.write_all(&output.stdout)?;
            std::process::exit(output.status.code().unwrap_or(0));
        }
    }
}
