from flask import Flask
from markupsafe import escape

app = Flask(__name__)


@app.route('/user/<username>')  # No type converter called out
def show_user_profile(username):
    return f'User: {escape(username)}'


@app.route('/post/<int:post_id>')  # Converts to int
def show_post(post_id):
    return f'Post: {post_id}'


@app.route('/path/<path:subpath>')  # Converts to Path? Might just be str
def show_subpath(subpath):
    return f'Subpath {escape(subpath)}'
