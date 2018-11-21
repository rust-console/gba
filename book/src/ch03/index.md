# Ch 3: Memory and Objects

Alright so we can do some basic "movement", but we left a big trail in the video
memory of everywhere we went. Most of the time that's not what we want at all.
If we want more hardware support we're going to have to use a new video mode. So
far we've only used Mode 3, but modes 4 and 5 are basically the same. Instead,
we'll switch focus to using a tiled graphical mode.

First we will go over the complete GBA memory mapping. Part of this is the
memory for tiled graphics, but also things like all those IO registers, where
our RAM is for scratch space, all that stuff. Even if we can't put all of them
to use at once, it's helpful to have an idea of what will be available in the
long run.

Tiled modes bring us three big new concepts that each have their own complexity:
tiles, backgrounds, and objects. Backgrounds and objects both use tiles, but the
background is for creating a very large static space that you can scroll around
the view within, and the objects are about having a few moving bits that appear
over the background. Careful use of backgrounds and objects is key to having the
best looking GBA game, so we won't even be able to cover it all in a single
chapter.

And, of course, since most games are pretty boring if they're totally static
we'll touch on the kinds of RNG implementations you might want to have on a GBA.
Most general purpose RNGs that you find are rather big compared to the amount of
memory we want to give them, and they often use a lot of `u64` operations, so
they end up much slower on a 32-bit machine like the GBA (you can lower 64-bit
ops to combinations of 32-bit ops, but that's quite a bit more work). We'll
cover a few RNG options that size down the RNG to a good size and a good speed
without trading away too much in terms of quality.

To top it all off, we'll make a simple "memory game" sort of thing. There's some
face down cards in a grid, you pick one to check, then you pick the other to
check, and then if they match the pair disappears.
