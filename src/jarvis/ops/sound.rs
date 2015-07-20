use std::io::prelude::*;
use std::process::{
    Command,
    Stdio,
    Child,
};

use util::{
    Error,
    Result,
};

const AMIXER: &'static str = "amixer";
const FLAGS: &'static [&'static str] = &["-M"];
const SET: &'static str = "sset";
const GET: &'static str = "sget";
const DEVICE: &'static str = "DAC1";

pub fn set(val: u8) -> Result<u8> {
    let mut cmd = Command::new(AMIXER);
    cmd.arg(SET).args(FLAGS).arg(DEVICE).arg(format!("{}%", val));
    cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = try!(cmd.spawn());
    let status = try!(child.wait());

    if !status.success() {
        let mut err = Error::new("failed to set sound level");
        if let Some(mut err_pipe) = child.stderr {
            let mut err_str = String::new();
            try!(err_pipe.read_to_string(&mut err_str));
            err.add_detail(&err_str);
            Err(err)
        } else {
            Err(err)
        }
    } else {
        if let Some(mut out_pipe) = child.stdout {
            let mut out_str = String::new();
            try!(out_pipe.read_to_string(&mut out_str));
            let val = try!(extract_volume(&out_str));
            Ok(val)
        } else {
            Err(Error::new("failed to get new sound level"))
        }
    }
}

pub fn get() -> Result<u8> {
    let mut cmd = Command::new(AMIXER);
    cmd.arg(GET).args(FLAGS).arg(DEVICE);
    cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = try!(cmd.spawn());
    let status = try!(child.wait());

    if !status.success() {
        let mut err = Error::new("failed to get sound level");
        if let Some(mut err_pipe) = child.stderr {
            let mut err_str = String::new();
            try!(err_pipe.read_to_string(&mut err_str));
            err.add_detail(&err_str);
            Err(err)
        } else {
            Err(err)
        }
    } else {
        if let Some(mut out_pipe) = child.stdout {
            let mut out_str = String::new();
            try!(out_pipe.read_to_string(&mut out_str));
            let val = try!(extract_volume(&out_str));
            Ok(val)
        } else {
            Err(Error::new("failed to get sound level"))
        }
    }
}

fn extract_volume(string: &str) -> Result<u8> {
    let strval: String = string.chars()
                    .skip_while(|&c| c != '[').skip(1)
                    .take_while(|&c| c != '%')
                    .collect();
    if strval.is_empty() {
        Err(Error::new("failed to find sound level"))
    } else {
        let numval: u8 = try!(strval.parse());
        Ok(numval)
    }
}

pub fn increase(val: u8) -> Result<()> {
    unimplemented!();
}

pub fn decrease(val: u8) -> Result<()> {
    unimplemented!();
}
