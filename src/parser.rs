use std::fmt;
use std::error;
use std::str::from_utf8;

use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use nom::IResult;

use ::protocol_types::*;

/// Parse a single /
named!(sep <&[u8], &[u8]>, tag!("/"));

/// Parse a single multiaddress in the form of `/ip4/127.0.0.1`.
named!(address <&[u8], Vec<u8> >,
    chain!(
           opt!(sep)             ~
        t: map_res!(
             take_until!("/"),
             from_utf8
           )                     ~
           sep                   ~
        a: is_not!("/"),
        || {
            let mut res: Vec<u8>= Vec::new();

            // TODO: Better error handling
            // Write the u16 code into the results vector
            if let Some(protocol) = ProtocolTypes::from_name(t) {
                res.write_u16::<LittleEndian>(protocol.to_code()).unwrap();

                let address_bytes = protocol.address_string_to_bytes(a).unwrap();

                // Write the address into the results vector
                res.extend(address_bytes.iter().cloned());
            }

            res
        }
    )
);

/// Parse a list of addresses
named!(addresses < &[u8], Vec< Vec<u8> > >, many1!(address));

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The given multiaddress is invalid")
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Invalid multiaddress"
    }
}

pub fn multiaddr_from_str(input: &str) -> Result<Vec<u8>, ParseError> {
    match addresses(input.as_bytes()) {
        IResult::Done(_, res) => {
            let res = res.iter()
                .fold(Vec::new(), |mut v, a| {
                    v.extend(a.iter().cloned());
                    v
                });

            Result::Ok(res)
        },
        _ => Result::Err(ParseError),
    }
}

fn from_code(code: &[u8]) -> ProtocolTypes {
    let mut code = code;
    let code = code.read_u16::<LittleEndian>().unwrap();
    println!("code {:?}", code);
    ProtocolTypes::from_code(code).unwrap()
}

fn take_size<'a>(i: &'a [u8], code: &[u8]) -> IResult<&'a [u8], &'a [u8]> {
    println!("taking size {:?}", from_code(code).to_size());
    println!("{:?}", i);
    take!(i, from_code(code).to_size() / 8)
}

named!(protocol < &[u8], ProtocolTypes >,
    chain!(
        code: take!(2) ~
        a   : apply!(take_size, code),
        || {
            println!("code {:?}, {:?}", code, a);
            from_code(code)
        }
    )
);

named!(protocols < &[u8], Vec<ProtocolTypes> >, many1!(protocol));

/// Panics on invalid bytes as this would mean data corruption!
pub fn protocols_from_bytes(input: &[u8]) -> Vec<ProtocolTypes> {
    match protocols(input) {
        IResult::Done(i, res) => {
            println!("remaining {:?}", i);
            for p in &res {
                println!("results {:?}", p);
            }
            res
        },
        e => {
            println!("{:?}", e);
            panic!("Failed to parse internal bytes, possible corruption")
        },
    }
}
