use bytes;
use prost::Message;
use tokio;
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::body::content_length_limit(1024 * 32)
        .and(warp::body::bytes())
        .map(|bytes: bytes::Bytes| {
            println!("bytes = {:?}", bytes);
            let msg = home_auto::messages::ThermostatState::decode(bytes);
            println!("msg = {:?}", msg);
            "ok"
        });

    warp::serve(route).run(([127, 0, 0, 1], 8080)).await
}
