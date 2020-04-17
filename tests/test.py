import requests
import json

url = "http://localhost:22022"
# test get index
a = 'abc'
res = requests.get(f"{url}/{1}/{a}")
print(res)

# test post task
data = {
    "Task": {
        "id": 1,
        "name": "haha",
        "tasktype": "abc"
    }
}

res = requests.post(f"{url}/task", json=json.dumps(data))
print(res.status_code, res.content)
