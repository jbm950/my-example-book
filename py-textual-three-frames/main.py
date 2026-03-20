from textual.app import App
from textual.containers import Horizontal, Vertical
from textual.widgets import Static


class ThreeFramesApp(App):
    """Class shows the same effect 3 different ways: using Horizontal and
    Vertical containers, using those containers in context managers, and using
    a grid layout. The 3 methods can be mix and matched."""
    CSS_PATH = 'three_frames.tcss'

    def compose(self):
        # It stops at the first non container it sees so you have to wrap the
        # first column in a Vertical object.
        yield Horizontal(
            Vertical(Static('Frame 1')),
            Vertical(
                Static('Frame 2'),
                Static('Frame 3')
                )
            )

    # def compose(self):
    #     with Horizontal():
    #         with Vertical(classes='column'):
    #             yield Static('Frame 1')
    #         with Vertical(classes='column'):
    #             yield Static('Frame 2')
    #             yield Static('Frame 3')

    # CSS_PATH = 'three_frames-grid.tcss'
    # def compose(self):
    #     yield Static('Frame 1', classes='box', id='one')
    #     yield Static('Frame 2', classes='box')
    #     yield Static('Frame 3', classes='box')



if __name__ == "__main__":
    ThreeFramesApp().run()
