from asyncio import sleep

from textual.app import App
from textual.message import Message
from textual.widgets import Tree


class TreeWithMessage(Tree):
    count = 0

    class MyMessage(Message):
        def __init__(self, message_num):
            self.message_num = message_num
            super().__init__()

    def on_mount(self):
        self.set_interval(1, self.send_message)

    async def send_message(self):
        await sleep(2)  # When done with time.sleep, the application locks up
        self.count += 1
        self.notify(f'Sending message {self.count}')
        self.post_message(self.MyMessage(self.count))


class TopApp(App):
    def compose(self):
        tree = TreeWithMessage('good label')
        for i in range(20):
            node = tree.root.add(f'Node {i}')
            for j in range(3):
                node.add_leaf(f'Leaf {j}')

        yield tree

    def on_tree_with_message_my_message(self, msg):
        self.notify(f'Recieved message {msg.message_num}')


if __name__ == "__main__":
    TopApp().run()
