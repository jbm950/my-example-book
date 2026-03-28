vim9script

def PopupAtCursor()
    popup_atcursor("Apparently there's a function for putting the popup at the cursor", {})
enddef

command PopupAtCursorCmd call PopupAtCursor()
nnoremap <leader>p1 :PopupAtCursorCmd<CR>
