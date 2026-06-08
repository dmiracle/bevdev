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
| 1 | How do I move a text block up or down in nvim? | In visual mode, select the block and use `:m '<-2` to move up or `:m '>+1` to move down. |
| 1 | How do I remove a visual block and paste it at a certain line in nvim? | Yank or cut the selection with `d` or `y`, jump to the target line, then paste with `p` or `P`. |
| 1 | How do I jump to the last line in vim? | Use `G`. |
| 1 | How do I paste inline after a visual yank in nvim? | Use `p` to paste after the cursor or `P` to paste before it; for linewise yanks, `p` places it below the current line. |
| 1 | How do I move backwards by word in nvim? | Use `b` to jump to the start of the previous word. |
| 1 | How do I open a function definition from Rust in nvim? | Use `gd` to jump to the local definition, or `gD` for the global definition. |
| 1 | How do I get rust-analyzer to display why it is tagging an error? | Open the diagnostic float with `vim.diagnostic.open_float()` or hover the item; the message usually explains the reason. |
| 1 | How can I open a file in a new tab in nvim? | Use `:tabnew filename` or `:tabedit filename`. |
| 1 | How do I switch to another tab in nvim? | Use `gt` for the next tab, `gT` for the previous tab, or `:tabnext N`. |
| 1 | How do I jump to the top of a file? | Use `gg`. |
| 1 | How do I do a global find and replace? | Use `:%s/old/new/g` to replace across the whole file. |

## Glow

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I add a pager to Glow? | Use `glow -p <file>` or set `pager: true` in `glow.yml`. |

## Ghostty

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I switch between terminals in the same window from the keyboard? | Check `ghostty +list-keybinds --default` and look for `next_tab`, `previous_tab`, or `goto_split`. |
