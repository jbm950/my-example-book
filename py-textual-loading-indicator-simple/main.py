from textual.app import App
from textual.widgets import LoadingIndicator


class LoadingIndicatorExampleApp(App):
    def compose(self):
        yield LoadingIndicator()


if __name__ == "__main__":
    LoadingIndicatorExampleApp().run()
