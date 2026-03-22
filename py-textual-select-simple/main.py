from textual.app import App
from textual.widgets import Header, Select

class DummyObj:
    def __init__(self, name):
        self.name = name


SELECT_2_VALUES = ['2 Option 1', '2 Option 2', '2 Option 3']


class SelectExample(App):
    def compose(self):
        yield Header()
        yield Select([('Text 1.1', 'Text 1.2'),
                      ('Text 2.1', 'Text 2.2'),
                      ('Text 3.1', DummyObj('object 3'))])
        yield Select.from_values(SELECT_2_VALUES)

    def on_select_changed(self, event):
        if isinstance(event.value, str):
            self.notify(event.value)
        elif isinstance(event.value, DummyObj):
            self.notify(event.value.name)


if __name__ == "__main__":
    SelectExample().run()
