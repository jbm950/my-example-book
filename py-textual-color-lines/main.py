from textual.app import App
from textual.widgets import Static


class ColorLines(App):
    def compose(self):
        yield Static('[$primary]Hello world[/] Non styled [$accent]Another![/]')


if __name__ == "__main__":
    ColorLines().run()
