# Buttons

It's all well and good to just show a picture, even to show an animation, but if
we want a game we have to let the user interact with something.

## Key Input Register

* KEYINPUT, `0x400_0130`, `u16`, read only

This little `u16` stores the status of _all_ the buttons on the GBA, all at
once. There's only 10 of them, and we have 16 bits to work with, so that sounds
easy. However, there's a bit of a catch. The register follows a "low-active"
convention, where pressing a button _clears_ that bit until it's released.

```rust
const NO_BUTTONS_PRESSED: u16 = 0b0000_0011_1111_1111;
```

The buttons are, going up in order from the 0th bit:

* A
* B
* Select
* Start
* Right
* Left
* Up
* Down
* R
* L

Bits above that are not used. However, since the arrow left and right can never
be pressed at the same time, and the arrows up and down can never be pressed at
the same time, this register will never read as zero.

When programming, we usually are thinking of what buttons we want to have _be
pressed_ instead of buttons we want to have _not be pressed_. This means that we
need an inversion to happen somewhere along the line. The easiest moment of
inversion is immediately as you read in from the register and wrap the value up
in a newtype.

```rust
pub fn read_key_input() -> KeyInput {
  KeyInput(KEYINPUT.read() ^ 0b0000_0011_1111_1111)
}
```

Now the KeyInput you get can be checked for what buttons are pressed by checking
for a set bit like you'd do anywhere else.

```rust
impl KeyInput {
  pub fn a_pressed(self) -> bool {
    (self.0 & A_BIT) > 0
  }
}
```

Note that the current `KEYINPUT` value changes in real time as the user presses
or releases the buttons. To account for this, it's best to read the value just
once per game frame and then use that single value as if it was the input across
the whole frame. If you've worked with polling input before that should sound
totally normal, but if not just always remember to gather the input once per
frame and then use that value across the whole frame.

## Key Interrupt Control

* KEYCNT, `0x400_0132`, `u16`, read/write

This lets you control what keys will trigger a keypad interrupt. Of course, for
the actual interrupt to fire you also need to set the `IME` and IE` registers
properly. See the [Interrupts](05-interrupts.md) section for details there.

The main thing to know about this register is that the keys are in _the exact
same order_ as the key input order. However, with this register they use a
high-active convention instead (eg: the bit is active when the button should be
pressed as part of the interrupt).

In addition to simply having the bits for the buttons, bit 14 is a flag for
enabling keypad interrupts (in addition to the flag in the `IE` register), and
bit 15 decides how having more than one button works. If bit 15 is disabled,
it's an OR combination (eg: "press any key to continue"). If bit 15 is enabled
it's an AND combination (eg: "press A+B+Start+Select to reset").
