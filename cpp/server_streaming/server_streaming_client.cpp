#include <chrono>
#include <iostream>
#include <memory>
#include <random>
#include <string>
#include <thread>

#include <grpc/grpc.h>
#include <grpcpp/channel.h>
#include <grpcpp/client_context.h>
#include <grpcpp/create_channel.h>
#include <grpcpp/security/credentials.h>

#include "server_streaming.grpc.pb.h"

using grpc::Channel;
using grpc::ClientContext;
using grpc::ClientReader;
using grpc::ClientReaderWriter;
using grpc::ClientWriter;
using grpc::Status;
using std::chrono::duration_cast;
using std::chrono::high_resolution_clock;
using std::chrono::milliseconds;

using server_streaming::StreamingFromServerRequest;
using server_streaming::StreamingFromServerResponse;
using server_streaming::ServerStreaming;

class ServerStreamingClient {
    public:
        ServerStreamingClient(std::shared_ptr<Channel> channel)
            : stub_(ServerStreaming::NewStub(channel)) {}

        // Assembles the client's payload, sends it and presents the response back
        // from the server.
        void StreamingFromServer(const uint32_t num_bytes) {
            // Data we are sending to the server.
            StreamingFromServerRequest request;
            request.set_num_bytes(num_bytes);

            // Container for the data we expect from the server.
            StreamingFromServerResponse response;

            // Context for the client. It could be used to convey extra information to
            // the server and/or tweak certain RPC behaviors.
            ClientContext context;
            auto start = high_resolution_clock::now();
            std::unique_ptr<ClientReader<StreamingFromServerResponse>> reader(
                stub_->StreamingFromServer(&context, request));
            size_t bytes_received = 0;
            while (reader->Read(&response)) {
                bytes_received += response.data().size();
            }
            Status status = reader->Finish();
            auto stop = high_resolution_clock::now();
            auto duration = duration_cast<milliseconds>(stop - start);
            if (status.ok()) {
                std::cout << "Received " << bytes_received << " bytes in " << duration.count() << " milliseconds." << std::endl;
                // std::cout << bytes_received / duration.count() << std::endl;
            } else {
                std::cout << "StreamingFromServer rpc failed." << std::endl;
            }
        }

    private:
        std::unique_ptr<ServerStreaming::Stub> stub_;
};

int main(int argc, char** argv) {
    ServerStreamingClient client(
        grpc::CreateChannel("localhost:50051",
                            grpc::InsecureChannelCredentials())
        );

    client.StreamingFromServer(640 * 1024 * 1024);

    return 0;
}