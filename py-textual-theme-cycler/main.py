from textual.app import App
from textual.widgets import Static


class ThemeCyclerApp(App):
    BINDINGS =[('t', 'cycle_theme', 'Cycle Theme')]
    theme_list = ['nord', 'tokyo-night', 'textual-dark']
    current_theme_idx = 0

    def compose(self):
        yield Static('Some good text, like really guys')

    def on_mount(self):
        self.theme = self.theme_list[self.current_theme_idx]

    def action_cycle_theme(self):
        self.current_theme_idx += 1
        if self.current_theme_idx == len(self.theme_list):
            self.current_theme_idx = 0

        self.theme = self.theme_list[self.current_theme_idx]



if __name__ == "__main__":
    ThemeCyclerApp().run()
