#[macro_use]
extern crate nom;
extern crate byteorder;

pub use self::protocols::*;
pub mod protocols;

use self::parser::*;
mod parser;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Multiaddr {
    bytes: Vec<u8>
}

impl Multiaddr {
    /// Create a new multiaddr based on a string representation, like
    /// `/ip4/127.0.0.1/udp/1234`.
    ///
    /// # Examples
    ///
    /// Simple construction
    ///
    /// ```
    /// use multiaddr::Multiaddr;
    ///
    /// let address = Multiaddr::new("/ip4/127.0.0.1/udp/1234").unwrap();
    /// assert_eq!(address.to_bytes(), [
    ///     0, 4, 127, 0, 0, 1,
    ///     0, 17, 4, 210
    /// ]);
    /// ```
    ///
    pub fn new(input: &str) -> Result<Multiaddr, ParseError> {
        let bytes = try!(parser::multiaddr_from_str(input));

        Result::Ok(Multiaddr {
            bytes: bytes,
        })
    }

    /// Return a copy to disallow changing the bytes directly
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_owned()
    }

    /// Return a list of protocols
    ///
    /// # Examples
    ///
    /// A single protocol
    ///
    /// ```
    /// use multiaddr::{Multiaddr, Protocols};
    ///
    /// let address = Multiaddr::new("/ip4/127.0.0.1").unwrap();
    /// assert_eq!(address.protocols(), vec![Protocols::IP4]);
    /// ```
    ///
    pub fn protocols(&self) -> Vec<Protocols> {
        parser::protocols_from_bytes(&self.bytes[..])
    }
}
