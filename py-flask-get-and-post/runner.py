import requests

base_url = 'http://127.0.0.1:5000'

response = requests.get(base_url + '/end1')
print(response.text)
response = requests.post(base_url + '/end1')
print(response.text)

response = requests.get(base_url + '/end2')
print(response.text)
response = requests.post(base_url + '/end2')
print(response.text)
