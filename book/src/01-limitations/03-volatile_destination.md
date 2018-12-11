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

### VolatilePtr

No syntax sugar doesn't mean we can't at least make things a little easier for
ourselves. Enter the `VolatilePtr<T>` type, which is a newtype over a `*mut T`.
One of those "manual" newtypes I mentioned where we can't use our nice macro.

```rust
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<T>(pub *mut T);
```

Obviously we want to be able to read and write:

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
blocks. This is where we can get ourselves into some trouble if we're not
careful. We have to decide between
[offset](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset) and
[wrapping_offset](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset).
The difference is that `offset` optimizes better, but also it can be Undefined
Behavior if the result is not "in bounds or one byte past the end of the same
allocated object". I asked [ubsan](https://github.com/ubsan) (who is the expert
that you should always listen to on matters like this) what that means exactly
when memory mapped hardware is involved (since we never allocated anything), and
the answer was that you _can_ use an `offset` in statically memory mapped
situations like this as long as you don't use it to jump to the address of
something that Rust itself allocated at some point. Cool, we all like being able
to use the one that optimizes better. Unfortunately, the downside to using
`offset` instead of `wrapping_offset` is that with `offset`, it's Undefined
Behavior _simply to calculate the out of bounds result_ (with `wrapping_offset`
it's not Undefined Behavior until you _use_ the out of bounds result). We'll
have to be quite careful when we're using `offset`.

```rust
  /// Performs a normal `offset`.
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.offset(count))
  }
```

Now, one thing of note is that doing the `offset` isn't `const`. The math for it
is something that's possible to do in a `const` way of course, but Rust
basically doesn't allow you to fiddle raw pointers much during `const` right
now. Maybe in the future that will improve.

If we did want to have a `const` function for finding the correct address within
a volatile block of memory we'd have to do all the math using `usize` values,
and then cast that value into being a pointer once we were done. It'd look
something like this:

```rust
const fn address_index<T>(address: usize, index: usize) -> usize {
  address + (index * std::mem::size_of::<T>())
}
```

But, back to methods for `VolatilePtr`, well we sometimes want to be able to
cast a `VolatilePtr` between pointer types. Since we won't be able to do that
with `as`, we'll have to write a method for it:

```rust
  /// Performs a cast into some new pointer type.
  pub fn cast<Z>(self) -> VolatilePtr<Z> {
    VolatilePtr(self.0 as *mut Z)
  }
```

### Volatile Iterating

How about that `Iterator` stuff I said we'd be missing? We can actually make
_an_ Iterator available, it's just not the normal "iterate by shared reference
or unique reference" Iterator. Instead, it's more like a "throw out a series of
`VolatilePtr` values" style Iterator. Other than that small difference it's
totally normal, and we'll be able to use map and skip and take and all those
neat methods.

So how do we make this thing we need? First we check out the [Implementing
Iterator](https://doc.rust-lang.org/core/iter/index.html#implementing-iterator)
section in the core documentation. It says we need a struct for holding the
iterator state. Right-o, probably something like this:

```rust
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VolatilePtrIter<T> {
  vol_ptr: VolatilePtr<T>,
  slots: usize,
}
```

And then we just implement
[core::iter::Iterator](https://doc.rust-lang.org/core/iter/trait.Iterator.html)
on that struct. Wow, that's quite the trait though! Don't worry, we only need to
implement two small things and then the rest of it comes free as a bunch of
default methods.

So, the code that we _want_ to write looks like this:

```rust
impl<T> Iterator for VolatilePtrIter<T> {
  type Item = VolatilePtr<T>;

  fn next(&mut self) -> Option<VolatilePtr<T>> {
    if self.slots > 0 {
      let out = Some(self.vol_ptr);
      self.slots -= 1;
      self.vol_ptr = unsafe { self.vol_ptr.offset(1) };
      out
    } else {
      None
    }
  }
}
```

Except we _can't_ write that code. What? The problem is that we used
`derive(Clone, Copy` on `VolatilePtr`. Because of a quirk in how `derive` works,
this makes `VolatilePtr<T>` will only be `Copy` if the `T` is `Copy`, _even
though the pointer itself is always `Copy` regardless of what it points to_.
Ugh, terrible. We've got three basic ways to handle this:

* Make the `Iterator` implementation be for `<T:Clone>`, and then hope that we
  always have types that are `Clone`.
* Hand implement every trait we want `VolatilePtr` (and `VolatilePtrIter`) to
  have so that we can override the fact that `derive` is basically broken in
  this case.
* Make `VolatilePtr` store a `usize` value instead of a pointer, and then cast
  it to `*mut T` when we actually need to read and write. This would require us
  to also store a `PhantomData<T>` so that the type of the address is tracked
  properly, which would make it a lot more verbose to construct a `VolatilePtr`
  value.

None of those options are particularly appealing. I guess we'll do the first one
because it's the least amount of up front trouble, and I don't _think_ we'll
need to be iterating non-Clone values. All we do to pick that option is add the
bound to the very start of the `impl` block, where we introduce the `T`:

```rust
impl<T: Clone> Iterator for VolatilePtrIter<T> {
  type Item = VolatilePtr<T>;

  fn next(&mut self) -> Option<VolatilePtr<T>> {
    if self.slots > 0 {
      let out = Some(self.vol_ptr.clone());
      self.slots -= 1;
      self.vol_ptr = unsafe { self.vol_ptr.clone().offset(1) };
      out
    } else {
      None
    }
  }
}
```

What's going on here? Okay so our iterator has a number of slots that it'll go
over, and then when it's out of slots it starts producing `None` forever. That's
actually pretty simple. We're also masking some unsafety too. In this case,
we'll rely on the person who made the `VolatilePtrIter` to have selected the
correct number of slots. This gives us a new method for `VolatilePtr`:

```rust
  pub unsafe fn iter_slots(self, slots: usize) -> VolatilePtrIter<T> {
    VolatilePtrIter {
      vol_ptr: self,
      slots,
    }
  }
```

With this design, making the `VolatilePtrIter` at the start is `unsafe` (we have
to trust the caller that the right number of slots exists), and then using it
after that is totally safe (if the right number of slots was given we'll never
screw up our end of it).

### VolatilePtr Formatting

Also, just as a little bonus that we probably won't use, we could enable our new
pointer type to be formatted as a pointer value.

```rust
impl<T> core::fmt::Pointer for VolatilePtr<T> {
  /// Formats exactly like the inner `*mut T`.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:p}", self.0)
  }
}
```

Neat!

### VolatilePtr Complete

That was a lot of small code blocks, let's look at it all put together:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VolatilePtr<T>(pub *mut T);
impl<T> VolatilePtr<T> {
  pub unsafe fn read(self) -> T {
    self.0.read_volatile()
  }
  pub unsafe fn write(self, data: T) {
    self.0.write_volatile(data);
  }
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.offset(count))
  }
  pub fn cast<Z>(self) -> VolatilePtr<Z> {
    VolatilePtr(self.0 as *mut Z)
  }
  pub unsafe fn iter_slots(self, slots: usize) -> VolatilePtrIter<T> {
    VolatilePtrIter {
      vol_ptr: self,
      slots,
    }
  }
}
impl<T> core::fmt::Pointer for VolatilePtr<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:p}", self.0)
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VolatilePtrIter<T> {
  vol_ptr: VolatilePtr<T>,
  slots: usize,
}
impl<T: Clone> Iterator for VolatilePtrIter<T> {
  type Item = VolatilePtr<T>;
  fn next(&mut self) -> Option<VolatilePtr<T>> {
    if self.slots > 0 {
      let out = Some(self.vol_ptr.clone());
      self.slots -= 1;
      self.vol_ptr = unsafe { self.vol_ptr.clone().offset(1) };
      out
    } else {
      None
    }
  }
}
```

## Volatile ASM

In addition to some memory locations being volatile, it's also possible for
inline assembly to be declared volatile. This is basically the same idea, "hey
just do what I'm telling you, don't get smart about it".

Normally when you have some `asm!` it's basically treated like a function,
there's inputs and outputs and the compiler will try to optimize it so that if
you don't actually use the outputs it won't bother with doing those
instructions. However, `asm!` is basically a pure black box, so the compiler
doesn't know what's happening inside at all, and it can't see if there's any
important side effects going on.

An example of an important side effect that doesn't have output values would be
putting the CPU into a low power state while we want for the next VBlank. This
lets us save quite a bit of battery power. It requires some setup to be done
safely (otherwise the GBA won't ever actually wake back up from the low power
state), but the `asm!` you use once you're ready is just a single instruction
with no return value. The compiler can't tell what's going on, so you just have
to say "do it anyway".
