import requests

headers = {
    "authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlcyI6W10sImlzcyI6Im1lLmZseSIsImRlZmF1bHRUcmF2ZWxlclBlcnNvbklkIjoiMTEyIiwidXNlcklkIjoiNTU1YjlhYTEtMDYzMC00NWQxLWFiMTktNWU2YTY1YjJmZGE1IiwicHVibGljX2V4cGlyZXNfZW0iOiIxNTc5NDA3NDk2NjczIiwidXNlckNvbXBhbnlKc29uIjoie1wiY29tcGFueUlkXCI6MzksXCJtZW1iZXJJZFwiOjE2NixcImVuYWJsZWRNZW1iZXJcIjp0cnVlLFwiYWRtaW5pc3RyYXRvclwiOmZhbHNlLFwiZW5hYmxlZENvbXBhbnlcIjp0cnVlLFwiY29udHJhY3RTaWduZWRcIjp0cnVlfSIsImF1dGgwSWQiOiJnb29nbGUtb2F1dGgyfDExMDQzODM3MTkzNjU4MDI5NTg5NiIsInByb2ZpbGVJZCI6IjkzIiwic2NvcGUiOiJmbHltZWFwaWRldiIsImRiSWQiOiI5MyIsImV4cCI6MTU4MTkxMzA5NiwiaWF0IjoxNTc5MzIxMDk2LCJlbWFpbCI6ImhvdWdodG9uYXdlQGdtYWlsLmNvbSJ9.6AQD77zzDSf9E3FD_f6gjcMRskCo_0XueGkNw9QRwXM",
    "content-type": "application/json"
}

query = """
{
  profile {
    accountDetails {
      id, accountEmail, displayName
    }
    companyMembership {
      companyId, companyName, memberId, companyAdmin
    }
  }
}
"""

s = requests.post("https://dev.fly.me/api/graphql", json = {'query': query} , headers = headers)
print(s.status_code)
print(s.text)
