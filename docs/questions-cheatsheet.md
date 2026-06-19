# Questions Cheatsheet

Concise running log of questions asked so far. Repeated questions get a higher `Score`. Vim questions are normalized to Neovim.

## Neovim

### Motion

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I go to the end of the line? | Use `g_` for the last non-blank character, or `$` for the absolute end of the line. |
| 1 | How do I start insert at the end of the line? | Press `A` in normal mode. |
| 1 | How do I start insert at the beginning of the line? | Press `I` in normal mode. |
| 1 | How do I move the cursor to the beginning of a line without going into insert mode? | Use `0` for column 1 or `^` for the first non-blank character. |
| 1 | How do I jump to a line in vim? | Use `42G` or `:42`. |
| 1 | How do I advance the cursor to the next word? | Use `w` for the next word start, `e` for the word end, `b` to go back. |
| 1 | How do I jump to the last line in vim? | Use `G`. |
| 1 | How do I move backwards by word in nvim? | Use `b` to jump to the start of the previous word. |
| 1 | How do I delete a word without entering insert mode? | Use `dw` to delete from the cursor to the start of the next word. |
| 1 | How do I jump to the top of a file? | Use `gg`. |

### Editing

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I overwrite where the cursor is? | Use `s`, `r<char>`, `R`, or `cw` depending on how much you want to replace. |
| 1 | How do I set tabs to 4 spaces? | Set `tabstop=4`, `shiftwidth=4`, `expandtab`, and usually `softtabstop=4`. |
| 1 | How do I insert a newline above in nvim? | Press `O` in normal mode to open a line above and enter insert mode. |
| 1 | How do I insert a line without going into insert mode? | Use `:put =''` to insert a blank line below, or `:put! =''` above. |
| 1 | What does `Shift-J` do in nvim and how do I undo it? | `J` joins the current line with the next line; undo with `u`. |
| 1 | How do I move a line forward or backward a tab stop in normal mode? | Use `>>` to indent the current line and `<<` to outdent it. |
| 1 | How do I move a text block up or down in nvim? | In visual mode, select the block and use `:m '<-2` to move up or `:m '>+1` to move down. |
| 1 | How do I remove a visual block and paste it at a certain line in nvim? | Yank or cut the selection with `d` or `y`, jump to the target line, then paste with `p` or `P`. |
| 1 | How do I paste inline after a visual yank in nvim? | Use `p` to paste after the cursor or `P` to paste before it; for linewise yanks, `p` places it below the current line. |
| 1 | How do I copy a single line to another file? | Yank the line with `yy`, open the target file, move to the destination, and paste with `p` or `P`. |
| 1 | What is the single-line version of the visual `>>` workflow? | Use `:w >> existingfile` if you want to append the current line or selection to another file. |
| 1 | How do I move a visual code block into a new file or append it to an existing file? | In visual mode, use `:'<,'>w newfile` to write the selection, or `:'<,'>w >> existingfile` to append. Delete the original selection after if you want a move. |
| 1 | How do I yank a line range? | Use an Ex range like `:10,20y` to yank lines 10 through 20. |
| 1 | How do I yank 10 lines before or after the cursor? | Use `:.-10,.y` for the previous 10 lines or `:.,.+10y` for the next 10 lines. |
| 1 | How do I yank from the current line to the end of the file? | Use `:.,$y`. |
| 1 | How do I do a global find and replace? | Use `:%s/old/new/g` to replace across the whole file. |
| 1 | How do I reset the buffer to the last saved state in nvim? | Use `:e!` to reload the file from disk and discard unsaved changes. |
| 1 | How do I reload a file from disk in nvim? | Use `:e` to reload if unchanged, or `:e!` to discard unsaved buffer changes and reload. |
| 1 | What does `recording @u` mean in nvim and how do I end it? | You started recording a macro into register `u`; press `q` again to stop recording. |
| 1 | My line numbers start at the current line in nvim. How do I get absolute line numbering? | Turn off `relativenumber` and keep `number` enabled. |
| 2 | How do I set absolute line numbering again? | Use `:set number norelativenumber`. |
| 1 | How do I cycle a collapsed function/fold in nvim? | Use `za` to toggle the fold under the cursor; `zo` opens it and `zc` closes it. |

### Layout

| Score | Question | Short answer |
|---|---|---|
| 1 | Can I show 2 tabs side by side in nvim? | Not as tabs; use a vertical split with `:vsplit` or `:vsp`. |
| 1 | How do I switch between splits in nvim? | Use `Ctrl-w` followed by `h`, `j`, `k`, or `l` to move between windows. |
| 1 | When in a vertical split how do I navigate between splits? | Use `Ctrl-w h` and `Ctrl-w l` to move left and right. |
| 1 | How can I open a file in a new tab in nvim? | Use `:tabnew filename` or `:tabedit filename`. |
| 2 | How do I switch to another tab in nvim? | Use `gt` for the next tab, `gT` for the previous tab, or `:tabnext N`; in LazyVim buffer tabs, use `Shift-l` / `Shift-h`. |

### Search

