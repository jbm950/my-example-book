from flask import Flask, request
from markupsafe import escape

app = Flask(__name__)


@app.post('/endpt')
def endpt():
    return f"""
Player: {request.form["player"]}
Class: {request.form["class"]}
Hitpoints: {request.args.get("hp", "0")}
"""
