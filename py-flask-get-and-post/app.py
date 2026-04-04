from flask import Flask, request

app = Flask(__name__)

@app.route('/end1', methods=['GET', 'POST'])
def end1():
    if request.method == 'POST':
        return 'Called end1 POST'
    else:
        return 'Called end1 GET'

@app.get('/end2')
def end2_get():
    return 'Called end2 GET'

@app.post('/end2')
def end2_post():
    return 'Called end2 POST'
