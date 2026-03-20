import requests


def main():
    player = 'Salvsis2'
    hiscores_base_url = 'https://secure.runescape.com/m=hiscore_oldschool/index_lite.json?player='
    resp = requests.get(hiscores_base_url + player)
    print(resp)
    print(resp.json())


if __name__ == "__main__":
    main()
