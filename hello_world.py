import requests
print("This is running inside docker and was built by the Rust code!")

response = requests.get("https://example.com")
print(response.content[:351])
