# The Key Input Register

The Key Input Register is our next IO register. Its shorthand name is
[KEYINPUT](http://problemkaputt.de/gbatek.htm#gbakeypadinput) and it's a `u16`
at `0x4000130`. The entire register is obviously read only, you can't tell the
GBA what buttons are pressed.

Each button is exactly one bit:

| Bit | Button |
|:---:|:------:|
|  0  | A |
|  1  | B |
|  2  | Select |
|  3  | Start |
|  4  | Right |
|  5  | Left |
|  6  | Up |
|  7  | Down |
|  8  | R |
|  9  | L |

The higher bits above are not used at all.

Similar to other old hardware devices, the convention here is that a button's
bit is **clear when pressed, active when released**. In other words, when the
user is not touching the device at all the KEYINPUT value will read
`0b0000_0011_1111_1111`. There's similar values for when the user is pressing as
many buttons as possible, but since the left/right and up/down keys are on an
arrow pad the value can never be 0 since you can't ever press every single key
at once.

When dealing with key input, the register always shows the exact key values at
any moment you read it. Obviously that's what it should do, but what it means to
you as a programmer is that you should usually gather input once at the top of a
game frame and then use that single input poll as the input values across the
whole game frame.

Of course, you might want to know if a user's key state changed from frame to
frame. That's fairly easy too: We just store the last frame keys as well as the
current frame keys (it's only a `u16`) and then we can xor the two values.
Anything that shows up in the xor result is a key that changed. If it's changed
and it's now down, that means it was pushed this frame. If it's changed and it's
now up, that means it was released this frame.

The other major thing you might frequently want is to know "which way" the arrow
pad is pointing: Up/Down/None and Left/Right/None. Sounds like an enum to me.
Except that often time we'll have situations where the direction just needs to
be multiplied by a speed and applied as a delta to a position. We want to
support that as well as we can too.

## Key Input Code

Let's get down to some code. First we want to make a way to read the address as
a `u16` and then wrap that in our newtype which will implement methods for
reading and writing the key bits.

```rust
pub const KEYINPUT: VolatilePtr<u16> = VolatilePtr(0x400_0130 as *mut u16);

/// A newtype over the key input state of the GBA.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyInputSetting(u16);

pub fn key_input() -> KeyInputSetting {
  unsafe { KeyInputSetting(KEYINPUT.read()) }
}
```

Now we want a way to check if a key is _being pressed_, since that's normally
how we think of things as a game designer and even as a player. That is, usually
you'd say "if you press A, then X happens" instead of "if you don't press A,
then X does not happen".

Normally we'd pick a constant for the bit we want, `&` it with our value, and
then check for `val != 0`. Since the bit we're looking for is `0` in the "true"
state we still pick the same constant and we still do the `&`, but we test with
`== 0`. Practically the same, right? Well, since I'm asking a rhetorical
question like that you can probably already guess that it's not the same. I was
shocked to learn this too.

All we have to do is ask our good friend
[Godbolt](https://rust.godbolt.org/z/d-8oCe) what's gonna happen when the code
compiles. The link there has the page set for the `stable` 1.30 compiler just so
that the link results stay consistent if you read this book in a year or
something. Also, we've set the target to `thumbv6m-none-eabi`, which is a
slightly later version of ARM than the actual GBA, but it's close enough for
just checking. Of course, in a full program small functions like these will
probably get inlined into the calling code and disappear entirely as they're
folded and refolded by the compiler, but we can just check.

It turns out that the `!=0` test is 4 instructions and the `==0` test is 6
instructions. Since we want to get savings where we can, and we'll probably
check the keys of an input often enough, we'll just always use a `!=0` test and
then adjust how we initially read the register to compensate. By using xor with
a mask for only the 10 used bits we can flip the "low when pressed" values so
that the entire result has active bits in all positions where a key is pressed.

```rust
pub fn key_input() -> KeyInputSetting {
  unsafe { KeyInputSetting(KEYINPUT.read_volatile() ^ 0b0000_0011_1111_1111) }
}
```

Now we add a method for seeing if a key is pressed. In the full library there's
a more advanced version of this that's built up via macro, but for this example
we'll just name a bunch of `const` values and then have a method that takes a
value and says if that bit is on.

```rust
pub const KEY_A: u16 = 1 << 0;
pub const KEY_B: u16 = 1 << 1;
pub const KEY_SELECT: u16 = 1 << 2;
pub const KEY_START: u16 = 1 << 3;
pub const KEY_RIGHT: u16 = 1 << 4;
pub const KEY_LEFT: u16 = 1 << 5;
pub const KEY_UP: u16 = 1 << 6;
pub const KEY_DOWN: u16 = 1 << 7;
pub const KEY_R: u16 = 1 << 8;
pub const KEY_L: u16 = 1 << 9;

impl KeyInputSetting {
  pub fn contains(&self, key: u16) -> bool {
    (self.0 & key) != 0
  }
}
```

Because each key is a unique bit you can even check for more than one key at
once by just adding two key values together.

```rust
let input_contains_a_and_l = input.contains(KEY_A + KEY_L);
```

And we wanted to save the state of an old frame and compare it to the current
frame to see what was different:

```rust
  pub fn difference(&self, other: KeyInputSetting) -> KeyInputSetting {
    KeyInputSetting(self.0 ^ other.0)
  }
```

Anything that's "in" the difference output is a key that _changed_, and then if
the key reads as pressed this frame that means it was just pressed. The exact
mechanics of all the ways you might care to do something based on new key
presses is obviously quite varied, but it might be something like this:

```rust
let this_frame_diff = this_frame_input.difference(last_frame_input);

if this_frame_diff.contains(KEY_B) && this_frame_input.contains(KEY_B) {
  // the user just pressed B, react in some way
}
```

And for the arrow pad, we'll make an enum that easily casts into `i32`. Whenever
we're working with stuff we can try to use `i32` / `isize` as often as possible
just because it's easier on the GBA's CPU if we stick to its native number size.
Having it be an enum lets us use `match` and be sure that we've covered all our
cases.

```rust
/// A "tribool" value helps us interpret the arrow pad.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(i32)]
pub enum TriBool {
  Minus = -1,
  Neutral = 0,
  Plus = +1,
}
```

Now, how do we determine _which way_ is plus or minus? Well... I don't know.
Really. I'm not sure what the best one is because the GBA really wants the
origin at 0,0 with higher rows going down and higher cols going right. On the
other hand, all the normal math you and I learned in school is oriented with
increasing Y being upward on the page. So, at least for this demo, we're going
to go with what the GBA wants us to do and give it a try. If we don't end up
confusing ourselves then we can stick with that. Maybe we can cover it over
somehow later on.

```rust
  pub fn column_direction(&self) -> TriBool {
    if self.contains(KEY_RIGHT) {
      TriBool::Plus
    } else if self.contains(KEY_LEFT) {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }

  pub fn row_direction(&self) -> TriBool {
    if self.contains(KEY_DOWN) {
      TriBool::Plus
    } else if self.contains(KEY_UP) {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }
```

So then in our game, every frame we can check for `column_direction` and
`row_direction` and then apply those to the player's current position to make
them move around the screen.

With that settled I think we're all done with user input for now. There's some
other things to eventually know about like key interrupts that you can set and
stuff, but we'll cover that later on because it's not necessary right now.
