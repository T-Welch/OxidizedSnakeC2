from flask import Flask, request, jsonify, redirect, url_for, render_template
from http import HTTPStatus
from models import db, HostSystem


app = Flask(__name__, template_folder='../templates', static_folder='../static')
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///dataschmace.db'
db.init_app(app)

@app.route('/')
def index():
    return render_template('index.html')



@app.route('/client-hello', methods=['POST'])
def client_hello():
    data = request.get_json()
    if not data:
        return jsonify({'error': 'Invalid JSON'}), HTTPStatus.BAD_REQUEST

    try:
        host_system = HostSystem(
            cpu_vendor=data.get('cpu_vendor'),
            ip_address=data.get('ip_address'),
            ip_prefix=data.get('ip_prefix'),
            network_count=data.get('network_count')
        )
        db.session.add(host_system)
        db.session.commit()
        return jsonify({'status': 'success'}), HTTPStatus.CREATED
    except Exception as e:
        return jsonify({'error': str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR


@app.route('/client-hello', methods=['GET'])
def get_client_hello():
    print(db.session.query(HostSystem).all())
    host_system = db.session.query(HostSystem).filter(HostSystem.id == 1).first()
    if host_system:
        print(f"CPU Vendor: {host_system.cpu_vendor}")
        print(f"IP Address: {host_system.ip_address}")
        print(f"IP Prefix: {host_system.ip_prefix}")
        print(f"Network Count: {host_system.network_count}")
    else:
        print("Host system not found")
    return "Okay then", HTTPStatus.METHOD_NOT_ALLOWED
    

if __name__ == '__main__':
    with app.app_context():
        db.create_all()    
    app.run(host='0.0.0.0', debug=True)