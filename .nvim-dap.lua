local function fmtPath(path)
	return string.gsub(path, "\\", "/")
end

local dap = require("dap")
local codelldb_root = vim.fn.stdpath('data') .. "/mason/packages/codelldb/extension/"
local codelldb_path = fmtPath(codelldb_root .. "adapter/codelldb")



dap.adapters.codelldb = {
	type = "server",
	port = "${port}",
	executable = {
		command = codelldb_path,
		args = { "--port", "${port}" },
		detached = false
	},
}

dap.configurations.rust = {
	{
		name = "Launch & Debug Game (GDExt Rust)",
		type = "codelldb",
		request = "launch",
		program = function()
			return "E:/gameDev/Godot/Engine/godot/bin/godot.windows.editor.x86_64.exe";
		end,
		args = {
			"-e",
			"-w"
		},
		cwd = "${workspaceFolder}",
		stopOnEntry = false,
	},
}

vim.api.nvim_create_autocmd({"BufWritePost"}, {
	pattern = {"*.rs"},
	command = "!cargo build --manifest-path=gdext/rust/Cargo.toml"
})