| Score | Question | Short answer |
|---|---|---|
| 1 | After a search with `/`, how do I set the cursor to the found word? | Press `Enter` to jump to the current match; `n` and `N` move through matches. |

### Config

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I turn on line numbers in nvim? | Use `:set number`; add `set number` to `init.vim` or `vim.opt.number = true` to `init.lua`. |
| 1 | How do I set absolute line numbering in the config? | Add `vim.opt.number = true` and `vim.opt.relativenumber = false` to `init.lua`, or `set number norelativenumber` to `init.vim`. |
| 1 | Is there a configuration in lazy.nvim where this is set? | Not by default; put the option in your own Neovim config, not in `lazy.nvim` itself. |
| 1 | How do I search my nvim config for the file where a setting is currently set? | Use `rg -n "number|relativenumber" ~/.config/nvim` or `:scriptnames` to trace loaded files. |
| 1 | Where does the nvim init file go? | Usually `~/.config/nvim/init.lua` or `~/.config/nvim/init.vim`. |
| 1 | Can nvim tell me what file it loaded? | Use `:echo $MYVIMRC`, `:echo stdpath('config')`, or `:scriptnames`. |
| 1 | How do I install plugins in nvim? | Use a plugin manager such as `lazy.nvim` and declare plugins in your Neovim config. |
| 1 | What are the red triangles next to line numbers in LazyVim? | They are `gitsigns.nvim` Git delete/topdelete hunk markers from LazyVim's sign column config. |
| 1 | What does a `0` beside a line number mean in LazyVim? | It is a sign-column marker; use `:sign place buffer=<bufnr>` or `:Inspect` on the line to identify the plugin/source. |

### LSP

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I add rust-analyzer to nvim? | Install `rust-analyzer`, add `rust-src`, then enable `rust_analyzer` through Neovim LSP config. |
| 1 | How do I open a function definition from Rust in nvim? | Use `gd` to jump to the local definition, or `gD` for the global definition. |
| 1 | How do I get rust-analyzer to display why it is tagging an error? | Open the diagnostic float with `vim.diagnostic.open_float()` or hover the item; the message usually explains the reason. |
| 1 | How do I view the rust-analyzer output if it is showing an error? | Use `:lua print(vim.lsp.get_log_path())` to find the LSP log, and `:checkhealth vim.lsp` or `:lua vim.diagnostic.open_float()` to inspect errors. |
| 1 | Is there a way in nvim to move to the next function in Rust? | Try `]m` for the next method/function start and `[m` for the previous one. |
| 1 | How do I exit comment mode after creating a newline under a comment in rust? | Press `Enter` or `o` to keep the comment, or disable auto-comment continuation with `:set formatoptions-=r formatoptions-=o`. |
| 1 | How do I open a Rust definition in a new tab with rust-analyzer? | Use `:tab split | lua vim.lsp.buf.definition()` from the symbol, or map a key to that sequence. |

## Rust

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I run the rust linter? | Run `cargo clippy`. |
| 1 | Should the linter catch different sized tabs in a Rust file? | Usually no. This is a formatting/editor setting issue, not something `clippy` is meant to enforce. Use `rustfmt` and editor tab settings. |
| 1 | How do I run rustfmt on my project? | Run `cargo fmt` from the project root. |
| 1 | How can I tell cargo fmt how many changes it made? | `cargo fmt` does not report a count; run it, then inspect `git diff --stat` or `git diff --shortstat`. |
| 1 | How can I quickly count all lines in files below `./src`? | Use `find src -type f -print0 | xargs -0 wc -l` for a file-by-file count and total. |

## Rust + Neovim

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I add rust-analyzer to nvim? | Install `rust-analyzer`, add `rust-src`, then enable `rust_analyzer` through Neovim LSP config. |
| 1 | How do I get rust-analyzer to display why it is tagging an error? | Open the diagnostic float with `vim.diagnostic.open_float()` or hover the item; the message usually explains the reason. |
| 1 | How do I open a Rust definition in a new tab with rust-analyzer? | Use `:tab split | lua vim.lsp.buf.definition()` from the symbol, or map a key to that sequence. |
| 1 | How do I view the rust-analyzer output if it is showing an error? | Use `:lua print(vim.lsp.get_log_path())` to find the LSP log, and `:checkhealth vim.lsp` or `:lua vim.diagnostic.open_float()` to inspect errors. |
| 1 | Is there a way in nvim to move to the next function in Rust? | Try `]m` for the next method/function start and `[m` for the previous one. |
| 1 | How do I exit comment mode after creating a newline under a comment in rust? | Press `Enter` or `o` to keep the comment, or disable auto-comment continuation with `:set formatoptions-=r formatoptions-=o`. |

## Glow

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I add a pager to Glow? | Use `glow -p <file>` or set `pager: true` in `glow.yml`. |

## Ghostty

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I switch between terminals in the same window from the keyboard? | Check `ghostty +list-keybinds --default` and look for `next_tab`, `previous_tab`, or `goto_split`. |

## Git

| Score | Question | Short answer |
|---|---|---|
| 1 | How do I change the commit message on my last git commit? | Run `git commit --amend -m "new message"`. |
