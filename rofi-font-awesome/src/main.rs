use std::io::Write;
use std::process::{Command, Stdio};

use failure::Error;
use rofi::Rofi;
use wl_clipboard_rs::copy;

const FONT_AWESOME_4: &str = include_str!("../data/font-awesome-4.txt");
// const FONT_AWESOME_5: &str = include_str!("../../data/font-awesome-5.txt");

fn main() -> Result<(), Error> {
    // call rofi
    let rofi = Rofi::builder()
        .dmenu(true)
        .i(true)
        .markup_rows(true)
        .p("font-awesome")
        .build();
    let s = match rofi.run(FONT_AWESOME_4)? {
        Some(s) => s,
        None => return Ok(()),
    };

    // parse output
    let s = parse_data(&s)?;

    // copy to clipboard
    copy_to_clipboard(&s)?;

    Ok(())
}

fn parse_data(s: &str) -> Result<String, Error> {
    let start_index = s
        .find("&#x")
        .ok_or_else(|| failure::err_msg("Parsing error."))?
        + 3;
    let s = unsafe { std::str::from_utf8_unchecked(&s.as_bytes()[start_index..]) };
    let mut iter = s.chars();
    let mut acc = 0;
    let mut count = 0;
    while let Some(c) = iter.next() {
        if c == ';' {
            break;
        }
        count += 1;
        acc *= 16;
        acc += c
            .to_digit(16)
            .ok_or_else(|| failure::err_msg("Invalid unicode."))?;
    }
    if count > 6 {
        failure::bail!("Invalid unicode.");
    }
    let c = std::char::from_u32(acc).ok_or_else(|| failure::err_msg("Invalid unicode."))?;
    Ok(c.to_string())
}

fn copy_to_clipboard<S>(s: S) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let s = s.as_ref();
    copy_to_x(s)?;
    let _ = copy_to_wayland(s); // Fail silently on Wayland for the moment
    Ok(())
}

fn copy_to_x(s: &str) -> Result<(), Error> {
    let mut child = Command::new("xclip")
        .args(&["-selection", "c"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child.stdin.as_mut().unwrap().write_all(s.as_bytes())?;
    let exit_status = child.wait()?;
    if !exit_status.success() {
        failure::bail!(
            "xclip exited unsuccessfully with exit code {:?}.",
            exit_status.code()
        );
    }
    Ok(())
}

fn copy_to_wayland(s: &str) -> Result<(), Error> {
    let mut options = copy::Options::new();
    options.clipboard(copy::ClipboardType::Regular);
    options.copy(copy::Source::Bytes(s.as_bytes()), copy::MimeType::Text)?;
    Ok(())
}
