# variable int compression

compression of single integers to variable sized bytes.

## Implementation

This implementation uses 7 bits of each byte to store data
and the last bit is used to store wether or not the integer continues afterwards.
