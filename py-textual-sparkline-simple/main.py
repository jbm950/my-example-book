from textual.app import App
from textual.widgets import Sparkline


class SparklineApp(App):
    value = 1

    def compose(self):
        yield Sparkline(data=[], summary_function=max)

    def on_mount(self):
        self.set_interval(1 / 2, self.add_data)

    def add_data(self):
        plot = self.query_one(Sparkline)

        new_data = plot.data + [self.value]
        new_data = new_data[-50:]  # Only keep 50 data points

        plot.data = new_data  # Mutate doesn't work, you have to reassign data

        if self.value == 5:
            self.value = 1
        else:
            self.value += 1


if __name__ == "__main__":
    SparklineApp().run()
