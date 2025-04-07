import httpx
from rich import print
import asyncio
import os

from utils import (
    make_dirs,
)
from models import Episode, InputConf
from kukufm import KuKuFM
from m3u8_downloader import M3U8Downloader
from audio_processor import AudioProcessor
from utils import human_readable_size


class KuKuFMDownloader:
    def __init__(
        self,
        conf: InputConf,
        http_client: httpx.AsyncClient,
        http_dl_client: httpx.AsyncClient,
        download_path: str,
    ):
        self.conf = conf
        self.kukufm = KuKuFM(http_client)
        self.m3u8_dl = M3U8Downloader(http_client, http_dl_client)
        self.audio_processor = AudioProcessor(http_dl_client)
        self.download_path = download_path

    async def download_episode(self, ep: Episode):
        print(f"Downloading episode {ep.title}")
        playlist_url = await self.m3u8_dl.get_playlist_url(ep.hls_url)

        tasks = []
        stream_urls = await self.m3u8_dl.get_stream_urls(playlist_url)

        show_dir = self.download_path + ep.show_title + "/"
        make_dirs(show_dir)

        temp_dir = show_dir + "temp/"
        make_dirs(temp_dir)
        episode_file = f"{ep.show_title} - {ep.title}.m4a"

        for idx, url in enumerate(stream_urls):
            file_name = url.split("/")[-1]
            tasks.append(
                self.m3u8_dl.download_stream(
                    idx,
                    url,
                    file_name,
                    temp_dir,
                )
            )

        stream_paths = await asyncio.gather(*tasks)

        ep_file_dir = await self.audio_processor.merge_files_ffmpeg(
            stream_paths, episode_file, show_dir
        )
        file_size = os.path.getsize(ep_file_dir)
        print(f"Downloaded - {episode_file} ({human_readable_size(file_size)})")
        print(f"Adding metadata to {episode_file}")
        await self.audio_processor.add_metadata_to_file(ep, ep_file_dir)
        print(f"Metadata added to {episode_file}")

    async def download(self):
        show_slug = self.kukufm.get_show_slug(self.conf.show_url)
        batch: list[asyncio.Task] = []

        async for ep in self.kukufm.get_episodes(
            show_slug,
            from_ep=self.conf.from_ep,
            to_ep=self.conf.to_ep,
        ):
            task = asyncio.create_task(self.download_episode(ep))
            batch.append(task)

            if len(batch) >= self.conf.parallel_downloads:
                await asyncio.gather(*batch)
                batch.clear()
        if batch:
            await asyncio.gather(*batch)
