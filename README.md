[![Latest Version][s1]][l1] [![MIT][s2]][l2] [![Chat on Miaou][s3]][l3]

[s1]: https://img.shields.io/crates/v/clima.svg
[l1]: https://crates.io/crates/clima

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://miaou.dystroy.org/static/shields/room.svg
[l3]: https://miaou.dystroy.org/3768?rust

# Clima

A viewer for Termimad.

This tiny application opens any Markdown file and displays it using **Termimad**.

Which isn't really useful as Termimad hasn't been designed to display any markdown file.
[Termimad](https://github.com/Canop/Termimad) is more of an helper to let you incorporate rich text and tables in your terminal applications.

Clima is thus *not a general purpose Markdown viewer* but can be used to try out or fix some markdown you'd like to incorporate into your terminal application using Termimad and that you prefer to edit as a file.

## Installation from source

Download the Repository

    git clone https://github.com/Canop/clima

and build and install `clima`

    cd clima
    cargo install --path .

## Precompiled binaries

They're available at https://dystroy.org/clima

## Usage

To open a file in the viewer, just pass its path:

    clima README.md

You can also provide the URL of a markdown file or of a github repository:

    clima https://github.com/Canop/broot

If you add the `--print` option, the file is just printed to stdout, the scrollable viewer doesn't appear:

    clima -p README.md


With the `--skin` argument, you can provide a Termimad skin file, either as JSON or Hjson:

    clima --skin skin.hjson

(this repository contains an example of such skin)
