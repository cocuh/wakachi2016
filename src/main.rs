#[macro_use]
extern crate nickel;
use nickel::{Nickel, HttpRouter, JsonBody, StaticFilesHandler};
use nickel::extensions::Redirect;

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate mecab;

extern crate hyper;
use hyper::header::AccessControlAllowOrigin;


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
        if self.body.len() >= 512 {
            return Err("too long");
        }
        let mut tagger = mecab::Tagger::new("");
        let mut result = Vec::new();
        let input :&str = &self.body.clone();
        for (id,node) in tagger.parse_to_node(input).iter_next().enumerate() {
            match node.stat as i32 {
                mecab::MECAB_BOS_NODE => {
                }
                mecab::MECAB_EOS_NODE => {
                }
                _ => {
                    match Word::from_node(id, node) {
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
    id: usize,
    tango: String,
    yomi: String,
    hinshi: String,
    kihonkei: String,
}

impl Word {
    fn from_node(id: usize, node: mecab::Node) -> Option<Word> {
        let tango = &(node.surface)[..node.length as usize];
        let feature = node.feature.split(",").collect::<Vec<_>>();

        let hinshi = get_with_default(&feature, 0, "");
        let yomi = get_with_default(&feature, 7, "");
        let kihonkei = get_with_default(&feature, 6, "");

        let word = Word {
            id: id,
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

    server.utilize(StaticFilesHandler::new("dist/"));

    server.post("/query", middleware! { |req, mut res|
        let doc = req.json_as::<Document>().unwrap();

        //res.set(AccessControlAllowOrigin::Any);
        res.set(nickel::mimes::MediaType::Json);

        let result = match doc.run_mecab() {
            Ok(v) => QueryResponse::ok(v),
            Err(e) => QueryResponse::error(e),
        };


        json::encode(&result).unwrap()
    });

    server.get("/", middleware! { |_, res|
        return res.redirect("/index.html")
    });

    server.get("/debug", middleware! { |_, res|
        let doc = Document::new("こくさんのおいしいおにく");

        let result = match doc.run_mecab() {
            Ok(v) => QueryResponse::ok(v),
            Err(_) => QueryResponse::error("mecab run"),
        };

        json::encode(&result).unwrap()
    });

    server.listen("192.168.0.123:8888");
}



