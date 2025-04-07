import httpx
from tenacity import retry, stop_after_attempt, wait_fixed


def _get_cookies() -> dict[str, str]:
    with open("cookies.txt") as f:
        cookie_str = f.read()
    return dict(item.strip().split("=", 1) for item in cookie_str.split(";"))


def get_aclient() -> httpx.AsyncClient:
    return httpx.AsyncClient(
        headers={
            "accept": "*/*",
            "accept-language": "en",
            "dnt": "1",
            "priority": "u=1, i",
            "sec-ch-ua": '"Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133"',
            "sec-ch-ua-mobile": "?0",
            "sec-ch-ua-platform": '"Linux"',
            "sec-fetch-dest": "empty",
            "sec-fetch-mode": "cors",
            "sec-fetch-site": "same-origin",
            "user-agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36",
        },
        cookies=_get_cookies(),
        timeout=60 * 3,
        verify=False,
    )


def get_dl_aclient() -> httpx.AsyncClient:
    return httpx.AsyncClient(timeout=60 * 3)


@retry(stop=stop_after_attempt(10), wait=wait_fixed(3))
async def fetch(client: httpx.AsyncClient, url: str) -> httpx.Response:
    return await client.get(url)
