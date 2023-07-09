import asyncio
import logging
import time

import grpc
import server_streaming_pb2
import server_streaming_pb2_grpc


async def streaming_from_server(
    stub: server_streaming_pb2_grpc.ServerStreamingStub,
) -> None:
    # The following two coroutines will be wrapped in a Future object
    # and scheduled in the event loop so that they can run concurrently
    bytes_received = 0
    tic = time.perf_counter()
    responses = stub.StreamingFromServer(
        server_streaming_pb2.StreamingFromServerRequest(num_bytes=640 * 1024 * 1024)
    )
    async for response in responses:
        bytes_received += len(response.data)
    toc = time.perf_counter()
    print(f"{bytes_received / (toc - tic):0.2f}")


async def run() -> None:
    async with grpc.aio.insecure_channel("localhost:50051") as channel:
        stub = server_streaming_pb2_grpc.ServerStreamingStub(channel)
        await streaming_from_server(stub)


if __name__ == "__main__":
    logging.basicConfig()
    asyncio.run(run())
