from models import Episode
import subprocess
from mutagen.mp4 import MP4, MP4Cover
from client import fetch
import os
import httpx


class AudioProcessor:
    def __init__(self, http_dl_client: httpx.AsyncClient):
        self.http_dl_client = http_dl_client

    async def add_metadata_to_file(self, ep: Episode, file_path: str):
        audio = MP4(file_path)
        audio["\xa9nam"] = ep.title
        audio["\xa9alb"] = ep.show_title
        audio["\xa9ART"] = ep.author
        audio["\xa9cmt"] = ep.description
        audio["trkn"] = [(ep.no, 0)]
        audio["\xa9lng"] = ep.language
        resp = await fetch(self.http_dl_client, ep.cover)
        img_data = resp.read()
        audio["covr"] = [MP4Cover(img_data, imageformat=MP4Cover.FORMAT_PNG)]
        audio.save()

    async def merge_files_ffmpeg(
        self, stream_paths: list[str], output_file: str, output_path: str
    ) -> str:
        merged_file = os.path.join(output_path, output_file)

        list_file = os.path.join(output_path, "inputs.txt")
        with open(list_file, "w") as f:
            for path in stream_paths:
                f.write(f"file '{os.path.abspath(path)}'\n")

        subprocess.run(
            [
                "ffmpeg",
                "-f",
                "concat",
                "-safe",
                "0",
                "-i",
                list_file,
                "-c",
                "copy",
                merged_file,
            ],
            check=True,
            capture_output=True,
            text=True,
        )

        os.remove(list_file)
        for path in stream_paths:
            os.remove(path)

        return merged_file
