vim9script

def PopupString(input_str: string)
    popup_create(input_str, {time: 1000})
enddef

def PopupStringList(input_str_list: list<string>)
    popup_create(input_str_list, {time: 1000})
enddef


# Commands/Keybinds
command PopupStrCmd call PopupString("Hello world!")
command PopupStrListCmd call PopupStringList(["Hello world!", "Another String"])

nnoremap <leader>p1 :PopupStrCmd<CR>
nnoremap <leader>p2 :PopupStrListCmd<CR>

