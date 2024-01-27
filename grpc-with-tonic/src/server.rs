use tonic::{transport::Server, Request, Response, Status};

use simple_service::simple_service_server::{SimpleService, SimpleServiceServer};
use simple_service::{MessageRequest, MessageResponse};

pub mod simple_service {
    tonic::include_proto!("simple_service");
}

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl SimpleService for Service {
    async fn message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        println!("Got a request: {:#?}", request);

        let response = MessageResponse {
            response: request.into_inner().message.into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = Service::default();

    Server::builder()
        .add_service(SimpleServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
