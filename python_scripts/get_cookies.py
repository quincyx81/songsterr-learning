import browser_cookie3
import json

cookies = browser_cookie3.firefox(domain_name="songsterr.com")

cookies_dict = [
    {
        "name": cookie.name,
        "value": cookie.value,
        "domain": cookie.domain,
        "path": cookie.path,
        "secure": cookie.secure,
        "expires": cookie.expires,
    }
    for cookie in cookies
]

print(json.dumps(cookies_dict, indent=4))

