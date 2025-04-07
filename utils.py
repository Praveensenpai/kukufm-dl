import os
import shutil


def human_readable_size(size: float, decimal_places: int = 2) -> str:
    unit = "?"
    for unit in ["B", "KB", "MB", "GB", "TB"]:
        if size < 1024.0:
            break
        size /= 1024.0
    return f"{size:.{decimal_places}f} {unit}"


def make_dirs(path: str) -> None:
    os.makedirs(path, exist_ok=True)


def delete_all_temp_folders(root_dir: str) -> None:
    for dirpath, dirnames, _ in os.walk(root_dir):
        for dirname in dirnames:
            if dirname.lower() == "temp":
                temp_path = os.path.join(dirpath, dirname)
                shutil.rmtree(temp_path)
