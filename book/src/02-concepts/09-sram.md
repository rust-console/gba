# Save RAM (SRAM)

* **Address Span:** `0xE00_0000` to `0xE00FFFF` (64k)

The actual amount of SRAM available depends on your game pak, and the 64k figure
is simply the maximum possible. A particular game pak might have less, and an
emulator will likely let you have all 64k if you want.

As with other portions of the address space, SRAM has some number of wait cycles
per use. As with ROM, you can change the wait cycle settings via the `WAITCNT`
register if the defaults don't work well for your game pak. See the ROM section
for full details of how the `WAITCNT` register works.

The game pak SRAM also has only an 8-bit bus, so have fun with that.

The GBA Direct Memory Access (DMA) unit cannot access SRAM.

Also, you [should not write to SRAM with code executing from
ROM](https://problemkaputt.de/gbatek.htm#gbacartbackupsramfram). Instead, you
should move the code to WRAM and execute the save code from there. We'll cover
how to handle that eventually.
