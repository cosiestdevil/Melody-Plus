use std::process::Command;

pub fn get_ssid() -> anyhow::Result<String>{
    Ok(std::str::from_utf8(Command::new("powershell").arg("-Command {(get-netconnectionProfile).Name}").output()?.stdout.as_slice())?.to_owned())
}
pub fn get_signal()->anyhow::Result<f32>{
    Ok(1.0)
}