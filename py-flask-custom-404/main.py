from flask import Flask

app = Flask(__name__)


@app.errorhandler(404)
def my_404(error):
    return 'My Error handler. 404! Oh noes!', 404
