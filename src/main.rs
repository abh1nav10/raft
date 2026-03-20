mod expose;

pub mod codegen {
    tonic::include_proto!("grpc.examples.unaryecho");
}

use codegen::{EchoRequest, EchoResponse};
use tonic::{Request, Response, Status};

struct Server;

#[tonic::async_trait]
impl codegen::echoo_server::Echoo for Server {
    async fn unary_echo(&self, _: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let message = String::from("Hello there!!!!!");

        Ok(Response::new(EchoResponse { message }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socketaddr: std::net::SocketAddr = "127.0.0.1:3000".parse()?;

    let service = Server;

    tonic::transport::Server::builder()
        .add_service(codegen::echoo_server::EchooServer::new(service))
        .serve(socketaddr)
        .await?;

    Ok(())
}
