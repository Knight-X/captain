use jsonrpsee_http_client::HttpClientBuilder;
use jsonrpsee_types::{jsonrpc::Params, traits::Client, jsonrpc::JsonValue};
use tokio::runtime::Builder;

pub async fn process(_string: String) -> Result<(), Box<dyn std::error::Error>> {

        let _str = _string.as_str();
        let uri = format!("http://{}", "127.0.0.1:3000");
        let client = HttpClientBuilder::default().build(&uri)?;
        let response: Result<String, _> = client.request("run_wasm", Params::Array(
          vec![JsonValue::String(_str.into())])).await;
        println!("r: {:?}", response);

        Ok(())
}

pub fn task (_string: String)  {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    
    runtime.block_on(async move {
         process(_string).await;
    });
}
