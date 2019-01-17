use log::{error, info};

use bulletproof_aggregator::Point;

fn main() {
    pretty_env_logger::init();

    println!("Test");

    let point = Point { x: 10, y: 40 };

    let client = reqwest::Client::new();
    let mut res = client
        .post("http://localhost:3000")
        .body(serde_cbor::ser::to_vec_packed(&point).unwrap())
        .send()
        .unwrap();

    info!("Status = {:?}", res.status());
    info!("Header = {:?}", res.headers());

    match res.text() {
        Ok(t) => info!("Body = {}", t),
        _ => {}
    };
}
