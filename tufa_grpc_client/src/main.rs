#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = payments::bitcoin_client::BitcoinClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(payments::BtcPaymentRequest {
        from_addr: "123456".to_owned(),
        to_addr: "654321".to_owned(),
        amount: 22,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
