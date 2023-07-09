# gRPC benchmark

This is a mini benchmark for the user cases that I work with. It primary uses unitary-streaming call
to retrieve a large a mount the data.

The following is the results so far:

| client \ server | cpp sync  |
| --------------- | --------- |
| cpp             | 1264 MB/s |
| python          | 641 MB/s  |
