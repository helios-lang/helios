# Helios Language Server (`helios-ls`)

The Helios Language Server is an intermediary between an editor or IDE and the
Helios compiler. It implements the [language server protocol], which allows us
to provide Helios with common editor functionality such as autocomplete, go-to
definitions and find-all-references in one centralised location.

This project is still in its early stages of development. So far, only a limited
functionality is provided. You may see the progress of the project under the
[Progress](#Progress) section.

## Progress

- [x] `initialize`
- [x] `textdocument/didOpen`
- [ ] `textdocument/didChange`
- [ ] `textdocument/didSave`
- [-] `textdocument/completion`
- [ ] `textdocument/hover`
- [ ] `textdocument/rename`

[language server protocol]: https://microsoft.github.io/language-server-protocol/
