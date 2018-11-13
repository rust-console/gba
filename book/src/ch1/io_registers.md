# IO Registers

The GBA has a large number of **IO Registers** (not to be confused with CPU
registers). These are special memory locations from `0x04000000` to
`0x040003FE`. GBATEK has a [full
list](http://problemkaputt.de/gbatek.htm#gbaiomap), but we only need to learn
about a few of them at a time as we go, so don't be worried.

The important facts to know about IO Registers are these:

* Each has their own specific size. Most are `u16`, but some are `u32`.
* All of them must be accessed in a `volatile` style.
* Each register is specifically readable or writable or both. Actually, with
  some registers there are even individual bits that are read-only or
  write-only.
  * If you write to a read-only position, those writes are simply ignored. This
    mostly matters if a writable register contains a read-only bit (such as the
    Display Control, next section).
  * If you read from a write-only position, you get back values that are
    [basically
    nonsense](http://problemkaputt.de/gbatek.htm#gbaunpredictablethings). There
    aren't really any registers that mix writable bits with read only bits, so
    you're basically safe here. The only (mild) concern is that when you write a
    value into a write-only register you need to keep track of what you wrote
    somewhere else if you want to know what you wrote (such to adjust an offset
    value by +1, or whatever).
  * You can always check GBATEK to be sure, but if I don't mention it then a bit
    is probably both read and write.
* Some registers have invalid bit patterns. For example, the lowest three bits
  of the Display Control register can't legally be set to the values 6 or 7.

When talking about bit positions, the numbers are _zero indexed_ just like an
array index is.
