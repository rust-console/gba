# The Hardware Memory Map

So we saw `hello_magic.rs` and then we learned what `volatile` was all about,
but we've still got a few things that are a bit mysterious. You can't just cast
a number into a pointer and start writing to it! That's totally crazy! That's
writing to un-allocated memory! Against the rules!

Well, _kinda_. It's true that you're not allowed to write _anywhere at all_, but
those locations were carefully selected locations.

You see, on a modern computer if you need to check if a key is pressed you ask
the Operating System (OS) to please go check for you. If you need to play a
sound, you ask the OS to please play the sound on a default sound output. If you
need to show a picture you ask the OS to give you access to the video driver so
that you can ask the video driver to please put some pixels on the screen.
That's mostly fine, except how does the OS actually do it? It doesn't have an OS
to go ask, it has to stop somewhere.

Ultimately, every piece of hardware is mapped into somewhere in the address
space of the CPU. You can't actually tell that this is the case as a normal user
because your program runs inside a virtualized address space. That way you can't
go writing into another program's memory and crash what they're doing or steal
their data (well, hopefully, it's obviously not perfect). Outside of the
virtualization layer the OS is running directly in the "true" address space, and
it can access the hardware on behalf of a program whenever it's asked to.

How does directly accessing the hardware work, _precisely_? It's just the same
as accessing the RAM. Each address holds some bits, and the CPU picks an address
and loads in the bits. Then the program gets the bits and has to decide what
they mean. The "driver" of a hardware device is just the layer that translates
between raw bits in the outside world and more meaningful values inside of the
program.

Of course, memory mapped hardware can change its bits at any time. The user can
press and release a key and you can't stop them. This is where `volatile` comes
in. Whenever there's memory mapped hardware you want to access it with
`volatile` operations so that you can be sure that you're sending the data every
time, and that you're getting fresh data every time.

## GBA Specifics

That's enough about the general concept of memory mapped hardware, let's get to
some GBA specifics. The GBA has the following sections in its memory map.

* BIOS
* Internal Work RAM (IWRAM)
* External Work RAM (EWRAM)
* IO Registers
* Palette RAM (PALRAM)
* Video RAM (VRAM)
* Object Attribute Memory (OAM)
* Game Pak ROM (ROM)
* Save RAM (SRAM)

Each of these has a few key points of interest:

* **Bus Width:** Also just called "bus", this is how many little wires are
  _physically_ connecting a part of the address space to the CPU. If you need to
  transfer more data than fits in the bus you have to do repeated transfers
  until it all gets through.
* **Read/Write Modes:** Most parts of the address space can be read from in 8,
  16, or 32 bits at a time (there's a few exceptions we'll see). However, a
  significant portion of the address space can't accept 8 bit writes. Usually
  this isn't a big deal, but standard `memcopy` routine switches to doing a
  byte-by-byte copy in some situations, so we'll have to be careful about using
  it in combination with those regions of the memory.
* **Access Speed:** On top of the bus width issue, not all memory can be
  accessed at the same speed. The "fast" parts of memory can do a read or write
  in 1 cycle, but the slower parts of memory can take a few cycles per access.
  These are called "wait cycles". The exact timings depend on what you configure
  the system to use, which is also limited by what your cartridge physically
  supports. You'll often see timings broken down into `N` cycles (non-sequential
  memory access) and `S` cycles (sequential memory access, often faster). There
  are also `I` cycles (internal cycles) which happen whenever the CPU does an
  internal operation that's more than one cycle to complete (like a multiply).
  Don't worry, you don't have to count exact cycle timings unless you're on the
  razor's edge of the GBA's abilities. For more normal games you just have to be
  mindful of what you're doing and it'll be fine.

Let's briefly go over the major talking points of each memory region. All of
this information is also available in GBATEK, mostly in their [memory
map](http://www.akkit.org/info/gbatek.htm#gbamemorymap) section (though somewhat
spread through the rest of the document too).

### BIOS

* **Location:** `0x0` to `0x3FFF` (16k)
* **Bus:** 32-bit
* **Access:** Protected read-only (see text).
* **Wait Cycles:** None

The "basic input output system". This contains a grab bag of utilities that do
various tasks. The code is optimized for small size rather than great speed, so
you can sometimes write faster versions of these routines. Also, calling a bios
function has more overhead than a normal function call. You can think of bios
calls as being similar to system calls to the OS on a desktop computer. Useful,
but costly.

As a side note, not only is BIOS memory read only, but it's memory protected so
that you can't even read from bios memory unless the system is currently
executing a function that's in bios memory. There's actually a bug in one bios
call that lets you read the bytes of the rest of the bios (if you really want),
but a normal `read_volatile` won't do the trick.

### Internal Work RAM (IWRAM)

* **Location:** .
* **Bus:** .
* **Access:** .
* **Wait Cycles:** .

### External Work RAM (EWRAM)

* **Location:** .
* **Bus:** .
* **Access:** .
* **Wait Cycles:** .

### IO Registers

* **Location:** .
* **Bus:** .
* **Access:** .
* **Wait Cycles:** .

### Palette RAM (PALRAM)

* **Location:** .
* **Bus:** .
* **Access:** .
* **Wait Cycles:** .

### Video RAM (VRAM)

* **Location:** .
* **Bus:** .
* **Access:** .
* **Wait Cycles:** .

### Object Attribute Memory (OAM)

* **Location:** .
* **Bus:** .
* **Access:** .
* **Wait Cycles:** .

### Game Pak ROM (ROM)

* **Location:** .
* **Bus:** .
* **Access:** .
* **Wait Cycles:** .

### Save RAM (SRAM)

* **Location:** .
* **Bus:** .
* **Access:** .
* **Wait Cycles:** .

