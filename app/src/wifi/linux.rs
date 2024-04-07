use std::process::Command;
use regex::Regex;

pub fn get_ssid()-> anyhow::Result<String>{
    let output = Command::new("iwgetid").arg("-r").output()?;
    Ok(std::str::from_utf8(output.stdout.as_slice())?.to_owned())
}
pub fn get_signal()->anyhow::Result<f32>{
    let regex =  Regex::new(r"Link Quality=(\d+)/(\d+)")?;
    let output = Command::new("iwconfig").output()?;
    let output = std::str::from_utf8(output.stdout.as_slice())?;
    let caps = regex.captures(output);
    if let Some(caps) = caps{
        return Ok(&caps[1].parse::<f32>()? / &caps[2].parse::<f32>()?);
    }
    Ok(0.0)
}