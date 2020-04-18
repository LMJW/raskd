import requests
import json

url = "http://localhost:22022"
# test get index
a = 'abc'
res = requests.get(f"{url}/{1}/{a}")
print(res)

# test post task
data = {
    "create": {
        "name": "haha",
        "tasktype": "abc",
    }
}
res = requests.post(f"{url}/task", json=data)
print(res.status_code, res.content)

# test the get task
res = requests.get(f"{url}/task")
print(res.status_code, res.content)
