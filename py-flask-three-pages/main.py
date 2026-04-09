from flask import Flask, render_template

app = Flask(__name__)


@app.route('/')
def index():
    return render_template('index.html')


@app.route('/end_1')
def end_1():
    return render_template('end_1.html')


@app.route('/end_2')
def end_2():
    return render_template('end_2.html')
