# Questions Cheatsheet

Concise running log of questions asked so far. Repeated questions get a higher `Score`. Vim questions are normalized to Neovim.

## Neovim

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I go to the end of the line? | Use `g_` for the last non-blank character, or `$` for the absolute end of the line. |
| 1 | How do I start insert at the end of the line? | Press `A` in normal mode. |
| 1 | How do I turn on line numbers in nvim? | Use `:set number`; add `set number` to `init.vim` or `vim.opt.number = true` to `init.lua`. |
| 1 | Where does the nvim init file go? | Usually `~/.config/nvim/init.lua` or `~/.config/nvim/init.vim`. |
| 1 | Can nvim tell me what file it loaded? | Use `:echo $MYVIMRC`, `:echo stdpath('config')`, or `:scriptnames`. |
| 1 | How do I jump to a line in vim? | Use `42G` or `:42`. |
| 1 | How do I advance the cursor to the next word? | Use `w` for the next word start, `e` for the word end, `b` to go back. |
| 1 | How do I overwrite where the cursor is? | Use `s`, `r<char>`, `R`, or `cw` depending on how much you want to replace. |
| 1 | How do I add rust-analyzer to nvim? | Install `rust-analyzer`, add `rust-src`, then enable `rust_analyzer` through Neovim LSP config. |
| 1 | How do I set tabs to 4 spaces? | Set `tabstop=4`, `shiftwidth=4`, `expandtab`, and usually `softtabstop=4`. |
| 1 | How do I insert a newline above in nvim? | Press `O` in normal mode to open a line above and enter insert mode. |
| 1 | How do I run the rust linter? | Run `cargo clippy`. |
| 1 | Should the linter catch different sized tabs in a Rust file? | Usually no. This is a formatting/editor setting issue, not something `clippy` is meant to enforce. Use `rustfmt` and editor tab settings. |
| 1 | How do I run rustfmt on my project? | Run `cargo fmt` from the project root. |

## Glow

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I add a pager to Glow? | Use `glow -p <file>` or set `pager: true` in `glow.yml`. |

## Ghostty

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I switch between terminals in the same window from the keyboard? | Check `ghostty +list-keybinds --default` and look for `next_tab`, `previous_tab`, or `goto_split`. |
