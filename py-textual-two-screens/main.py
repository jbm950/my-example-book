from logging import basicConfig, getLogger

from textual.app import App
from textual.screen import Screen
from textual.widgets import Static

logger = getLogger()
basicConfig(filename='log.out')


class Screen1(Screen):
    def compose(self):
        yield Static('Screen 1')


class Screen2(Screen):
    def compose(self):
        yield Static('Screen 2')


class TopApp(App):
    BINDINGS = [("s", "toggle_screen()", "Toggle Screen")]
    SCREENS = {'screen-1': Screen1,
               'screen-2': Screen2}

    def on_mount(self):
        self.push_screen('screen-1')

    def action_toggle_screen(self):
        if isinstance(self.screen, Screen1):
            self.push_screen('screen-2')
        elif isinstance(self.screen, Screen2):
            self.push_screen('screen-1')


if __name__ == "__main__":
    TopApp().run()
