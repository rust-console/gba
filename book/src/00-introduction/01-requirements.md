# Reader Requirements

This book naturally assumes that you've already read Rust's core book:

* [The Rust Programming Language](https://doc.rust-lang.org/book/)

Now, I _know_ it sounds silly to say "if you wanna program Rust on this old
video game system you should already know how to program Rust", but the more
people I meet and chat with the more they tell me that they jumped into Rust
without reading any or all of the book. You know who you are.

Please, read the whole book!

In addition to the core book, there's also an expansion book that I will declare
to be required reading for this:

* [The Rustonomicon](https://doc.rust-lang.org/nomicon/)

The Rustonomicon is all about trying to demystify `unsafe`. We'll end up using a
fair bit of unsafe code as a natural consequence of doing direct hardware
manipulations. Using unsafe is like [swinging a
sword](https://www.zeldadungeon.net/wp-content/uploads/2013/04/tumblr_mlkpzij6T81qizbpto1_1280.gif),
you should start slowly, practice carefully, and always pay attention no matter
how experienced you think you've become.

That said, it's sometimes a [necessary
tool](https://www.youtube.com/watch?v=rTo2u13lVcQ) to get the job done, so you
have to break out of the borderline pathological fear of using it that most rust
programmers tend to have.
