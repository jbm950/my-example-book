from pathlib import Path

from textual.app import App
from textual.containers import Horizontal
from textual.widgets import Static, Tree


class FilePreviewApp(App):
    CSS = """
    Static {
        height: 1fr;
        width: 1fr;
        border: $accent;
    }

    Tree {
        height: 1fr;
        width: 1fr;
    }
    """

    def compose(self):
        tree = Tree("Current Working Directory")
        self.populate_tree(tree.root, Path.cwd())
        tree.root.expand()
        yield Horizontal(tree, Static())

    def populate_tree(self, node, path):
        for dir_obj in path.iterdir():
            if dir_obj.is_dir():
                subnode = node.add(str(dir_obj))
                self.populate_tree(subnode, dir_obj)
            else:
                node.add_leaf(str(dir_obj))

    def on_tree_node_highlighted(self, event):
        static = self.query_one(Static)

        path = Path(event.node.label.plain)
        if path.is_file():
            try:
                static.update(path.read_text())
                return
            except Exception:
                pass

        static.update('')


if __name__ == "__main__":
    FilePreviewApp().run()
