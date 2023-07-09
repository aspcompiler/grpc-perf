#include <fcntl.h>
#include <unistd.h>   // close
#include <sys/mman.h> // mmap
#include <filesystem>

#include <algorithm>
#include <condition_variable>
#include <iostream>
#include <thread>
#include <vector>

#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/ext/proto_server_reflection_plugin.h>

#include "server_streaming.grpc.pb.h"

using grpc::Server;
using grpc::ServerBuilder;
using grpc::ServerContext;
using grpc::ServerWriter;
using grpc::Status;

using server_streaming::StreamingFromServerRequest;
using server_streaming::StreamingFromServerResponse;
using server_streaming::ServerStreaming;

const uint32_t MAX_MESSAGE_SIZE = 1 << 20;

class ServerStreamingServiceImpl final : public ServerStreaming::Service
{

public:
    explicit ServerStreamingServiceImpl()
    {
    }


    Status StreamingFromServer(ServerContext *context, const StreamingFromServerRequest *request, ServerWriter<StreamingFromServerResponse> *writer) override
    {
        if (request->num_bytes() <= 0)
        {
            std::string err_msg = "StreamingFromServer requires specifying `num_bytes`";
            return Status(grpc::StatusCode::FAILED_PRECONDITION, err_msg);
        }

        uint64_t packet_index = 0;
        uint32_t bytes_remaining = request->num_bytes();

        while (bytes_remaining > 0)
        {
            if (context->IsCancelled())
            {
                break;
            }

            // handle protobuf size limits by only sending 1MB per message
            size_t bytes_to_send = std::min(bytes_remaining, MAX_MESSAGE_SIZE);
            bytes_remaining -= bytes_to_send;
            std::vector<uint64_t> data;
            data.reserve(bytes_to_send / sizeof(uint64_t));
            while (bytes_to_send > 0)
            {
                data.push_back(packet_index);
                bytes_to_send -= sizeof(uint64_t);
                packet_index++;
            }
            StreamingFromServerResponse resp;
            resp.set_data(data.data(), data.size() * sizeof(uint64_t));
            writer->Write(resp);
        }

        return Status::OK;
    }

private:
};

void RunServer()
{
    std::string server_address("0.0.0.0:50051");

    ServerStreamingServiceImpl service;

    grpc::EnableDefaultHealthCheckService(true);
    grpc::reflection::InitProtoReflectionServerBuilderPlugin();
    ServerBuilder builder;
    // Listen on the given address without any authentication mechanism.
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());
    // Register "service" as the instance through which we'll communicate with
    // clients. In this case it corresponds to an *synchronous* service.
    builder.RegisterService(&service);
    // Finally assemble the server.
    std::unique_ptr<Server> server(builder.BuildAndStart());
    std::cout << "Server listening on " << server_address << std::endl;

    // Wait for the server to shutdown. Note that some other thread must be
    // responsible for shutting down the server for this call to ever return.
    server->Wait();
}

int main(int argc, char **argv)
{
    RunServer();

    return 0;
}

