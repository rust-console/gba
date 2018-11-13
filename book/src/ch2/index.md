# Ch 2: User Input

It's all well and good to draw three pixels, but they don't do anything yet. We
want them to do something, and for that we need to get some input from the user.

The GBA, as I'm sure you know, has an arrow pad, A and B, L and R, Start and
Select. That's a little more than the NES/GB/CGB had, and a little less than the
SNES had. As you can guess, we get key state info from an IO register.

Also, we will need a way to keep the program from running "too fast". On a
modern computer or console you do this with vsync info from the GPU and Monitor,
and on the GBA we'll be using vsync info from an IO register that tracks what
the display hardware is doing.

As a way to apply our knowledge We'll make a simply "light cycle" game where
your dot leaves a trail behind them and you die if you go off the screen or if
you touch your own trail. We just make a copy of `hello2.rs` named
`light_cycle.rs` and then fill it in as we go through the chapter. Normally you
might not place the entire program into a single source file, particularly as it
grows over time, but since these are small examples it's much better to have
them be completely self contained than it is to have them be "properly
organized" for the long term.
