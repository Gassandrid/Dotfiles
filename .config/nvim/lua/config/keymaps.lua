-- Keymaps are automatically loaded on the VeryLazy event
-- Default keymaps that are always set: https://github.com/LazyVim/LazyVim/blob/main/lua/lazyvim/config/keymaps.lua
-- Add any additional keymaps here

-- keymap bindings ctrl + r to run the current python script only if it is python, using venv at ~/python_main_venv/
vim.api.nvim_create_autocmd("FileType", {
  pattern = "python",
  callback = function()
    vim.keymap.set("n", "<leader>rr", function()
      local file_path = vim.fn.expand("%:p")
      local venv_path = vim.fn.expand("~/python_main_venv/bin/python")
      local cmd = string.format("%s %s", venv_path, file_path)

      -- Open LazyVim's terminal toggle (same as Ctrl+/)
      vim.cmd("ToggleTerm")

      -- Wait a moment for the terminal to open
      vim.defer_fn(function()
        -- Send command to the terminal
        require("toggleterm").exec(cmd)
      end, 100)
    end, { noremap = true, silent = true, buffer = true, desc = "Run Python script with venv in ToggleTerm" })
  end,
})
