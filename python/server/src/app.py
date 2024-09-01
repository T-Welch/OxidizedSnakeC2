from flask import Flask, request, jsonify, redirect, url_for, render_template
from http import HTTPStatus
app = Flask(__name__, template_folder='../templates', static_folder='../static')

@app.route('/')
def index():
    return render_template('index.html')



@app.route('/client-hello', methods=['POST'])
def client_hello():
    data = request.get_json()
    print(data)
    return jsonify({'status': 'success'}), 200

@app.route('/client-hello', methods=['GET'])
def get_client_hello():
    print("No gettin Client Hello")
    return "Okay then", HTTPStatus.METHOD_NOT_ALLOWED
    

if __name__ == '__main__':
    app.run(host='0.0.0.0', debug=True)