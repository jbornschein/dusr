
//! Update records on remote DNS server


use std::process::{Command, Stdio};
use std::io;
use std::io::Write;

use std::net::IpAddr;

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


pub struct Updater {
    nameserver: String,
    default_ttl: i32,
}

impl Updater {
    fn nsupdate_commands(&self, name: &str, ip: IpAddr) -> String {
        format!("
        server {nameserver}

        update delete {name} a
        update add {name} {ttl} a {ip}
        send
        ", nameserver=self.nameserver, ttl=self.default_ttl,name=name, ip=ip)
    }

    pub fn new() -> Updater {
        Updater {
            nameserver: "ns.capsec.org".to_string(),
            default_ttl: 3600,
        }
    }

    pub fn update_dns(&self, name: &str, ip: IpAddr) -> Result<(), NSUpdateError> {
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

            stdin.write_all(
                self.nsupdate_commands(name, ip).as_bytes())?;
        }

        let ecode = child.wait().expect("Failed to wait on child");

        if ecode.success() {
            return Ok(());
        } else {
            return Err(From::from("Failed to run nsupdate"));
        }
    }
}



#[test]
#[ignore]
fn test_update_dns() {
    let updater = Updater::new();
    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    assert!(updater.update_dns("try.d.capsec.org", ip).is_ok());
}
