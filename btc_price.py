import requests
print("This is running inside docker and was built by the Rust code!")

response = requests.get("https://api.gemini.com/v2/ticker/btcusd")
print(response.content)
