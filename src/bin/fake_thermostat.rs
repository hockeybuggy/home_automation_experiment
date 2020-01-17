use prost::Message;
use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let msg = home_auto::create_thermostat_state("foo".to_string());
    // let expected_len = msg.encoded_len();
    let mut buf: Vec<u8> = Vec::with_capacity(200); // TODO fixed len for no reason
    msg.encode(&mut buf).unwrap();

    dbg!(msg);

    let client = reqwest::Client::new();
    let _resp = client
        .post("http://localhost:8080")
        .body(buf)
        .send()
        .await?;
    // println!("{:#?}", resp);
    println!("sent");
    Ok(())
}
