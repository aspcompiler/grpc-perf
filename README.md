# gRPC benchmark

This is a mini benchmark for the user cases that I work with. It primary uses unitary-streaming call
to retrieve a large a mount the data.

The following is the results running on my MacBook Pro (15-inch, 2016) with 2.6 GHz 6-Core Intel 
Core i7 and 16 GB memory running macOS Ventura 13.4.

| client \ server | cpp sync  | rust tonic | python     |
| --------------- | --------- | ---------- | ---------- |
| cpp sync        | 1264 MB/s | 1165 MB/s  | 1150 MB/s  |
| python          |  641 MB/s |  565 MS/s  |  608 MB/s  |
| rust tonic      | 1434 MB/s |  705 MB/s  | 1531 MB/s  |