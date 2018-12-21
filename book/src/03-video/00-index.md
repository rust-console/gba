# Video

GBA Video starts with an IO register called the "Display Control Register", and
then spirals out from there. You generally have to use Palette RAM (PALRAM),
Video RAM (VRAM), Object Attribute Memory (OAM), as well as any number of other
IO registers.

They all have to work together just right, and there's a lot going on when you
first try doing it, so try to take it very slowly as you're learning each step.
