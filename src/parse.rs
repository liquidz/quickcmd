use std::str::FromStr;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub enum Request {
    Url(String),
    Search(String, String),
}

impl FromStr for Request {
    type Err = Error;

    fn from_str(s: &str) -> Result<Request, Error> {
        let res: Vec<&str> = s.splitn(2, ' ').collect();
        match res.len() {
            1 => Ok(Request::Url(res[0].to_string())),
            2 => Ok(Request::Search(res[0].to_string(), res[1].to_string())),
            _ => Err(Error::new(ErrorKind::InvalidInput, "invalid")),
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(Request::Url("foo".to_string()),
               "foo".parse::<Request>().unwrap());

    assert_eq!(Request::Search("foo".to_string(), "bar".to_string()),
               "foo bar".parse::<Request>().unwrap());
}
