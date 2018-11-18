# Ch 3: Memory and Objects

Alright so we can do some basic "movement", but we left a big trail in the video
memory of where we went. Most of the time that's not what we want at all. If we
want to draw something over top of our background without trashing the
background memory that's an "object" (but not in the "Object Oriented" sense).
You might recall that objects have their own layer that you can enable in the
display control register.

Of course, once we're drawing these objects we'll want some scratch space to
work with them a bit, so we'll finally go over the GBA's full memory layout.

And since most games are pretty boring without an RNG, we'll cover the kinds of
RNG that you might want to include in a GBA game.

Then we'll do something or other that includes moving things and and RNG...
which is pretty much any game at all.
