import requests

base_url = 'http://127.0.0.1:5000'

response = requests.post(base_url + '/endpt', data={'player': 'GoodGuy',
                                                    'class': 'Paladin'})
print(response.text)

response = requests.post(base_url + '/endpt?hp=35', data={'player': 'BadGuy',
                                                          'class': 'Rogue'})
print(response.text)
