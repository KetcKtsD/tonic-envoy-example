use rand::random;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tonic::transport::Server;
use crate::generated::echo_service_server::{EchoService, EchoServiceServer};
use crate::generated::{ClientStreamingEchoRequest, ClientStreamingEchoResponse, EchoRequest, EchoResponse, Empty, ServerStreamingEchoRequest};


mod generated {
    include!("generated/grpc.gateway.testing.rs");
}

type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Default)]
struct Greeter;

impl EchoService for Greeter {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let req = request.into_inner();
        println!("Got a request from {:?}", req);
        if random::<bool>() {
            let resp = EchoResponse {
                message: req.message,
                message_count: 1,
            };
            Ok(Response::new(resp))
        } else {
            Err(Status::unavailable("unavailable"))
        }
    }

    async fn echo_abort(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        Err(Status::unavailable("unavailable"))
    }

    async fn no_op(&self, request: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::unavailable("unavailable"))
    }

    type ServerStreamingEchoStream = ();

    async fn server_streaming_echo(&self, request: Request<ServerStreamingEchoRequest>) -> Result<Response<Self::ServerStreamingEchoStream>, Status> {
        Err(Status::unavailable("unavailable"))
    }

    type ServerStreamingEchoAbortStream = ();

    async fn server_streaming_echo_abort(&self, request: Request<ServerStreamingEchoRequest>) -> Result<Response<Self::ServerStreamingEchoAbortStream>, Status> {
        Err(Status::unavailable("unavailable"))
    }

    async fn client_streaming_echo(&self, request: Request<Streaming<ClientStreamingEchoRequest>>) -> Result<Response<ClientStreamingEchoResponse>, Status> {
        Err(Status::unavailable("unavailable"))
    }

    type FullDuplexEchoStream = ();

    async fn full_duplex_echo(&self, request: Request<Streaming<EchoRequest>>) -> Result<Response<Self::FullDuplexEchoStream>, Status> {
        Err(Status::unavailable("unavailable"))
    }

    type HalfDuplexEchoStream = ();

    async fn half_duplex_echo(&self, request: Request<Streaming<EchoRequest>>) -> Result<Response<Self::HalfDuplexEchoStream>, Status> {
        Err(Status::unavailable("unavailable"))
    }
}


#[tokio::main]
async fn main() -> MainResult {
    let addr = "0.0.0.0:9090".parse().unwrap();
    println!("GreeterServer listening on {}", addr);
    Server::builder()
        .add_service(EchoServiceServer::new(Greeter::default()))
        .serve(addr)
        .await?;
    Ok(())
}
