import httpx
from models import Episode, Show
from rich import print


class KuKuFM:
    def __init__(self, http_dl_client: httpx.AsyncClient):
        self.http_dl_client = http_dl_client

    async def get_show(self, show_slug: str) -> Show:
        url = f"https://kukufm.com/api/v2.1/channels/{show_slug}/episodes?lang=english&page=1"
        resp = await self.http_dl_client.get(url, follow_redirects=True)
        if resp.status_code == 404:
            raise ValueError(f"Show/Slug {show_slug} not found")
        resp_json = resp.json()
        show_title = resp_json["show"]["title"]
        description = resp_json["show"]["description"]
        author_name = resp_json["show"]["author"]["name"]
        language = resp_json["show"]["language"]
        n_pages = resp_json["n_pages"]

        return Show(
            title=show_title,
            description=description,
            author_name=author_name,
            language=language,
            n_pages=n_pages,
        )

    async def get_episodes(self, show_slug: str, from_ep: int, to_ep: int):
        print("Fetching episodes...")
        per_page_ep = 10
        current_page = ((from_ep - 1) // per_page_ep) + 1

        show = await self.get_show(show_slug)

        print()
        print("[blue]Show Title: {} [/blue]".format(show.title))
        print("[blue]Author: {} [/blue]".format(show.author_name))
        print("[blue]Description: {} [/blue]".format(show.description))
        print("[blue]Language: {} [/blue]".format(show.language))
        print("[blue]Estimated Episodes : ~{} [/blue]".format(show.n_pages * 10))
        print()
        while True:
            url = f"https://kukufm.com/api/v2.1/channels/{show_slug}/episodes?lang=english&page={current_page}"
            resp = await self.http_dl_client.get(url, follow_redirects=True)

            if resp.status_code == 404:
                raise ValueError(f"Show/Slug {show_slug} not found")

            data = resp.json()
            episodes = data.get("episodes", [])

            if not episodes:
                break

            for episode in episodes:
                ep_number = episode["index"]
                if ep_number < from_ep:
                    continue

                if to_ep and ep_number > to_ep:
                    return

                yield Episode(
                    show_title=show.title,
                    title=episode["title"],
                    no=ep_number,
                    duration_s=episode["duration_s"],
                    hls_url=episode["content"]["hls_url"],
                    cover=episode["image"],
                    author=show.author_name,
                    language=show.language,
                    description=show.description,
                )

            if current_page >= data.get("n_pages", current_page):
                break

            current_page += 1

    def get_show_slug(self, show_url: str):
        parts = show_url.strip().split("/")
        try:
            return parts[parts.index("show") + 1]
        except (ValueError, IndexError):
            raise ValueError(f"Show/Slug not found in URL {show_url}")
