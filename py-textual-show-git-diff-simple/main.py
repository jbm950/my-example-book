from pathlib import Path
from subprocess import DEVNULL, PIPE, Popen

from textual.app import App
from textual.widgets import Header, Footer, Static


class HelloWorldApp(App):
    def __init__(self, diff):
        super().__init__()
        self.diff = diff

    def compose(self):
        yield Header()
        yield Static(self.diff)
        yield Footer()


def git_diff(directory, start_hash, end_hash):
    process = Popen(['git', 'diff',  start_hash, end_hash],
                    cwd=directory,
                    text=True, stdout=PIPE, stderr=DEVNULL)

    return process.stdout.read()


if __name__ == "__main__":
    path = Path('/home/jmilam/git_repos/milam-notes')
    start_hash = '1dd0e91'
    end_hash = '1395438'
    diff = git_diff(path, start_hash, end_hash)

    HelloWorldApp(diff).run()

