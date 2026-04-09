from flask import (
    Flask,
    flash,
    redirect,
    render_template,
    request,
    url_for
)

app = Flask(__name__)

# For a real app, do not keep the key in source code!
app.secret_key = b'_5#y2nAF4Q8z\n\xec]/'

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/login', methods=['GET', 'POST'])
def login():
    if request.method == 'POST':
        flash('You were successfully logged in!')
        return redirect(url_for('index'))

    return render_template('login.html')
