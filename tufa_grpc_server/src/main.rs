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
#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl payments::bitcoin_server::Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: tonic::Request<payments::BtcPaymentRequest>,
    ) -> Result<tonic::Response<payments::BtcPaymentResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = payments::BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr),
        };
        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();
    tonic::transport::Server::builder()
        .add_service(payments::bitcoin_server::BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;
    Ok(())
}
