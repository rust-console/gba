# Work RAM

## External Work RAM (EWRAM)

* **Address Span:** `0x2000000` to `0x203FFFF` (256k)

This is a big pile of space, the use of which is up to each game. However, the
external work ram has only a 16-bit bus (if you read/write a 32-bit value it
silently breaks it up into two 16-bit operations) and also 2 wait cycles (extra
CPU cycles that you have to expend _per 16-bit bus use_).

It's most helpful to think of EWRAM as slower, distant memory, similar to the
"heap" in a normal application. You can take the time to go store something
within EWRAM, or to load it out of EWRAM, but if you've got several operations
to do in a row and you're worried about time you should pull that value into
local memory, work on your local copy, and then push it back out to EWRAM.

## Internal Work RAM (IWRAM)

* **Address Span:** `0x3000000` to `0x3007FFF` (32k)

This is a smaller pile of space, but it has a 32-bit bus and no wait.

By default, `0x3007F00` to `0x3007FFF` is reserved for interrupt and BIOS use.
The rest of it is mostly up to you. The user's stack space starts at `0x3007F00`
and proceeds _down_ from there. For best results you should probably start at
`0x3000000` and then go upwards. Under normal use it's unlikely that the two
memory regions will crash into each other.
