use dumb_json_parser::{self, Value};
use reqwest::blocking;
use std::env;

// Hacky, zero effort CLI written to see how horrible the parser is to use, lol

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Please provide a crate name. \n e.g.: `crate_checker trimsec`");
        return;
    }

    let client = blocking::Client::builder()
        .user_agent("github.com/furtidev/dumb_json_parser")
        .build()
        .unwrap();

    let resp_main = client
        .get(format!(
            "https://crates.io/api/v1/crates/{}?include=categories",
            args[1]
        ))
        .send()
        .unwrap();

    let json_main = dumb_json_parser::parse(resp_main.text().unwrap());

    let resp_developers = client
        .get(format!(
            "https://crates.io/api/v1/crates/{}/owner_user",
            args[1]
        ))
        .send()
        .unwrap();

    let json_developers = dumb_json_parser::parse(resp_developers.text().unwrap());

    if let Some(Value::Obj(body)) = json_main.child {
        let Some(Value::Obj(raw_dev_list)) = json_developers.child else {
            println!("invalid json body received");
            return;
        };

        let Value::Arr(ref dev_list) = raw_dev_list.children[0].rhs else {
            println!("invalid json body received");
            return;
        };

        let Value::Obj(ref crate_object) = body.children[0].rhs else {
            println!("invalid json body received");
            return;
        };

        let Value::Str(ref name) = crate_object.children[1].rhs else {
            println!("invalid json body received");
            return;
        };

        let Value::Num(ref downloads) = crate_object.children[8].rhs else {
            println!("invalid json body received");
            return;
        };

        let Value::Str(ref latest_version) = crate_object.children[10].rhs else {
            println!("invalid json body received");
            return;
        };

        let Value::Str(ref description) = crate_object.children[15].rhs else {
            println!("invalid json body received");
            return;
        };

        println!("{} - {}", name, description);
        println!("  Developed by:");
        dev_list.children.iter().for_each(|raw_user| {
            let Value::Obj(user) = raw_user else { return };

            let Value::Str(ref user_login) = user.children[1].rhs else {
                println!("invalid json body received");
                return;
            };

            println!("    - {}", user_login.clone())
        });
        println!(
            "  {} downloads, currently on version {}",
            downloads, latest_version
        );
    }
}
