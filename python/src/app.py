from flask import Flask, request, jsonify, redirect, url_for, render_template

app = Flask(__name__, template_folder='../templates', static_folder='../static')

@app.route('/')
def index():
    return render_template('index.html')



@app.route('/client-hello', methods=['POST'])
def client_hello():
    print("Client Hello")
    return jsonify({'status': 'success'}), 200
    

if __name__ == '__main__':
    app.run(host='0.0.0.0', debug=True)