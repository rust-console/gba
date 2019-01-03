# Non-Video

Besides video effects the GBA still has an okay amount of stuff going on.

Obviously you'll want to know how to read the user's button inputs. That can
almost go without saying, except that I said it.

Each other part can be handled in about any order you like.

Using interrupts is perhaps one of the hardest things for us as Rust programmers
due to quirks in our compilation process. Our code all gets compiled to 16-bit
THUMB instructions, and we don't have a way to mark a function to be compiled
using 32-bit ASM instructions instead. However, an interrupt handler _must_ be
written in 32-bit ASM instructions for it to work. That means that we have to
write our interrupt handler in 32-bit ASM by hand. We'll do it, but I don't
think we'll be too happy about it.

The Link Cable related stuff is also probably a little harder to test than
anything else. Just because link cable emulation isn't always the best, and or
you need two GBAs with two flash carts and the cable for hardware testing.
Still, we'll try to go over it eventually.
