# Volatile

Before we focus on what the numbers mean, first let's ask ourselves: Why are we
doing _volatile_ writes? You've probably never used that keywords before at all.
What _is_ volatile anyway?

Well, the optimizer is pretty aggressive, and so it'll skip reads and writes
when it thinks can. Like if you write to a pointer once, and then again a moment
later, and it didn't see any other reads in between, it'll think that it can
just skip doing that first write since it'll get overwritten anyway. Sometimes
that's correct, but sometimes it's not.

Marking a read or write as _volatile_ tells the compiler that it really must do
that action, and in the exact order that we wrote it out. It says that there
might even be special hardware side effects going on that the compiler isn't
aware of. In this case, the write to the display control register sets a video
mode, and the writes to the Video RAM set pixels that will show up on the
screen.

Similar to "atomic" operations you might have heard about, all volatile
operations are enforced to happen in the exact order that you specify them, but
only relative to other volatile operations. So something like

```rust
c.write_volatile(5);
a += b;
d.write_volatile(7);
```

might end up changing `a` either before or after the change to `c` (since the
value of `a` doesn't affect the write to `c`), but the write to `d` will
_always_ happen after the write to `c`, even though the compiler doesn't see any
direct data dependency there.

If you ever go on to use volatile stuff on other platforms it's important to
note that volatile doesn't make things thread-safe, you still need atomic for
that. However, the GBA doesn't have threads, so we don't have to worry about
those sorts of thread safety concerns (there's interrupts, but that's another
matter).

## Volatile by default

Of course, writing out `volatile_write` every time is more than we wanna do.
There's clarity and then there's excessive. This is a chance to write our first
[newtype](https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html).
Basically a type that's got the exact same binary representation as some other
type, but new methods and trait implementations.

We want a `*mut T` that's volatile by default, and also when we offset it...
well the verdict is slightly unclear on how `offset` vs `wrapping_offset` work
when you're using pointers that you made up out of nowhere. I've asked the
experts and they genuinely weren't sure, so we'll make an `offset` method that
does a `wrapping_offset` just to be careful.

```rust
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<T>(pub *mut T);
impl<T> VolatilePtr<T> {
  pub unsafe fn read(&self) -> T {
    core::ptr::read_volatile(self.0)
  }
  pub unsafe fn write(&self, data: T) {
    core::ptr::write_volatile(self.0, data);
  }
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.wrapping_offset(count))
  }
}
```
