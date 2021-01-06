/// This Source Code Form is subject to the terms of the Mozilla Public
/// License, v. 2.0. If a copy of the MPL was not distributed with this
/// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate lazy_static;
mod rest;

use clap::{App, Arg};
use rest::server;

fn main() {
    let version_id = "0.1.0";
    let matches = App::new("hugin")
        .version(version_id)
        .author("Étienne Marais <etienne@maiste.fr>")
        .about("REST Server for Vanir.")
        .subcommand(
            App::new("run")
                .about("Run the REST Server")
                .version(version_id)
                .arg(
                    Arg::with_name("port")
                        .help("Change the exposed port. Default is 8080.")
                        .short("p")
                        .long("port")
                        .default_value("8080"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("run") => {
            let port = matches
                .subcommand_matches("run")
                .unwrap()
                .value_of("port")
                .unwrap();
            println!("Execute server right now on port {}!", port);
            server::run(String::from(port)).expect("Server failed!");
        }
        None => println!("I'm Hugin, a REST Server for the Vanir project."),
        _ => println!("Unknown command, sorry..."),
    };
}
