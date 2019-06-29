# clima
A Markdown terminal reader (experimental)

**Warning:** This is **not** suitable for normal usage.
This might corrupt your hard disk, change your passwords and sell your house.


This tiny application opens any Markdown file and displays it using **Termimad**.
Which isn't really useful as Termimad hasn't been designed to display any markdown file ([Termimad](https://github.com/Canop/Termimad) is more of an helper to let you incorporate rich text and tables in your terminal applications).


Maybe I'll add the missing features to Termimad, then to **Clima**.

In the meantime *the only reasonnable usage of **Clima*** is to try out or fix some markdown you'd like to incorporate into your terminal application using Termimad and that you prefer to edit as a file.

## Usage

Fetch the repository, go to it, then do

    cargo build -release

The executable is in `target/release`. You may now ensure it's found when you need it. For example:

    sudo ln -s path-to-clima/target/release/clima /usr/bin/clima

To open a file, just pass its path:

    clima README.md


