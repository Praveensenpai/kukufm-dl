import asyncio
from typing import Final

import typer
from typer import Option

from client import get_aclient, get_dl_aclient
from utils import make_dirs, delete_all_temp_folders
from downloader import KuKuFMDownloader
from models import InputConf

app = typer.Typer()

@app.command()
def kukufm(
    url: str = Option(..., "--url", help="Show URL"),
    from_ep: int = Option(..., "--from-ep", help="Start episode (>= 1)"),
    to_ep: int = Option(..., "--to-ep", help="End episode (0 for infinite)"),
    parallel_downloads: int = Option(..., "--parallel-downloads", help="Number of parallel downloads"),
):
    if "/show/" not in url:
        raise typer.BadParameter("URL must contain '/show/'.")
    if from_ep < 1:
        raise typer.BadParameter("From episode must be at least 1.")
    if to_ep < 0:
        raise typer.BadParameter("To episode must be 0 or more.")
    if to_ep != 0 and from_ep > to_ep:
        raise typer.BadParameter("To episode can't be less than from episode.")
    if parallel_downloads < 1:
        raise typer.BadParameter("Parallel downloads must be at least 1.")

    config = InputConf(
        show_url=url,
        from_ep=from_ep,
        to_ep=to_ep,
        parallel_downloads=parallel_downloads,
    )

    asyncio.run(start_download(config))


async def start_download(config: InputConf):
    DOWNLOAD_PATH: Final[str] = "downloads/"
    make_dirs(DOWNLOAD_PATH)
    delete_all_temp_folders(DOWNLOAD_PATH)
    http_client = get_aclient()
    http_dl_client = get_dl_aclient()
    kukufm_dl = KuKuFMDownloader(
        conf=config,
        http_client=http_client,
        http_dl_client=http_dl_client,
        download_path=DOWNLOAD_PATH,
    )
    await kukufm_dl.download()


if __name__ == "__main__":
    app()
