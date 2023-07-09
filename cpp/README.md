# C++ GRPC Servers and Clients

## Dependencies

### GRPC

We need to build and install the gRPC libraries. Follow the [cpp quick start](https://grpc.io/docs/languages/cpp/quickstart/)
to the end of [Try it](https://grpc.io/docs/languages/cpp/quickstart/#try-it) section.

If all goes smoothly, the gRPC libraries will be in $HOME/.local.

We can use the following variables when we build other projects:

```shell
$ export MY_INSTALL_DIR=$HOME/.local

$ cmake -DCMAKE_PREFIX_PATH=$MY_INSTALL_DIR ../..
```

For release build, add `-DCMAKE_BUILD_TYPE=Release` to `cmake`.

## Build Instructions

From the directory of each project

```bash
$ mkdir build
$ cd build
$ cmake ../
$ make -j
```

## Running

From each build directory, run the server first and then run the client

## Developing with VSCode

To get IntelliSense working and to debug, follow [Introductory Videos for C++](https://code.visualstudio.com/docs/cpp/introvideos-cpp).
This will create several files in the `.vscode` dir:
* c_cpp_properties.json: Settings for IntelliSense
* tasks.json and settings.json: Together they contain the CMake settings for Build.
* launch.json: Debug command and settings.
