# Helios Language Server (HeliosLS)

The Helios Language Server is an intermediary between an editor or IDE and the
Helios compiler. It implements the [Language Server Protocol][language-server-protocol],
which allows us to provide Helios with common editor functionality such as
autocomplete, go-to definitions and find-all-references in one centralised
location.

This project is still in its early stages of development. So far, only a limited
functionality is provided. You may see the progress of the project under the
[Progress](#Progress) section.

## Building and testing

Because this package is located in the Helios workspace (for now), building and
testing this package (or "crate" in Rust-talk) separately is a little different:

```shell
$ cargo build --package helios-ls # building
$ cargo test --package helios-ls # testing
```

## Usage

Once this package has been built, the executable produced will be called
`helios-ls`. For now, the server can only be used through standard input and
output (`stdin` and `stdout`).

You should not need to invoke this executable manually. There are client
implementations (such as [Helios for Visual Studio Code][vscode-helios-github])
that will do this for you. These clients will handle the communication between
the client and server and provide the editor-specific features previously
mentioned.

## Progress

- [x] `initialize`
- [x] `textdocument/didOpen`
- [ ] `textdocument/didChange`
- [ ] `textdocument/didSave`
- [ ] `textdocument/completion`
- [ ] `textdocument/hover`
- [ ] `textdocument/rename`

## License

Unless explicitly stated otherwise, all files in this repository are licensed
under the [Apache License, Version 2.0][apache-license].

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[language-server-protocol]: https://microsoft.github.io/language-server-protocol/
[vscode-helios-github]: https://github.com/helios-lang/vscode-helios
