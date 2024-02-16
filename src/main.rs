use jsonrpsee::{
    core::{
        client::{ClientT, Error},
        params::ObjectParams,
        JsonValue,
    },
    http_client::{HeaderMap, HeaderValue, HttpClientBuilder},
    rpc_params,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cookie =
        std::fs::read_to_string(std::env::var("HOME").unwrap() + "/.bitcoin/signet/.cookie")
            .unwrap_or_else(|e| panic!("Failed reading .cookie: {e}"));
    let auth = "Basic ".to_string() + &base64_light::base64_encode(&cookie);
    let mut headers = HeaderMap::new();
    headers.insert("authorization", HeaderValue::from_str(&auth).unwrap());

    let client = HttpClientBuilder::default()
        .set_headers(headers)
        .build("http://localhost:38332/")
        .unwrap();

    let response_hash = client
        .request::<String, _>("getblockhash", rpc_params![0])
        .await?;
    println!(
        "Genesis block hash with positional params array: {}",
        response_hash
    );

    let mut params = ObjectParams::new();
    params.insert("blockhash", response_hash)?;
    let response_block = client.request::<JsonValue, _>("getblock", params).await?;
    println!(
        "Genesis block data with named params object: {:#?}",
        response_block
    );

    Ok(())
}
