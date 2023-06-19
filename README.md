

# Usage

```bash
# checks for mounted Kindles, outputs to clippings/
kindlecliprs

# if that doesn't work point it to your clippings file
kindlecliprs -f /path/to/My\ Clippings.txt
```

The first version should work on Linux and MacOS; it checks for mounted Kindles using `df` and then prompts for confirmation before parsing.

## Input

Kindles store highlights, notes and bookmarks in `My Clippings.txt` files which can be parsed:

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

## Output

This script uses [askama](https://github.com/djc/askama/tree/main) to render the [template](/templates/quote_template.md) file. The template is based on [Jinja](https://jinja.palletsprojects.com/) which is highly customisable, but the script must be recompiled after changing the template because it becomes part of the code via a Rust macro.

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

# About

I wanted to improve my Rust and add functionality to [this](https://github.com/robertmartin8/KindleClippings) great project (which heavily inspired this one).



