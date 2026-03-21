from textual.app import App
from textual.message import Message
from textual.widgets import Tree


class TreeWithMessage(Tree):
    BINDINGS = [('s', 'send_message', 'Send a message')]
    count = 0

    class MyMessage(Message):
        def __init__(self, message_num):
            self.message_num = message_num
            super().__init__()

    def action_send_message(self):
        self.count += 1
        self.notify(f'Sending message {self.count}')
        self.post_message(self.MyMessage(self.count))


class TopApp(App):
    def compose(self):
        yield TreeWithMessage('good label')

    def on_tree_with_message_my_message(self, msg):
        self.notify(f'Recieved message {msg.message_num}')


if __name__ == "__main__":
    TopApp().run()
