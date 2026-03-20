from textual.app import App
from textual.reactive import reactive
from textual.widgets import Digits


class DigitsExampleApp(App):
    BINDINGS = [('a', 'add_digit', 'Add digit'),
                ('s', 'subtract_digit', 'Subtract digit')]
    digit = reactive(0)

    def compose(self):
        yield Digits(str(self.digit))

    def action_add_digit(self):
        self.digit += 1

    def action_subtract_digit(self):
        self.digit -= 1

    def watch_digit(self):
        for digit_widget in self.query(Digits):
            digit_widget.update(str(self.digit))


if __name__ == "__main__":
    DigitsExampleApp().run()
