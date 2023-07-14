from concurrent import futures
import logging

import grpc
import server_streaming_pb2
import server_streaming_pb2_grpc

MAX_MESSAGE_SIZE = 1 << 20

class ServerStreamingServicer(server_streaming_pb2_grpc.ServerStreamingServicer):
    """Provides methods that implement functionality of server_streaming server."""

    def StreamingFromServer(self, request, context):
        bytes_remaining = request.num_bytes
        while bytes_remaining > 0:
            bytes_to_send = min(MAX_MESSAGE_SIZE, bytes_remaining)
            yield server_streaming_pb2.StreamingFromServerResponse(
                data=b"\x00" * bytes_to_send
            )
            bytes_remaining -= bytes_to_send

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    server_streaming_pb2_grpc.add_ServerStreamingServicer_to_server(
        ServerStreamingServicer(), server
    )
    server.add_insecure_port("[::]:50051")
    server.start()
    server.wait_for_termination()


if __name__ == "__main__":
    logging.basicConfig()
    serve()