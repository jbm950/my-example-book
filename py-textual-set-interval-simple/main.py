from textual.app import App
from textual.containers import Horizontal, Vertical
from textual.reactive import reactive
from textual.widgets import Digits, Static


class SetIntervalApp(App):
    BINDINGS = [('s', 'create_frame', 'Create Frame')]
    digit = reactive(0)
    CSS = """
    Digits {
        height: 1fr;
        width: 1fr;
     }

     Static {
        height: 1fr;
        width: 1fr;
     }
    """

    def compose(self):
        yield Horizontal(
            Vertical(
                Digits(str(self.digit)),
                Static("Press s to open a second screen")
                ),
            Horizontal()
            )

    def on_mount(self):
        self.set_interval(1, self.increment)

    def action_create_frame(self):
        statics = self.query(Static)
        if len(statics) == 2:
            statics.last().remove()
            return

        self.query(Horizontal).last().mount(RightScreen('Added this screen'))

    def increment(self):
        self.digit += 1

    def watch_digit(self):
        for digit_widget in self.query(Digits):
            digit_widget.update(str(self.digit))


class RightScreen(Static):
    pass


if __name__ == "__main__":
    SetIntervalApp().run()
