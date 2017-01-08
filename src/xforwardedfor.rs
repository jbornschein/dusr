
use std::net::IpAddr;
use std::str::FromStr;

use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::outcome::Outcome::*;


/// Representation of the X-Forwarded-For header.
///
/// Can be used to extract the remote IP address of a request.
/// The list typically contains the original client as first entry;
/// Successive entries correspond to further HTTP proxies. We return
/// an empty list when the header isn't present.
pub struct XForwardedFor {
    addrs: Vec<IpAddr>,
}

impl  XForwardedFor {
    fn new(addrs: Vec<IpAddr>) -> XForwardedFor {
        XForwardedFor { addrs: addrs }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for XForwardedFor {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<XForwardedFor, Self::Error> {
        let headers = request.headers();
        let mut addrs = Vec::with_capacity(3);
        for val in headers.get("X-Forwarded-For") {
            for ip_str in val.split(",") {
                let ip_str = ip_str.trim();
                match IpAddr::from_str(ip_str) {
                    Ok(addr) => addrs.push(addr),
                    // TODO remove println!
                    Err(parse_error) => println!("Failed parsing ip {}: {:?}", ip_str, parse_error)
                }
            }
        }
        Success(XForwardedFor::new(addrs))
    }
}

/////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
use rocket::http::{Method, Header};

#[test]
fn test_empty() {
    let request = Request::new(Method::Get, "/test");
    let xforwarded = XForwardedFor::from_request(&request);

    assert!(xforwarded.is_success());
    assert!(xforwarded.unwrap().addrs.len() == 0);
}

#[test]
fn test_ipv4() {
    let mut request = Request::new(Method::Get, "/test");
    request.add_header(Header::new("X-Forwarded-For", "1.1.1.1, 2.2.2.2"));

    let xforwarded = XForwardedFor::from_request(&request);

    assert!(xforwarded.is_success());
    assert!(xforwarded.unwrap().addrs.len() == 2);
}

#[test]
fn test_ipv6() {
    let mut request = Request::new(Method::Get, "/test");
    request.add_header(Header::new("X-Forwarded-For", "2001::23:42, f800:1::1"));

    let xforwarded = XForwardedFor::from_request(&request);

    assert!(xforwarded.is_success());
    assert!(xforwarded.unwrap().addrs.len() == 2);
}
