from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class StreamingFromServerRequest(_message.Message):
    __slots__ = ["num_bytes"]
    NUM_BYTES_FIELD_NUMBER: _ClassVar[int]
    num_bytes: int
    def __init__(self, num_bytes: _Optional[int] = ...) -> None: ...

class StreamingFromServerResponse(_message.Message):
    __slots__ = ["data"]
    DATA_FIELD_NUMBER: _ClassVar[int]
    data: bytes
    def __init__(self, data: _Optional[bytes] = ...) -> None: ...
