vim9script

def HelloWorld()
    echow "Hello world!"
enddef

command HelloWorldCmd call HelloWorld()
nnoremap <leader>hw :HelloWorldCmd<CR>

# Open this file and run `:source %` and then the `\hw` should run the
# function.
