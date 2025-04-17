# QAOP's Personal Editor

Rather than customise Neovim or Emacs, I have decided to write my own text
editor for my personal programming projects.  I will design the interface as I
wish, and will probably be inspired by many plugins.

I intend to add to it over the years, making it my ultimate editor.

# Planned integrations

Although, I will design and code the editor, I do intend to take advantage of
technologies out there to help make it a great editor for me.  Such technologies
are, but not limited to:

- Tree-sitter, for fast parsing, syntax highlighting and code navigation.
- LSP support for Intellisense and advanced code navigation (possibly the
  `async-lsp` crate).
- Rope-buffer support for text editing (probably the `crop` crate).
- Crossterm (and perhaps Ratatui) for terminal output.

It will be interesting to integrate Boyer-Moore text searches into the rope
buffers, and even to extend that for regex searches.
