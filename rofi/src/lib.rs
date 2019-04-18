use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use failure::Error;

#[derive(Default)]
pub struct Rofi {
    args: Vec<Cow<'static, str>>,
}

impl Rofi {
    pub fn builder() -> RofiBuilder {
        RofiBuilder::default()
    }

    pub fn run<S>(&self, input: S) -> Result<Option<String>, Error>
    where
        S: AsRef<str>,
    {
        let child = Command::new("rofi")
            .args(self.args.iter().map(|s| s.as_ref()))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        child.stdin.unwrap().write_all(input.as_ref().as_bytes())?;
        let mut s = String::new();
        child.stdout.unwrap().read_to_string(&mut s)?;
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }

    pub fn run_from_file(&self, input: File) -> Result<Option<String>, Error> {
        let child = Command::new("rofi")
            .args(self.args.iter().map(|s| s.as_ref()))
            .stdin(input)
            .stdout(Stdio::piped())
            .spawn()?;
        let mut s = String::new();
        child.stdout.unwrap().read_to_string(&mut s)?;
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }
}

pub struct RofiBuilder {
    dmenu: bool,
    i: bool,
    markup_rows: bool,
    p: String,
}

impl Default for RofiBuilder {
    fn default() -> RofiBuilder {
        RofiBuilder {
            dmenu: false,
            i: false,
            markup_rows: false,
            p: "Selection".to_string(),
        }
    }
}

impl RofiBuilder {
    pub fn build(self) -> Rofi {
        let mut args = Vec::new();
        if self.dmenu {
            args.push("-dmenu".into());
        }
        if self.i {
            args.push("-i".into());
        }
        if self.markup_rows {
            args.push("-markup-rows".into());
        }
        args.push("-p".into());
        args.push(self.p.into());
        Rofi { args }
    }
    pub fn dmenu(mut self, boolean: bool) -> Self {
        self.dmenu = boolean;
        self
    }
    pub fn i(mut self, boolean: bool) -> Self {
        self.i = boolean;
        self
    }
    pub fn markup_rows(mut self, boolean: bool) -> Self {
        self.markup_rows = boolean;
        self
    }
    pub fn p<S>(mut self, prompt: S) -> Self
    where
        S: Into<String>,
    {
        self.p = prompt.into();
        self
    }
}
