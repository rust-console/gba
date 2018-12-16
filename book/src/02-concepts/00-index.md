# Broad Concepts

The GameBoy Advance sits in a middle place between the chthonic game consoles of
the ancient past and the "small PC in a funny case" consoles of the modern age.

On the one hand, yeah, you're gonna find a few strange conventions as you learn
all the ropes.

On the other, at least we're writing in Rust at all, and not having to do all
the assembly by hand.

This chapter for "concepts" has a section for each part of the GBA's hardware
memory map, going by increasing order of base address value. The sections try to
explain as much as possible while sticking to just the concerns you might have
regarding that part of the memory map.

For an assessment of how to wrangle all three parts of the video system (PALRAM,
VRAM, and OAM), along with the correct IO registers, into something that shows a
picture, you'll want the Video chapter.

Similarly, the "IO Registers" part of the GBA actually controls how you interact
with every single bit of hardware connected to the GBA. A full description of
everything is obviously too much for just one section of the book. Instead you
get an overview of general IO register rules and advice. Each particular
register is described in the appropriate sections of either the Video or
Non-Video chapters.
