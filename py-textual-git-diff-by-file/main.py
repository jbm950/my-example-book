from pathlib import Path
from subprocess import DEVNULL, PIPE, Popen

from textual.app import App
from textual.widgets import Collapsible, Static


class GitDiffByFile(App):
    def __init__(self, git_diff_summary):
        super().__init__()
        self._git_diff_summary = git_diff_summary

    def compose(self):
        for file_name, file_lines in self._git_diff_summary.items():
            yield Collapsible(Static(_format_lines(file_lines)), title=file_name)


def _format_lines(file_lines):
    result = []
    for line in file_lines:
        if line.startswith('-'):
            result.append(f'[$error]{line}[/]')
        elif line.startswith('+'):
            result.append(f'[$success]{line}[/]')
        elif line.startswith('@@'):
            result.append(f'[$accent]{line}[/]')
        else:
            result.append(line)

    return '\n'.join(result)


def git_diff(directory, start_hash, end_hash):
    process = Popen(['git', 'diff',  start_hash, end_hash],
                    cwd=directory,
                    text=True, stdout=PIPE, stderr=DEVNULL)

    diff = process.stdout.read()
    diff_lines = diff.split('\n')
    diff_summary = {}
    file_starts = [idx for idx, line in enumerate(diff_lines) if line.startswith('diff')]
    for file_start, file_end in zip(file_starts, file_starts[1:]):
        file_name = diff_lines[file_start].split()[3]
        changes_start = _find_changes_start(diff_lines[file_start:file_end], file_start)
        file_lines = diff_lines[changes_start:file_end]

        diff_summary[file_name] = file_lines

    last_file_name = diff_lines[file_starts[-1]].split()[3]
    changes_start = _find_changes_start(diff_lines[file_starts[-1]:], file_starts[-1])
    last_file_lines = diff_lines[changes_start:]
    diff_summary[last_file_name] = last_file_lines

    return diff_summary


def _find_changes_start(file_lines, file_start):
    for idx, line in enumerate(file_lines):
        if line.startswith('+++'):
            return idx + 1 + file_start


if __name__ == "__main__":
    path = Path('/home/jmilam/git_repos/milam-notes')
    start_hash = 'd10ce687c679cf0a419a30e2793bd2cc26e67610'
    end_hash = '55d4f6722a27169d5f2ea6a929e4f116eb03039e'
    GitDiffByFile(git_diff(path, start_hash, end_hash)).run()
