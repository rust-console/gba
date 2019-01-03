# Direct Memory Access

The GBA has four Direct Memory Access (DMA) units that can be utilized. They're
mostly the same in terms of overall operation, but each unit has special rules
that make it better suited to a particular task.

**Please Note:** TONC and GBATEK have slightly different concepts of how a DMA
unit's registers should be viewed. I've chosen to go by what GBATEK uses.

## General DMA

A single DMA unit is controlled through four different IO Registers.

* **Source:** (`DMAxSAD`, read only) A `*const` pointer that the DMA reads from.
* **Destination:** (`DMAxDAD`, read only) A `*mut` pointer that the DMA writes
  to.
* **Count:** (`DMAxCNT_L`, read only) How many transfers to perform.
* **Control:** (`DMAxCNT_H`, read/write) A register full of bit-flags that
  controls all sorts of details.

Here, the `x` is replaced with 0 through 3 when utilizing whichever particular
DMA unit.

### Source Address

This is either a `u32` or `u16` address depending on the unit's assigned
transfer mode (see Control). The address MUST be aligned.

With DMA0 the source must be internal memory. With other DMA units the source
can be any non-`SRAM` location.

### Destination Address

As with the Source, this is either a `u32` or `u16` address depending on the
unit's assigned transfer mode (see Control). The address MUST be aligned.

With DMA0/1/2 the destination must be internal memory. With DMA3 the destination
can be any non-`SRAM` memory (allowing writes into Game Pak ROM / FlashROM,
assuming that your Game Pak hardware supports that).

### Count

This is a `u16` that says how many transfers (`u16` or `u32`) to make.

DMA0/1/2 will only actually accept a 14-bit value, while DMA3 will accept a full
16-bit value. A value of 0 instead acts as if you'd used the _maximum_ value for
the DMA in question. Put another way, DMA0/1/2 transfer `1` through `0x4000`
words, with `0` as the `0x4000` value, and DMA3 transfers `1` through `0x1_0000`
words, with `0` as the `0x1_0000` value.

The maximum value isn't a very harsh limit. Even in just `u16` mode, `0x4000`
transfers is 32k, which would for example be all 32k of `IWRAM` (including your
own user stack). If you for some reason do need to transfer more than a single
DMA use can move around at once then you can just setup the DMA a second time
and keep going.

### Control

This `u16` bit-flag field is where things get wild.

* Bits 0-4 do nothing
* Bit 5-6 control how the destination address changes per transfer:
  * 0: Offset +1
  * 1: Offset -1
  * 2: No Change
  * 3: Offset +1 and reload when a Repeat starts (below)
* Bit 7-8 similarly control how the source address changes per transfer:
  * 0: Offset +1
  * 1: Offset -1
  * 2: No Change
  * 3: Prohibited
* Bit 9: enables Repeat mode.
* Bit 10: Transfer `u16` (false) or `u32` (true) data.
* Bit 11: "Game Pak DRQ" flag. GBATEK says that this is only allowed for DMA3,
  and also your Game Pak hardware must be equipped to use DRQ mode. I don't even
  know what DRQ mode is all about, and GBATEK doesn't say much either. If DRQ is
  set then you _must not_ set the Repeat bit as well. The `gba` crate simply
  doesn't bother to expose this flag to users.
* Bit 12-13: DMA Start:
  * 0: "Immediate", which is 2 cycles after requested.
  * 1: VBlank
  * 2: HBlank
  * 3: Special, depending on what DMA unit is involved:
    * DMA0: Prohibited.
    * DMA1/2: Sound FIFO (see the [Sound](04-sound.md) section)
    * DMA3: Video Capture, intended for use with the Repeat flag, performs a
      transfer per scanline (similar to HBlank) starting at `VCOUNT` 2 and
      stopping at `VCOUNT` 162. Intended for copying things from ROM or camera
      into VRAM.
* Bit 14: Interrupt upon DMA complete.
* Bit 15: Enable this DMA unit.

## DMA Life Cycle

The general technique for using a DMA unit involves first setting the relevent
source, destination, and count registers, then setting the appropriate control
register value with the Enable bit set.

Once the Enable flag is set the appropriate DMA unit will trigger at the
assigned time (Bit 12-13). The CPU's operation is halted while any DMA unit is
active, until the DMA completes its task. If more than one DMA unit is supposed
to be active at once, then the DMA unit with the lower number will activate and
complete before any others.

When the DMA triggers via _Enable_, the `Source`, `Destination`, and `Count`
values are copied from the GBA's registers into the DMA unit's internal
registers. Changes to the DMA unit's internal copy of the data don't affect the
values in the GBA registers. Another _Enable_ will read the same values as
before.

If DMA is triggered via having _Repeat_ active then _only_ the Count is copied
in to the DMA unit registers. The `Source` and `Destination` are unaffected
during a Repeat. The exception to this is if the destination address control
value (Bits 5-6) are set to 3 (`0b11`), in which case a _Repeat_ will also
re-copy the `Destination` as well as the `Count`.

Once a DMA operation completes, the Enable flag of its Control register will
automatically be disabled, _unless_ the Repeat flag is on, in which case the
Enable flag is left active. You will have to manually disable it if you don't
want the DMA to kick in again over and over at the specified starting time.

## DMA Limitations

The DMA units cannot access `SRAM` at all.

If you're using HBlank to access any part of the memory that the display
controller utilizes (`OAM`, `PALRAM`, `VRAM`), you need to have enabled the
"HBlank Interval Free" bit in the Display Control Register (`DISPCNT`).

Whenever DMA is active the CPU is _not_ active, which means that
[Interrupts](05-interrupts.md) will not fire while DMA is happening. This can
cause any number of hard to track down bugs. Try to limit your use of the DMA
units if you can.
