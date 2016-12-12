use std::str::FromStr;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
enum Request {
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

// fn split(s: &str) -> Vec<&str> {
//    let res: Vec<&str> = s.splitn(2, ' ').collect();
//    res
// }
//
// fn parse(s: &str) -> Request {
//    let res = split(s);
//    match res.len() {
//        1 => Request::Url(res[0].to_string()),
//        2 => Request::Search(res[0].to_string(), res[1].to_string()),
//        _ => Request::InvalidRequest,
//    }
// }
//
// #[test]
// fn test_split() {
//    let v = split("hello world");
//    assert_eq!(v, ["hello", "world"]);
// }
//
// #[test]
// fn test_parse() {
//    match parse("foo") {
//        Request::Url(v) => assert_eq!(v, "foo"),
//        v => panic!("error: {:?}", v),
//    }
//    match parse("foo bar") {
//        Request::Search(k, v) => {
//            assert_eq!(k, "foo");
//            assert_eq!(v, "bar");
//        }
//        v => panic!("error: {:?}", v),
//    }
// }
