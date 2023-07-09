cmake_minimum_required(VERSION 3.8)

project(ServerStreaming C CXX)

include(../cmake/common.cmake)

# Proto file
get_filename_component(ss_proto "../../protos/server_streaming.proto" ABSOLUTE)
get_filename_component(ss_proto_path "${ss_proto}" PATH)

# Generated protobuf sources
set(ss_proto_srcs "${CMAKE_CURRENT_BINARY_DIR}/server_streaming.pb.cc")
set(ss_proto_hdrs "${CMAKE_CURRENT_BINARY_DIR}/server_streaming.pb.h")
set(ss_grpc_srcs "${CMAKE_CURRENT_BINARY_DIR}/server_streaming.grpc.pb.cc")
set(ss_grpc_hdrs "${CMAKE_CURRENT_BINARY_DIR}/server_streaming.grpc.pb.h")
add_custom_command(
      OUTPUT "${ss_proto_srcs}" "${ss_proto_hdrs}" "${ss_grpc_srcs}" "${ss_grpc_hdrs}"
      COMMAND ${_PROTOBUF_PROTOC}
      ARGS --grpc_out "${CMAKE_CURRENT_BINARY_DIR}"
        --cpp_out "${CMAKE_CURRENT_BINARY_DIR}"
        -I "${ss_proto_path}"
        --plugin=protoc-gen-grpc="${_GRPC_CPP_PLUGIN_EXECUTABLE}"
        "${ss_proto}"
      DEPENDS "${ss_proto}")

# Include generated *.pb.h files
include_directories("${CMAKE_CURRENT_BINARY_DIR}")

# ss_grpc_proto
add_library(ss_grpc_proto
  ${ss_grpc_srcs}
  ${ss_grpc_hdrs}
  ${ss_proto_srcs}
  ${ss_proto_hdrs})
target_link_libraries(ss_grpc_proto
  ${_REFLECTION}
  ${_GRPC_GRPCPP}
  ${_PROTOBUF_LIBPROTOBUF})

# Targets server_streaming_(client|server)
foreach(_target 
    server_streaming_client 
    server_streaming_server)
    add_executable(${_target} ${_target}.cpp)
    target_link_libraries(${_target}
        ss_grpc_proto
        ${_REFLECTION}
        ${_GRPC_GRPCPP}
        ${_PROTOBUF_LIBPROTOBUF})   
endforeach()
