from pathlib import Path
import requests
import tomllib


def main():
    creds = tomllib.loads(Path('creds').read_text())
    response = requests.post(
        'https://accounts.spotify.com/api/token',
        headers={'Content-Type': 'application/x-www-form-urlencoded'},
        data=f'grant_type=client_credentials&client_id={creds["client_id"]}&client_secret={creds["client_secret"]}'
        )
    print(response.json())


if __name__ == "__main__":
    main()
