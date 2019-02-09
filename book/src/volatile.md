# Volatile

I know that you just got your first program running and you're probably excited
to learn more about GBA stuff, but first we have to cover a subject that's not
quite GBA specific.

In the `hello_magic.rs` file we had these lines

```rust
    (0x600_0000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x600_0000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x600_0000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
```

You've probably seen or heard of the
[write](https://doc.rust-lang.org/core/ptr/fn.write.html) function before, but
you'd be excused if you've never heard of its cousin function,
[write_volatile](https://doc.rust-lang.org/core/ptr/fn.write_volatile.html).

What's the difference? Well, when the compiler sees normal reads and writes, it
assumes that those go into plain old memory locations. CPU registers, RAM,
wherever it is that the value's being placed. The compiler assumes that it's
safe to optimize away some of the reads and writes, or maybe issue the reads and
writes in a different order from what you wrote. Normally this is okay, and it's
exactly what we want the compiler to be doing, quietly making things faster for us.

However, some of the time we access values from parts of memory where it's
important that each access happen, and in the exact order that we say. In our
`hello_magic.rs` example, we're writing directly into the video memory of the
display. The compiler sees that the rest of the Rust program never read out of
those locations, so it might think "oh, we can skip those writes, they're
pointless". It doesn't know that we're having a side effect besides just storing
some value at an address.

By declaring a particular read or write to be `volatile` then we can force the
compiler to issue that access. Further, we're guaranteed that all `volatile`
access will happen in exactly the order it appears in the program relative to
other `volatile` access. However, non-volatile access can still be re-ordered
relative to a volatile access. In other words, for parts of the memory that are
volatile, we must _always_ use a volatile read or write for our program to
perform properly.

For exactly this reason, we've got the [voladdress](https://docs.rs/voladdress/)
crate. It used to be part of the GBA crate, but it became big enough to break
out into a stand alone crate. It doesn't even do too much, it just makes it a
lot less error prone to accidentally forget to use volatile with our memory
mapped addresses. We just call `read` and `write` on any `VolAddress` that we
happen to see and the right thing will happen.
