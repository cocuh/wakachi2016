#[macro_use]
extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "hello youjo"
        }
    });

    server.listen("127.0.0.1:8888");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
