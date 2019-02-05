# Volatile Destination

TODO: update this when we can make more stuff `const`

## Volatile Memory

The compiler is an eager friend, so when it sees a read or a write that won't
have an effect, it eliminates that read or write. For example, if we write

```rust
let mut x = 5;
x = 7;
```

The compiler won't actually ever put 5 into `x`. It'll skip straight to putting
7 in `x`, because we never read from `x` when it's 5, so that's a safe change to
make. Normally, values are stored in RAM, which has no side effects when you
read and write from it. RAM is purely for keeping notes about values you'll need
later on.

However, what if we had a bit of hardware where we wanted to do a write and that
did something _other than_ keeping the value for us to look at later? As you saw
in the `hello_magic` example, we have to use a `write_volatile` operation.
Volatile means "just do it anyway". The compiler thinks that it's pointless, but
we know better, so we can force it to really do exactly what we say by using
`write_volatile` instead of `write`.

This is kinda error prone though, right? Because it's just a raw pointer, so we
might forget to use `write_volatile` at some point.

Instead, we want a type that's always going to use volatile reads and writes.
Also, we want a pointer type that lets our reads and writes to be as safe as
possible once we've unsafely constructed the initial value.

### Constructing The VolAddress Type

First, we want a type that stores a location within the address space. This can
be a pointer, or a `usize`, and we'll use a `usize` because that's easier to
work with in a `const` context (and we want to have `const` when we can get it).
We'll also have our type use `NonZeroUsize` instead of just `usize` so that
`Option<VolAddress<T>>` stays as a single machine word. This helps quite a bit
when we want to iterate over the addresses of a block of memory (such as
locations within the palette memory). Hardware is never at the null address
anyway. Also, if we had _just_ an address number then we wouldn't be able to
track what type the address is for. We need some
[PhantomData](https://doc.rust-lang.org/core/marker/struct.PhantomData.html),
and specifically we need the phantom data to be for `*mut T`:

* If we used `*const T` that'd have the wrong
  [variance](https://doc.rust-lang.org/nomicon/subtyping.html).
* If we used `&mut T` then that's fusing in the ideas of _lifetime_ and
  _exclusive access_ to our type. That's potentially important, but that's also
  an abstraction we'll build _on top of_ this `VolAddress` type if we need it.

One abstraction layer at a time, so we start with just a phantom pointer. This gives us a type that looks like this:

```rust
#[derive(Debug)]
#[repr(transparent)]
pub struct VolAddress<T> {
  address: NonZeroUsize,
  marker: PhantomData<*mut T>,
}
```

Now, because of how `derive` is specified, it derives traits _if the generic
parameter_ supports those traits. Since our type is like a pointer, the traits
it supports are distinct from whatever traits the target type supports. So we'll
provide those implementations manually.

```rust
impl<T> Clone for VolAddress<T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<T> Copy for VolAddress<T> {}
impl<T> PartialEq for VolAddress<T> {
  fn eq(&self, other: &Self) -> bool {
    self.address == other.address
  }
}
impl<T> Eq for VolAddress<T> {}
impl<T> PartialOrd for VolAddress<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.address.cmp(&other.address))
  }
}
impl<T> Ord for VolAddress<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.address.cmp(&other.address)
  }
}
```

Boilerplate junk, not interesting. There's a reason that you derive those traits
99% of the time in Rust.

### Constructing A VolAddress Value

Okay so here's the next core concept: If we unsafely _construct_ a
`VolAddress<T>`, then we can safely _use_ the value once it's been properly
created.

```rust
// you'll need these features enabled and a recent nightly
#![feature(const_int_wrapping)]
#![feature(min_const_unsafe_fn)]

impl<T> VolAddress<T> {
  pub const unsafe fn new_unchecked(address: usize) -> Self {
    VolAddress {
      address: NonZeroUsize::new_unchecked(address),
      marker: PhantomData,
    }
  }
  pub const unsafe fn cast<Z>(self) -> VolAddress<Z> {
    VolAddress {
      address: self.address,
      marker: PhantomData,
    }
  }
  pub unsafe fn offset(self, offset: isize) -> Self {
    VolAddress {
      address: NonZeroUsize::new_unchecked(self.address.get().wrapping_add(offset as usize * core::mem::size_of::<T>())),
      marker: PhantomData,
    }
  }
}
```

So what are the unsafety rules here?

* Non-null, obviously.
* Must be aligned for `T`
* Must always produce valid bit patterns for `T`
* Must not be part of the address space that Rust's stack or allocator will ever
  uses.

So, again using the `hello_magic` example, we had

```rust
(0x400_0000 as *mut u16).write_volatile(0x0403);
```

And instead we could declare

```rust
const MAGIC_LOCATION: VolAddress<u16> = unsafe { VolAddress::new(0x400_0000) };
```

### Using A VolAddress Value

Now that we've named the magic location, we want to write to it.

```rust
impl<T> VolAddress<T> {
  pub fn read(self) -> T
  where
    T: Copy,
  {
    unsafe { (self.address.get() as *mut T).read_volatile() }
  }
  pub unsafe fn read_non_copy(self) -> T {
    (self.address.get() as *mut T).read_volatile()
  }
  pub fn write(self, val: T) {
    unsafe { (self.address.get() as *mut T).write_volatile(val) }
  }
}
```

So if the type is `Copy` we can `read` it as much as we want. If, somehow, the
type isn't `Copy`, then it might be `Drop`, and that means if we read out a
value over and over we could cause the `drop` method to trigger UB. Since the
end user might really know what they're doing, we provide an unsafe backup
`read_non_copy`.

On the other hand, we can `write` to the location as much as we want. Even if
the type isn't `Copy`, _not running `Drop` is safe_, so a `write` is always
safe.

Now we can write to our magical location.

```rust
MAGIC_LOCATION.write(0x0403);
```

### VolAddress Iteration

We've already seen that sometimes we want to have a base address of some sort
and then offset from that location to another. What if we wanted to iterate over
_all the locations_. That's not particularly hard.

```rust
impl<T> VolAddress<T> {
  pub const unsafe fn iter_slots(self, slots: usize) -> VolAddressIter<T> {
    VolAddressIter { vol_address: self, slots }
  }
}

#[derive(Debug)]
pub struct VolAddressIter<T> {
  vol_address: VolAddress<T>,
  slots: usize,
}
impl<T> Clone for VolAddressIter<T> {
  fn clone(&self) -> Self {
    VolAddressIter {
      vol_address: self.vol_address,
      slots: self.slots,
    }
  }
}
impl<T> PartialEq for VolAddressIter<T> {
  fn eq(&self, other: &Self) -> bool {
    self.vol_address == other.vol_address && self.slots == other.slots
  }
}
impl<T> Eq for VolAddressIter<T> {}
impl<T> Iterator for VolAddressIter<T> {
  type Item = VolAddress<T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.slots > 0 {
      let out = self.vol_address;
      unsafe {
        self.slots -= 1;
        self.vol_address = self.vol_address.offset(1);
      }
      Some(out)
    } else {
      None
    }
  }
}
impl<T> FusedIterator for VolAddressIter<T> {}
```

### VolAddressBlock

Obviously, having a base address and a length exist separately is error prone.
There's a good reason for slices to keep their pointer and their length
together. We want something like that, which we'll call a "block" because
"array" and "slice" are already things in Rust.

```rust
#[derive(Debug)]
pub struct VolAddressBlock<T> {
  vol_address: VolAddress<T>,
  slots: usize,
}
impl<T> Clone for VolAddressBlock<T> {
  fn clone(&self) -> Self {
    VolAddressBlock {
      vol_address: self.vol_address,
      slots: self.slots,
    }
  }
}
impl<T> PartialEq for VolAddressBlock<T> {
  fn eq(&self, other: &Self) -> bool {
    self.vol_address == other.vol_address && self.slots == other.slots
  }
}
impl<T> Eq for VolAddressBlock<T> {}

impl<T> VolAddressBlock<T> {
  pub const unsafe fn new_unchecked(vol_address: VolAddress<T>, slots: usize) -> Self {
    VolAddressBlock { vol_address, slots }
  }
  pub const fn iter(self) -> VolAddressIter<T> {
    VolAddressIter {
      vol_address: self.vol_address,
      slots: self.slots,
    }
  }
  pub unsafe fn index_unchecked(self, slot: usize) -> VolAddress<T> {
    self.vol_address.offset(slot as isize)
  }
  pub fn index(self, slot: usize) -> VolAddress<T> {
    if slot < self.slots {
      unsafe { self.vol_address.offset(slot as isize) }
    } else {
      panic!("Index Requested: {} >= Bound: {}", slot, self.slots)
    }
  }
  pub fn get(self, slot: usize) -> Option<VolAddress<T>> {
    if slot < self.slots {
      unsafe { Some(self.vol_address.offset(slot as isize)) }
    } else {
      None
    }
  }
}
```

Now we can have something like:

```rust
const OTHER_MAGIC: VolAddressBlock<u16> = unsafe {
  VolAddressBlock::new_unchecked(
    VolAddress::new(0x600_0000),
    240 * 160
  )
};

OTHER_MAGIC.index(120 + 80 * 240).write_volatile(0x001F);
OTHER_MAGIC.index(136 + 80 * 240).write_volatile(0x03E0);
OTHER_MAGIC.index(120 + 96 * 240).write_volatile(0x7C00);
```

### Docs?

If you wanna see these types and methods with a full docs write up you should
check the GBA crate's source.

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

Note that if you use a linker script to include any ASM with your Rust program
(eg: the `crt0.s` file that we setup in the "Development Setup" section), all of
that ASM is "volatile" for these purposes. Volatile isn't actually a _hardware_
concept, it's just an LLVM concept, and the linker script runs after LLVM has
done its work.
