from pathlib import Path

from textual.app import App
from textual.containers import Horizontal
from textual.widgets import Static, Tree


class FilePreviewApp(App):
    CSS_PATH = 'file-preview.tcss'
    def compose(self):
        tree = Tree("Current Working Directory")
        self.populate_tree(tree.root, Path.cwd())
        tree.root.expand()
        yield Horizontal(tree)

    def populate_tree(self, node, path):
        for dir_obj in path.iterdir():
            if dir_obj.is_dir():
                subnode = node.add(str(dir_obj))
                self.populate_tree(subnode, dir_obj)
            else:
                node.add_leaf(str(dir_obj))

    def on_tree_node_selected(self, event):
        statics = self.query(Static)
        if len(statics) == 1:
            statics.last().remove()
            return

        path = Path(event.node.label.plain)
        if path.is_file():
            self.query_one(Horizontal).mount(FilePreviewScreen(path.read_text()))


class FilePreviewScreen(Static):
    pass

if __name__ == "__main__":
    FilePreviewApp().run()
