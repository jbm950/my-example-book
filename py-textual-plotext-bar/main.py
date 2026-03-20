import random

from textual.app import App
from textual_plotext import PlotextPlot


class PlotextExampleApp(App):
    def compose(self):
        yield PlotextPlot()

    def on_mount(self):
        plt = self.query_one(PlotextPlot).plt
        categories = ['apple', 'banana', 'peach', 'pear']
        values = [int(random.random() * 10) for _ in range(len(categories))]
        plt.bar(categories, values, orientation='horizontal', width=3 / 5)
        plt.title('Fruit Stats')


if __name__ == "__main__":
    PlotextExampleApp().run()
