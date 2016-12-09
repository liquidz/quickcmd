#[derive(Debug)]
enum Search {
    Google(String),
    DuckDuckGo(String),
    NoAction,
}

impl Default for Search {
    fn default() -> Search {
        Search::NoAction
    }
}


#[test]
fn test_search_default() {
    let v: Search = Default::default();
    match v {
        Search::NoAction => assert!(true),
        v => panic!("error: {:?}", v),
    }
}
