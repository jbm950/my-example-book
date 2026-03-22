from textual.app import App
from textual.containers import Vertical
from textual.screen import ModalScreen
from textual.reactive import reactive
from textual.widgets import Digits, LoadingIndicator, Static


class PopUpModal(ModalScreen):
    digit = reactive(5)

    def compose(self):
        yield Vertical(
            Digits(str(self.digit)),
            LoadingIndicator()
        )

    def on_mount(self):
        self.set_interval(1, self.update_time)

    def update_time(self):
        self.digit -= 1
        if self.digit <= 0:
            self.app.pop_screen()

    def watch_digit(self):
        for digit_widget in self.query(Digits):
            digit_widget.update(str(self.digit))


class ModalScreenExample(App):
    BINDINGS = [('s', 'create_modal', 'Create the popup modal')]
    CSS = """
    PopUpModal {
        align: center middle;
    }

    Vertical {
        height: 10;
        width: 40;
        border: thick $background 80%;
        background: $surface;
    }

    Digits {
        width: 100%;
    }

    LoadingIndicator {
        width: 100%;
    }
    """

    def compose(self):
        yield Static('So many texts!')

    def action_create_modal(self):
        self.push_screen(PopUpModal())


if __name__ == "__main__":
    ModalScreenExample().run()
