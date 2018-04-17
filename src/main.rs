extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

fn main() {
    let client = DynamoDbClient::simple(Region::ApNortheast1);
    println!("Hello, world!");
}
