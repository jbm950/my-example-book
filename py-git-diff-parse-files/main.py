from pathlib import Path
from subprocess import DEVNULL, PIPE, Popen


def main(directory, start_hash, end_hash):
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

    print(list(diff_summary.keys()))


def _find_changes_start(file_lines, file_start):
    for idx, line in enumerate(file_lines):
        if line.startswith('+++'):
            return idx + 1 + file_start


if __name__ == "__main__":
    path = Path('/home/jmilam/git_repos/milam-notes')
    start_hash = '37f7d61e0e6c1318f086e1ea1002ac9d34b94b52'
    end_hash = '5ac14a866a1d07967c09345fee1792e2f22495e0'
    main(path, start_hash, end_hash)
