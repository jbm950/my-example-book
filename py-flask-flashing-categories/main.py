from flask import Flask, flash, redirect, render_template, url_for

app = Flask(__name__)

# For a real app, do not keep the key in source code!
app.secret_key = b'_5#72nAF4Q8z\n\xec]/'


@app.route('/')
def index():
    return render_template('index.html')


@app.route('/warn')
def just_warnings():
    flash('Warning 1', 'warning')
    flash('Oh my gosh this is getting deprecated!', 'warning')
    return redirect(url_for('index'))


@app.route('/warn_and_error')
def warn_and_error():
    flash('Warning 2', 'warning')
    flash('You did it wrong', 'error')
    flash('This was also wrong', 'error')
    return redirect(url_for('index'))
