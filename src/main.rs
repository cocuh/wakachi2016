use std::collections::HashMap;

#[macro_use]
extern crate nickel;
use nickel::{Nickel, HttpRouter, JsonBody, StaticFilesHandler};

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate mecab;


#[derive(RustcDecodable, RustcEncodable, Debug)]
struct QueryResponse {
    is_success: bool,
    data: Vec<Word>,
    msg: String,
}

impl QueryResponse {
    fn ok(data: Vec<Word>) -> QueryResponse {
        QueryResponse {
            is_success: true,
            data: data,
            msg: String::from(""),
        }
    }
    fn error(msg: &str) -> QueryResponse{
        QueryResponse {
            is_success: false,
            data: Vec::new(),
            msg: String::from(msg),
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Document {
    body: String,
}

impl Document {
    fn new(body: &str) -> Self{
        Document {
            body: String::from(body),
        }
    }

    fn run_mecab(&self) -> Result<Vec<Word>, String> {
        let mut tagger = mecab::Tagger::new("");
        let mut result = Vec::new();
        let input :&str = &self.body.clone();
        for node in tagger.parse_to_node(input).iter_next() {
            match node.stat as i32 {
                mecab::MECAB_BOS_NODE => {
                }
                mecab::MECAB_EOS_NODE => {
                }
                _ => {
                    match Word::from_node(node) {
                        Some(word) => {
                            result.push(word)
                        }
                        None => {
                        }
                    }
                }
            }
        }
        return Ok(result);
    }
}


fn get_with_default<T:Clone>(vec :&Vec<T>, idx: usize, default: T) -> T{
    match vec.get(idx) {
        Some(v) => v.clone(),
        None => default,
    }
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Word {
    tango: String,
    yomi: String,
    hinshi: String,
    kihonkei: String,
}

impl Word {
    fn from_node(node: mecab::Node) -> Option<Word> {
        let tango = &(node.surface)[..node.length as usize];
        let feature = node.feature.split(",").collect::<Vec<_>>();
        println!("{} {}", tango, node.feature);

        let hinshi = get_with_default(&feature, 0, "");
        let yomi = get_with_default(&feature, 7, "");
        let kihonkei = get_with_default(&feature, 6, "");

        let word = Word {
            tango: String::from(tango),
            yomi: String::from(yomi),
            hinshi: String::from(hinshi),
            kihonkei: String::from(kihonkei),
        };
        Some(word)
    }
}


fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("asserts/"));

    server.post("/query", middleware! { |req, mut res|
        let doc = req.json_as::<Document>().unwrap();

        res.set(nickel::mimes::MediaType::Json);

        let result = match doc.run_mecab() {
            Ok(v) => QueryResponse::ok(v),
            Err(_) => QueryResponse::error("mecab run"),
        };

        json::encode(&result).unwrap()
    });

    server.get("/", middleware! { |_, res|
        let mut data = HashMap::new();
        data.insert("data", "こくさんのおいしいおにく");
        return res.render("templates/main.tpl", &data);
    });

    server.get("/debug", middleware! { |_, res|
        let doc = Document::new("こくさんのおいしいおにく");

        let result = match doc.run_mecab() {
            Ok(v) => QueryResponse::ok(v),
            Err(_) => QueryResponse::error("mecab run"),
        };

        json::encode(&result).unwrap()
    });

    server.listen("127.0.0.1:8888");
}


