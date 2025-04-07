from tenacity import retry, stop_after_attempt, wait_fixed
import httpx
import m3u8
from client import fetch


class M3U8Downloader:
    def __init__(
        self, http_client: httpx.AsyncClient, http_dl_client: httpx.AsyncClient
    ):
        self.http_client = http_client
        self.http_dl_client = http_dl_client

    async def get_playlist_url(self, hls_url: str) -> str:
        m3u8_obj = await self.fetch_m3u8(self.http_client, hls_url)
        base_url = m3u8_obj.base_uri
        playlists = m3u8_obj.data["playlists"]

        playlists = sorted(
            playlists, key=lambda x: x["stream_info"]["bandwidth"], reverse=True
        )

        return base_url + playlists[0]["uri"]

    async def get_stream_urls(self, playlist_url: str) -> list[str]:
        m3u8_obj = await self.fetch_m3u8(self.http_client, playlist_url)
        base_url = m3u8_obj.base_uri
        streams = m3u8_obj.data["segments"]
        return [base_url + stream["uri"] for stream in streams]

    async def fetch_m3u8(
        self, client: httpx.AsyncClient, playlist_url: str
    ) -> m3u8.M3U8:
        response = await fetch(client, playlist_url)
        response.raise_for_status()
        return m3u8.loads(response.text, uri=playlist_url)

    @retry(stop=stop_after_attempt(10), wait=wait_fixed(3))
    async def download_stream(
        self,
        idx: int,
        url: str,
        filename: str,
        output_path: str,
    ) -> str:
        async with self.http_dl_client.stream("GET", url) as response:
            path = output_path + filename
            with open(path, "wb") as f:
                async for chunk in response.aiter_bytes():
                    f.write(chunk)

        return path
