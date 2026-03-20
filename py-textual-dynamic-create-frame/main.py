from textual.app import App
from textual.containers import Horizontal
from textual.widgets import Static


class DynamicCreateFrameApp(App):
    BINDINGS = [('s', 'create_frame', 'Create Frame')]
    CSS_PATH = 'dynamic-create-frame.tcss'

    def compose(self):
        yield Horizontal(
            Static("Press s to open a second screen")
            )

    def action_create_frame(self):
        statics = self.query(Static)
        if len(statics) == 2:
            statics.last().remove()
            return

        self.query_one(Horizontal).mount(RightScreen('Added this screen'))


class RightScreen(Static):
    pass


if __name__ == "__main__":
    DynamicCreateFrameApp().run()
