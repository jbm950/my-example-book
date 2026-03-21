from threading import Thread
from time import sleep

from textual.app import App
from textual.message import Message
from textual.widgets import Tree


class TreeWithMessage(Tree):
    count = 0
    threads = []

    class MyMessage(Message):
        def __init__(self, message_num):
            self.message_num = message_num
            super().__init__()

    def on_mount(self):
        self.send_message_timer = self.set_interval(1, self.send_message)

    def send_message(self):
        self.threads.append(Thread(target=self.send_message_thread))
        self.threads[-1].start()

    def send_message_thread(self):
        sleep(2)
        self.count += 1
        self.notify(f'Sending message {self.count}')
        self.post_message(self.MyMessage(self.count))

    def finish_threads(self):
        self.send_message_timer.pause()
        for thread in self.threads:
            thread.join()



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

    def exit(self):
        self.query_one(TreeWithMessage).finish_threads()
        super().exit()


if __name__ == "__main__":
    TopApp().run()
