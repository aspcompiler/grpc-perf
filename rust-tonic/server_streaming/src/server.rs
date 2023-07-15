use futures::Stream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use server_streaming::server_streaming_server::{ServerStreaming, ServerStreamingServer};
use server_streaming::{StreamingFromServerRequest, StreamingFromServerResponse};

pub mod server_streaming {
    tonic::include_proto!("server_streaming");
}

#[derive(Debug)]
pub struct ServerStreamingService {}

const MAX_MESSAGE_SIZE: usize = 1 << 20;

#[tonic::async_trait]
impl ServerStreaming for ServerStreamingService {
    type StreamingFromServerStream = MyServerLogic;

    async fn streaming_from_server(
        &self,
        request: Request<StreamingFromServerRequest>,
    ) -> Result<Response<Self::StreamingFromServerStream>, Status> {
        // info!("StreamingFromServer = {:?}", request);
        Ok(Response::new(MyServerLogic {
            total_bytes_to_handle: request.into_inner().num_bytes as usize,
        }))
    }
}

pub struct MyServerLogic {
    total_bytes_to_handle: usize,
}

impl Stream for MyServerLogic {
    type Item = Result<StreamingFromServerResponse, Status>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.total_bytes_to_handle > 0 {
            // handle protobuf size limits by only sending 1MB per message
            let bytes_to_send = std::cmp::min(self.total_bytes_to_handle, MAX_MESSAGE_SIZE);
            self.total_bytes_to_handle -= bytes_to_send;
            let data = vec![0; bytes_to_send];
            std::task::Poll::Ready(Some(Ok(StreamingFromServerResponse { data })))
        } else {
            std::task::Poll::Ready(None)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let addr = "[::]:50051".parse().unwrap();
    let service = ServerStreamingService {};
    Server::builder()
        .add_service(ServerStreamingServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
