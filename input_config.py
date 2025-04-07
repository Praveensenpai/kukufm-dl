from rich.console import Console
from rich.prompt import Prompt
from models import InputConf

console = Console()


def _get_valid_int(prompt_msg: str, allow_zero: bool = False) -> int:
    while True:
        value = Prompt.ask(prompt_msg)
        if not value.isdigit():
            console.print("[red]Enter a valid positive integer.[/red]")
            continue
        value = int(value)

        if value < 0:
            console.print("[red]Enter a valid positive integer.[/red]")
            continue

        if value == 0 and not allow_zero:
            console.print("[red]Zero is not allowed here.[/red]")
            continue
        return value


def _get_valid_url(prompt_msg: str) -> str:
    while True:
        url = Prompt.ask(prompt_msg)
        if "/show/" in url:
            return url
        console.print("[red]Invalid show URL. It must contain '/show/'.[/red]")


def get_config_from_user() -> InputConf:
    console.print("[bold cyan]Welcome to Kukufm Downloader[/bold cyan]")

    show_url = _get_valid_url("[bold green]Enter the show URL[/bold green]")

    while True:
        from_ep = _get_valid_int(
            "[bold blue]Enter the start episode number[/bold blue]"
        )
        to_ep = _get_valid_int(
            "[bold magenta]Enter the end episode number (0 for infinite)[/bold magenta]",
            allow_zero=True,
        )

        if from_ep < 1:
            console.print("[red]From Episode can't be less than 1.[/red]")
            continue

        if to_ep != 0 and from_ep > to_ep:
            console.print("[red]To Episode can't be less than From Episode.[/red]")
            continue

        if to_ep == 0:
            console.print(
                "[yellow]To Episode is set to 0 (infinity). It'll go on until it runs out.[/yellow]"
            )
        parallel_downloads = _get_valid_int(
            "[bold yellow]Enter the number of parallel downloads[/bold yellow]"
        )

        return InputConf(
            show_url=show_url,
            from_ep=from_ep,
            to_ep=to_ep,
            parallel_downloads=parallel_downloads,
        )
