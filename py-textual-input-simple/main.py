from textual.app import App
from textual.widgets import Input


class InputExample(App):
    def compose(self):
        yield Input(type='text')
        yield Input(type='integer')
        yield Input(type='number')
        yield Input(password=True)

    def on_input_submitted(self, event):
        self.notify(str(event.value))


if __name__ == "__main__":
    InputExample().run()
