from textual.app import App
from textual_plotext import PlotextPlot


class PlotextExampleApp(App):
    def compose(self):
        yield PlotextPlot()

    def on_mount(self):
        plt = self.query_one(PlotextPlot).plt
        x = [i * 0.01 for i in range(-500, 501)]
        y = [i ** 2 for i in x]
        plt.plot(x, y)
        plt.title('Quadratic')


if __name__ == "__main__":
    PlotextExampleApp().run()
