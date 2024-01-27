use aes::cipher::{
    generic_array::GenericArray, BlockCihper, BlockDecrypt, BlockEncrypt, GenericArray, KeyInit,
};
use aes::Aes128;
use simple_service::simple_service_client::SimpleServiceClient;
use simple_service::MessageRequest;

pub mod simple_service {
    tonic::include_proto!("simple_service");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SimpleServiceClient::connect("http://[::1]:50551").await?;

    let message = String::from("Hello World!");
    // encrypt message with AES
    let key = GenericArray::from([0u8; 16]);
    let cipher = Aes128::new(&key);
    let block = GenericArray::from(message.as_bytes());

    let request = tonic::Request::new(MessageRequest {
        message: "Hello World!".into(),
    });

    let key = aes::Key::generate();

    let response = client.message(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
