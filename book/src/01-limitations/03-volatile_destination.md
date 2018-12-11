# Volatile Destination

There's a reasonable chance that you've never heard of `volatile` before, so
what's that? Well, it's a term that can be used in more than one context, but
basically it means "get your grubby mitts off my stuff you over-eager compiler".

## Volatile Memory

The first, and most common, form of volatile thing is volatile memory. Volatile
memory can change without your program changing it, usually because it's not a
location in RAM, but instead some special location that represents an actual
hardware device, or part of a hardware device perhaps. The compiler doesn't know
what's going on in this situation, but when the program is actually run and the
CPU gets an instruction to read or write from that location, instead of just
accessing some place in RAM like with normal memory, it accesses whatever bit of
hardware and does _something_. The details of that something depend on the
hardware, but what's important is that we need to actually, definitely execute
that read or write instruction.

This is not how normal memory works. Normally when the compiler
sees us write values into variables and read values from variables, it's free to
optimize those expressions and eliminate some of the reads and writes if it can,
and generally try to save us time. Maybe it even knows some stuff about the data
dependencies in our expressions and so it does some of the reads or writes out
of order from what the source says, because the compiler knows that it won't
actually make a difference to the operation of the program. A good and helpful
friend, that compiler.

Volatile memory works almost the opposite way. With volatile memory we
need the compiler to _definitely_ emit an instruction to do a read or write and
they need to happen _exactly_ in the order that we say to do it. Each volatile
read or write might have any sort of side effect that the compiler
doesn't know about, and it shouldn't try to be clever about the optimization. Just do what we
say, please.

In Rust, we don't mark volatile things as being a separate type of thing,
instead we use normal raw pointers and then call the
[read_volatile](https://doc.rust-lang.org/core/ptr/fn.read_volatile.html) and
[write_volatile](https://doc.rust-lang.org/core/ptr/fn.write_volatile.html)
functions (also available as methods, if you like), which then delegate to the
LLVM
[volatile_load](https://doc.rust-lang.org/core/intrinsics/fn.volatile_load.html)
and
[volatile_store](https://doc.rust-lang.org/core/intrinsics/fn.volatile_store.html)
intrinsics. In C and C++ you can tag a pointer as being volatile and then any
normal read and write with it becomes the volatile version, but in Rust we have
to remember to use the correct alternate function instead.

I'm told by the experts that this makes for a cleaner and saner design from a
_language design_ perspective, but it really kinda screws us when doing low
level code. References, both mutable and shared, aren't volatile, so they
compile into normal reads and writes. This means we can't do anything we'd
normally do in Rust that utilizes references of any kind. Volatile blocks of
memory can't use normal `.iter()` or `.iter_mut()` based iteration (which give
`&T` or `&mut T`), and they also can't use normal `Index` and `IndexMut` sugar
like `a + x[i]` or `x[i] = 7`.

Unlike with normal raw pointers, this pain point never goes away. There's no way
to abstract over the difference with Rust as it exists now, you'd need to
actually adjust the core language by adding an additional pointer type (`*vol
T`) and possibly a reference type to go with it (`&vol T`) to get the right
semantics. And then you'd need an `IndexVol` trait, and you'd need
`.iter_vol()`, and so on for every other little thing. It would be a lot of
work, and the Rust developers just aren't interested in doing all that for such
a limited portion of their user population. We'll just have to deal with not
having any syntax sugar.

But no syntax sugar doesn't mean we can't at least do a little work for
ourselves. Enter the `VolatilePtr<T>` type, which is a newtype over a `*mut T`:

```rust
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<T>(*mut T);
```

Obviously we'll need some methods go with it. The basic operations are reading
and writing of course:

```rust
impl<T> VolatilePtr<T> {
  /// Performs a `read_volatile`.
  pub unsafe fn read(self) -> T {
    self.0.read_volatile()
  }

  /// Performs a `write_volatile`.
  pub unsafe fn write(self, data: T) {
    self.0.write_volatile(data);
  }
```

And we want a way to jump around when we do have volatile memory that's in
blocks. For this there's both
[offset](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset) and
[wrapping_offset](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset).
The difference is that `offset` optimizes better, but also it can be Undefined
Behavior if the result is not "in bounds or one byte past the end of the same
allocated object". I asked [ubsan](https://github.com/ubsan) (who is the expert
that you should always listen to on matters like this) what that means for us,
and the answer was that you _can_ use an `offset` in statically memory mapped
situations like this as long as you don't use it to jump to the address of
something that Rust itself allocated at some point. Unfortunately, the downside
to using `offset` instead of `wrapping_offset` is that with `offset`, it's
Undefined Behavior _simply to calculate the out of bounds result_, and with
`wrapping_offset` it's not Undefined Behavior until you _use_ the out of bounds
result.

```rust
  /// Performs a normal `offset`.
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.offset(count))
  }
```

Now, one thing of note is that doing the `offset` isn't `const`. If we wanted to have a `const` function for
finding the correct spot within a volatile block of memory we'd have to do all the math using `usize` values
and then cast that value into being a pointer once we were done. In the future Rust might be
able to do it without a goofy work around, but `const` is quite limited at the moment.
It'd look something like this:

```rust
const fn address_index<T>(address: usize, index: usize) -> usize {
    address + (index * std::mem::size_of::<T>())
}
```

We will sometimes want to be able to cast a `VolatilePtr` between pointer types. Since we
won't be able to do that with `as`, we'll have to write a method for that:

```rust
  /// Performs a cast into some new pointer type.
  pub fn cast<Z>(self) -> VolatilePtr<Z> {
    VolatilePtr(self.0 as *mut Z)
  }
```

TODO: iterator stuff

Also, just as a little bonus that we probably won't use, we could enable our new pointer type
to be formatted as a pointer value.

```rust
impl<T> core::fmt::Pointer for VolatilePtr<T> {
  /// Formats exactly like the inner `*mut T`.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:p}", self.0)
  }
}
```

## Volatile ASM
