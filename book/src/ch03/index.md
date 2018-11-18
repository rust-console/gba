# Ch 3: Memory and Objects

Alright so we can do some basic "movement", but we left a big trail in the video
memory of everywhere we went. Most of the time that's not what we want at all.
If we want more hardware support we're going to have to use a new video mode. So
far we've only used Mode 3, but modes 4 and 5 are basically the same. Instead,
we'll focus on using a tiled mode. The tiled modes take less time to arrange,
which means more time for game computations. If your game has much complexity at
all, you'll naturally want to use a tiled mode to display it.

Tiled modes bring us two big new concepts that each have their own complexity:
backgrounds and objects. They share some concepts, but fundamentally the
background is for creating a very large static space that you can scroll around
the view within, and the objects are about having a few moving bits that appear
over the background. Careful use of backgrounds and objects is key to having the
best looking GBA game, so we won't even be able to cover it all in a single
chapter.

Of course, once we're all set drawing the objects and backgrounds, we'll want
them to start keeping track of what's where, and maybe store info on stuff
that's off screen. That means it's finally time to go over the GBA's memory
layout so that we know where we have scratch space to work with.

And, of course, since most games are pretty boring if they're totally static
we'll touch on the kinds of RNG implementations you might want to have on a GBA.
Most general purpose RNGs that you find are rather big compared to the amount of
memory we want to give them, and they often use a lot of `u64` operations, so
they end up much slower on a 32-bit machine like the GBA (you can lower 64-bit
ops to combinations of 32-bit ops, but that's quite a bit more work). We'll
cover a few RNG options that size down the RNG to a good size and a good speed
without trading away too much in terms of quality.

To top it all off, we'll make a simple memory game sort of thing.
