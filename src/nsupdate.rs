use std::process::{Command, Stdio};
use std::io;
use std::io::Write;

use  std::net::IpAddr;

#[derive(Debug)]
pub enum NSUpdateError {
    Io(io::Error),
    General(String)
}

impl From<io::Error> for NSUpdateError {
    fn from(err: io::Error) -> NSUpdateError {
        NSUpdateError::Io(err)
    }
}

impl<'a> From<&'a str> for NSUpdateError {
    fn from(s: &str) -> NSUpdateError {
        NSUpdateError::General(s.to_string())
    }
}

const NSUPDATE_KEY: &'static str = "/home/joerg/Kcapsec.org.+157+18350.key";

fn nsupdate_commands(name: &str, ip: IpAddr) -> String {
    format!("
server ns.capsec.org
zone d.capsec.org

update delete try.d.capsec.org a
update add {name} 3600 a {ip}
send
", name=name, ip=ip)
}

pub fn update_dns(name: &str, ip: IpAddr) -> Result<(), NSUpdateError> {
    // Create nsupdate child process


    let mut child = Command::new("nsupdate")
        .arg("-v")
        .arg("-k").arg(NSUPDATE_KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not start nsupdate");

    // Feed child with instructions...
    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(nsupdate_commands(name, ip).as_bytes())?;
    }

    let ecode = child.wait().expect("Failed to wait on child");

    // let stdout = child.stdout.as_mut().unwrap();
    // println!("{}", String::from_utf8_lossy(&stdout));

    if ecode.success() {
        return Ok(());
    } else {
        return Err(From::from("Failed to run nsupdate"));
    }
}


#[test]
fn test_update_dns() {
    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    assert!(update_dns("try.d.capsec.org", ip).is_ok());
}
