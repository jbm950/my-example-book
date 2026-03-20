from pathlib import Path

from textual.app import App
from textual.widgets import Tree


class SimpleTreeApp(App):
    def compose(self):
        tree = Tree("Current Working Directory")
        self.populate_tree(tree.root, Path.cwd())
        tree.root.expand()
        yield tree

    def populate_tree(self, node, path):
        for dir_obj in path.iterdir():
            if dir_obj.is_dir():
                subnode = node.add(str(dir_obj))
                self.populate_tree(subnode, dir_obj)
            else:
                node.add_leaf(str(dir_obj))

    def on_tree_node_selected(self, event):
        self.notify(event.node.label.plain)


if __name__ == "__main__":
    SimpleTreeApp().run()
