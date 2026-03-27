from textual.app import App
from textual.containers import Horizontal, VerticalScroll
from textual.widgets import Button, ContentSwitcher, DataTable, Markdown


class ContentSwitcherExample(App):
    CSS = """
    Screen {
        align: center middle;
        padding: 1;
    }

    #buttons {
        height: 3;
        width: auto;
    }

    ContentSwitcher {
        border: round $primary;
        width: 90%;
        height: 1fr;
    }

    MarkdownH2 {
        background: $panel;
        color: yellow;
        border: none;
        padding: 0 1;
    }
    """
    def compose(self):
        with Horizontal(id='buttons'):
            yield Button('DataTable', id='data-table')
            yield Button('Markdown', id='markdown')

        with ContentSwitcher(initial='data-table'):
            yield DataTable(id='data-table')
            with VerticalScroll(id='markdown'):
                yield Markdown('SOOOO `much` Markdown **text**!')

    def on_button_pressed(self, event):
        self.query_one(ContentSwitcher).current = event.button.id

    def on_mount(self):
        dt = self.query_one(DataTable)
        dt.add_columns('col 1', 'col 2', 'col 3')
        dt.add_rows([('cell 11', 'cell 12', 'cell 13'), ('cell 21', 'cell 22', 'cell 23')])


if __name__ == "__main__":
    ContentSwitcherExample().run()
