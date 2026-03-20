from textual.app import App
from textual.reactive import reactive
from textual.widgets import ProgressBar


class ProgressBarApp(App):
    CSS = """
    ProgressBar {
        content-align: center middle;
    }
    """

    def compose(self):
        yield ProgressBar(total=100)

    def on_mount(self):
        self.set_interval(1 / 10, self.make_progress)

    def make_progress(self):
        self.query_one(ProgressBar).advance(1)


if __name__ == "__main__":
    ProgressBarApp().run()
