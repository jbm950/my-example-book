from pathlib import Path
import time
import tomllib
from requests_oauthlib import OAuth2Session


def main():
    creds = tomllib.loads(Path('creds').read_text())
    scopes = ['user-modify-playback-state',
              'user-read-playback-state',
              'playlist-modify-public',
              'playlist-modify-private']

    redirect_uri = 'https://127.0.0.1:8888/callback'
    auth_base_url = 'https://accounts.spotify.com/authorize'
    token_url = 'https://accounts.spotify.com/api/token'

    oauth = OAuth2Session(creds['client_id'], scope=scopes, redirect_uri=redirect_uri)
    auth_url, state = oauth.authorization_url(auth_base_url)

    print("Go here and authorize:")
    print(auth_url)
    redirect_response = input("Paste the full redirect URL here: ")

    token = oauth.fetch_token(
        token_url,
        authorization_response=redirect_response,
        client_secret=creds['client_secret'])

    print("Access token obtained!")

    response = oauth.get("https://api.spotify.com/v1/me/player")

    print(response.json())

    oauth.put("https://api.spotify.com/v1/me/player/pause")
    time.sleep(3)
    oauth.put("https://api.spotify.com/v1/me/player/play")


if __name__ == "__main__":
    main()
