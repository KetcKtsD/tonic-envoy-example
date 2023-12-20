use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tonic::transport::Server;
use crate::generated::echo_service_server::{EchoService, EchoServiceServer};
use crate::generated::{ClientStreamingEchoRequest, ClientStreamingEchoResponse, EchoRequest, EchoResponse, Empty, ServerStreamingEchoRequest, ServerStreamingEchoResponse};


mod generated {
    include!("generated/grpc.gateway.testing.rs");
}

type MainResult = Result<(), Box<dyn std::error::Error>>;

struct Greeter;

#[tonic::async_trait]
impl EchoService for Greeter {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("{}: Got a request: {:?}", line!(), request);
        let req = request.into_inner();
        let response = EchoResponse {
            message: req.message,
            message_count: 0,
        };
        Ok(Response::new(response))
    }

    async fn echo_abort(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("{}: Got a request: {:?}", line!(), request);
        Err(Status::not_found("not_found"))
    }

    async fn no_op(&self, request: Request<Empty>) -> Result<Response<Empty>, Status> {
        println!("{}: Got a request: {:?}", line!(), request);
        Err(Status::already_exists("already_exists"))
    }

    type ServerStreamingEchoStream = ReceiverStream<Result<ServerStreamingEchoResponse, Status>>;

    async fn server_streaming_echo(&self, request: Request<ServerStreamingEchoRequest>) -> Result<Response<Self::ServerStreamingEchoStream>, Status> {
        println!("{}: Got a request: {:?}", line!(), request);
        Err(Status::cancelled("cancelled"))
    }

    type ServerStreamingEchoAbortStream = ReceiverStream<Result<ServerStreamingEchoResponse, Status>>;

    async fn server_streaming_echo_abort(&self, request: Request<ServerStreamingEchoRequest>) -> Result<Response<Self::ServerStreamingEchoAbortStream>, Status> {
        println!("{}: Got a request: {:?}", line!(), request);
        Err(Status::aborted("aborted"))
    }

    async fn client_streaming_echo(&self, request: Request<Streaming<ClientStreamingEchoRequest>>) -> Result<Response<ClientStreamingEchoResponse>, Status> {
        println!("{}: Got a request: {:?}", line!(), request);
        Err(Status::invalid_argument("invalid_argument"))
    }

    type FullDuplexEchoStream = ReceiverStream<Result<EchoResponse, Status>>;

    async fn full_duplex_echo(&self, request: Request<Streaming<EchoRequest>>) -> Result<Response<Self::FullDuplexEchoStream>, Status> {
        println!("{}: Got a request: {:?}", line!(), request);
        Err(Status::unauthenticated("unauthenticated"))
    }

    type HalfDuplexEchoStream = ReceiverStream<Result<EchoResponse, Status>>;

    async fn half_duplex_echo(&self, request: Request<Streaming<EchoRequest>>) -> Result<Response<Self::HalfDuplexEchoStream>, Status> {
        println!("{}: Got a request: {:?}", line!(), request);
        Err(Status::unimplemented("unimplemented"))
    }
}


#[tokio::main]
async fn main() -> MainResult {
    let addr = "0.0.0.0:9090".parse().unwrap();
    println!("GreeterServer listening on {}", addr);
    Server::builder()
        .add_service(EchoServiceServer::new(Greeter))
        .serve(addr)
        .await?;
    Ok(())
}
