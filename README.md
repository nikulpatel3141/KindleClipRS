
Split your Kindle Highlights, Notes and Bookmarks to plain text files.

There are [many](https://github.com/topics/kindle-clippings) existing solutions for this, though none of the ones I've seen easily let you [change the output formatting](#clipping-formatting)..

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

## Usage

Running the release binary for your system will output parsed clippings to `clippings/`

```bash
# detect mounted Kindles - only on Linux/MacOS
$ kindlecliprs
...
 
# if that doesn't work point it to your clippings file
$ kindlecliprs -f /path/to/My\ Clippings.txt
...

# or run from source
$ git clone https://github.com/nikulpatel3141/KindleClipRS
...
$ cd KindleClipRS
$ cargo run -- -f /path/to/My\ Clippings.txt
...
```

Alternatively you can use `cargo run`, or `cargo run -- -f ...` if you have [Rust](https://www.rust-lang.org/tools/install) installed.

### Input

Kindles store highlights, notes and bookmarks in `My Clippings.txt` files eg:

```
The Infinite Game (Sinek, Simon)
- Your Bookmark on page 153 | location 1795 | Added on Monday, 1 May 2023 18:26:53


==========
Pragmatic Programmer, The (Thomas, David)
- Your Highlight on page 162 | location 1880-1881 | Added on Monday, 1 May 2023 19:56:58

Every time you find yourself doing something repetitive, get into the habit of thinking “there must be a better way.” Then find it.
==========
Think Again (Grant, Adam)
- Your Note on page 169 | location 1955 | Added on Monday, 1 May 2023 22:49:34

Remember this
==========
The Infinite Game (Sinek, Simon)
- Your Highlight on page 164 | location 2356-2358 | Added on Friday, 2 May 2023 22:57:21

An infinite mindset embraces abundance whereas a finite mindset operates with a scarcity mentality.
==========
```

### Output

This script parses the clippings file into `Clipping` structs and renders them using a [Jinja](https://jinja.palletsprojects.com/) [template](/templates/clipping_template.md).

It will output the parsed clippings into separate files:

```bash
$ kindlecliprs
[2023-06-19T19:13:23Z INFO  kindlecliprs] Attempting to find a clipping file from any mounted Kindles
[2023-06-19T19:13:23Z INFO  kindlecliprs] Found a Kindle mounted on /media/nikul/Kindle
Found clippings file /media/nikul/Kindle/documents/My Clippings.txt, do you want to continue? yes
...
$ cd clippings/
$ tail -n +1 *
==> Pragmatic Programmer, The - Thomas, David.md <==
- *Highlight* (page: 162)
`Every time you find yourself doing something repetitive, get into the habit of thinking “there must be a better way.” Then find it.`

==> The Infinite Game - Sinek, Simon.md <==
- *Bookmark* (page: 153)
``

- *Highlight* (page: 164)
`An infinite mindset embraces abundance whereas a finite mindset operates with a scarcity mentality.`

==> Think Again - Grant, Adam.md <==
- *Note* (page: 169)
`Remember this`
```

### Clipping Formatting

The [template](/templates/clipping_template.md) file determines how the `Quote` structs are rendered at compile time. The template becomes part of the code via a Rust macro. See [askama](https://github.com/djc/askama/tree/main) for more details.

The upside of this is template validity is checked at compile time, but you'll need to recompile when you want to change the template.


## About

This project is based on [this](https://github.com/robertmartin8/KindleClippings) great one and is a means for me to improve my Rust. I've definitely learned a lot through this project, though I've still got a long way to go. Please reach out with feedback if you have any!

## TODO

- Upload to [crates.io](https://crates.io/)
- Add logic to remove deleted highlights
- Write tests
