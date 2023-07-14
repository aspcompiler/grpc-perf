use std::mem;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use server_streaming::server_streaming_server::{ServerStreaming, ServerStreamingServer};
use server_streaming::{StreamingFromServerRequest, StreamingFromServerResponse};

pub mod server_streaming {
    tonic::include_proto!("server_streaming");
}

#[derive(Debug)]
pub struct ServerStreamingService {}

const MAX_MESSAGE_SIZE: u32 = 1 << 20;

#[tonic::async_trait]
impl ServerStreaming for ServerStreamingService {
    type StreamingFromServerStream = ReceiverStream<Result<StreamingFromServerResponse, Status>>;

    async fn streaming_from_server(
        &self,
        request: Request<StreamingFromServerRequest>,
    ) -> Result<Response<Self::StreamingFromServerStream>, Status> {
        println!("StreamingFromServer = {:?}", request);

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            let mut packet_index: u64 = 0;
            let mut bytes_remaining = request.get_ref().num_bytes;

            while bytes_remaining > 0 {
                // handle protobuf size limits by only sending 1MB per message
                let mut bytes_to_send = std::cmp::min(bytes_remaining, MAX_MESSAGE_SIZE);
                bytes_remaining -= bytes_to_send;
                let mut data: Vec<u64> = Vec::with_capacity(bytes_to_send as usize / 8);
                while bytes_to_send > 0 {
                    data.push(packet_index);
                    bytes_to_send -= 8;
                    packet_index += 1;
                }

                let data = unsafe {
                    let ratio = mem::size_of::<u64>() / mem::size_of::<u8>();

                    let length = data.len() * ratio;
                    let capacity = data.capacity() * ratio;
                    let ptr = data.as_mut_ptr() as *mut u8;

                    // Don't run the destructor for vec32
                    mem::forget(data);

                    // Construct new Vec
                    Vec::from_raw_parts(ptr, length, capacity)
                };

                match tx
                    .send(Result::<_, Status>::Ok(StreamingFromServerResponse {
                        data,
                    }))
                    .await
                {
                    Ok(_) => {}
                    Err(err) => {
                        error!("get_results: failed to send data to client: {}", err);
                        break;
                    }
                }
            }

            info!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
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
