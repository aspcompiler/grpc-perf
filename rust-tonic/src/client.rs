use server_streaming::server_streaming_client::ServerStreamingClient;
use server_streaming::StreamingFromServerRequest;

pub mod server_streaming {
    tonic::include_proto!("server_streaming");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ServerStreamingClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(StreamingFromServerRequest {
        num_bytes: 640 * 1024 * 1024,
    });
    let mut bytes_received = 0;
    let mut stream = client.streaming_from_server(request).await?.into_inner();
    // counter the time
    let start = std::time::Instant::now();
    while let Some(response) = stream.message().await? {
        bytes_received += response.data.len();
    }
    let duration = start.elapsed();
    println!("{:.2}",
        bytes_received as f64 / duration.as_secs_f64()
    );
    Ok(())
}
