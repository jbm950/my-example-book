from textual.app import App
from textual.widgets import DataTable

HEADERS = ['Number', 'Name', 'Type']
ROWS = [(1, 'Bulbasaur', 'Grass'),
        (4, 'Charmander', 'Fire'),
        (7, 'Squirtle', 'Water')]


class DataTableExample(App):
    def compose(self):
        table = DataTable()
        table.add_columns(*HEADERS)
        table.add_rows(ROWS)
        yield table


if __name__ == "__main__":
    DataTableExample().run()
