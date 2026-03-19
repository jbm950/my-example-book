from textual.app import App
from textual.containers import Vertical
from textual.widgets import Collapsible, Digits, Static


class CollapsibleExampleApp(App):
    def compose(self):
        with Vertical():
            with Collapsible():
                yield Digits('12345')
            with Collapsible(title='Second expand'):
                yield Static('A hello world collapsible')


if __name__ == "__main__":
    CollapsibleExampleApp().run()
