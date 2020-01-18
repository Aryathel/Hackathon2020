import requests

res = requests.get('https://flight-engine-hack-2020.herokuapp.com/flights?date=2020-01-17')
print(res.status_code)
print(res.text)
