from textual.app import App
from textual.widgets import Static


class NotifySimpleApp(App):
    BINDINGS = [('a', 'notify_1', 'Notify 1'),
                ('b', 'notify_2', 'Notify 2'),
                ('c', 'notify_3', 'Notify 3')]

    def compose(self):
        yield Static("Hello World")

    def action_notify_1(self):
        self.notify('Notify 1')

    def action_notify_2(self):
        self.notify('Notify 2')

    def action_notify_3(self):
        self.notify('Notify 3')


if __name__ == "__main__":
    NotifySimpleApp().run()
