# gRPC benchmark

This is a mini benchmark for the user cases that I work with. It primary uses unitary-streaming call
to retrieve a large a mount the data.

The following is the results so far:

| client \ server | cpp sync  | rust tonic | python     |
| --------------- | --------- | ---------- | ---------- |
| cpp sync        | 1264 MB/s |  831 MB/s  | 1150 MB/s  |
| python          |  641 MB/s |  494 MS/s  |  608 MB/s  |
| rust tonic      | 1434 MB/s |  536 MB/s  | 1531 MB/s  |