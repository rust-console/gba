# Book Goals and Style

So, what's this book actually gonna teach you?

I'm _not_ gonna tell you how to use a crate that already exists.

Don't get me wrong, there _is_ a [gba](https://crates.io/crates/gba) crate, and
it's on crates.io and all that jazz.

However, unlike most crates that come with a tutorial book, I don't want to just
teach you how to use the crate. What I want is to teach you what you need to
know so that you could build the crate yourself, from scratch, if it didn't
already exist for you. Let's call it the [Handmade
Hero](https://handmadehero.org/) school of design. Much more than you might find
in other Rust crate books, I'll be attempting to show a lot of the _why_ in
addition to just the _how_. Once you know how to do it all on your own, you can
decide for yourself if the `gba` crate does it well, or if you think you can
come up with something that suits your needs better.

Overall the book is sorted for easy review once you're trying to program
something, and the GBA has a few interconnected concepts, so some parts of the
book end up having to refer you to portions that you haven't read yet. The
chapters and sections are sorted so that _minimal_ future references are
required, but it's unavoidable that it'll happen sometimes.

The actual "tutorial order" of the book is the
[Examples](../05-examples/00-index.md) chapter. Each section of that chapter
breaks down one of the provided examples in the [examples
directory](https://github.com/rust-console/gba/tree/master/examples) of the
repository. We go over what sections of the book you'll need to have read for
the example code to make sense, and also how we apply the general concepts
described in the book to the specific example cases.
