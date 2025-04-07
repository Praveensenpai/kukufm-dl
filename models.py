from dataclasses import dataclass


@dataclass
class Episode:
    show_title: str
    title: str
    no: int
    duration_s: int
    hls_url: str
    cover: str
    author: str
    language: str
    description: str


@dataclass
class InputConf:
    show_url: str
    from_ep: int
    to_ep: int
    parallel_downloads: int


@dataclass
class Show:
    title: str
    description: str
    author_name: str
    language: str
    n_pages: int
