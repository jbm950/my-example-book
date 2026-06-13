Simple example that shows how to build and run Rust code in a python project.
Steps taken were:
* Setup the project
    * create initial project with `uv new <project name>`
    * `uv add maturin`
    * `uv run maturin new <extension name>`
* Write the code
    * I just edited the main.py and lib.rs files. All the other project files
      were automated during the project setup.
* Run the code
    * Easiest at this point to source the top level .venv rather than run with
      uv.
    * In the extension folder run `maturin develop` to build and install the
      submodule
    * Then you can run `python main.py` like normal
