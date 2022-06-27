extern crate i3ipc;
extern crate pest;

use i3ipc::I3Connection;

mod model;
mod parser;

#[macro_use]
extern crate pest_derive;

fn main() {
    // establish a connection to i3 over a unix socket
    let mut connection = I3Connection::connect().unwrap();

    // request and print the i3 version
    let reply = connection.get_variable_replaced_config().unwrap();
    let config = parser::parse(reply.config);

    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("{}", json);
}
