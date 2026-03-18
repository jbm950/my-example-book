from pathlib import Path
from subprocess import DEVNULL, PIPE, Popen


def main(directory, start_hash, end_hash):
    process = Popen(['git', 'diff',  start_hash, end_hash],
                    cwd=directory,
                    text=True, stdout=PIPE, stderr=DEVNULL)

    print(process.stdout.read())


if __name__ == "__main__":
    path = Path('/home/jmilam/git_repos/milam-notes')
    start_hash = '5ac14a866a1d07967c09345fee1792e2f22495e0'
    end_hash = '37f7d61e0e6c1318f086e1ea1002ac9d34b94b52'
    main(path, start_hash, end_hash)
