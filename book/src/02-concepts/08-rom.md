# Game Pak ROM / Flash ROM (ROM)

* **Address Span (Wait State 0):** `0x800_0000` to `0x9FF_FFFF`
* **Address Span (Wait State 1):** `0xA00_0000` to `0xBFF_FFFF`
* **Address Span (Wait State 2):** `0xC00_0000` to `0xDFF_FFFF`

The game's ROM data is a single set of data that's up to 32 megabytes in size.
However, that data is mirrored to three different locations in the address
space. Depending on which part of the address space you use, it can affect the
memory timings involved.

TODO: describe `WAITCNT` here, we won't get a better chance at it.

TODO: discuss THUMB vs ARM code and why THUMB is so much faster (because ROM is a 16-bit bus)
