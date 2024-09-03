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
        print(host_system)
        return jsonify({'status': 'success'}), HTTPStatus.CREATED
    except Exception as e:
        return jsonify({'error': str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR

@app.route('/client-hello', methods=['GET'])
def get_client_hello():
    host_systems = db.session.query(HostSystem).all()
    host_systems_list = [
        {
            "id": host_system.id,
            "cpu_vendor": host_system.cpu_vendor,
            "ip_address": host_system.ip_address,
            "ip_prefix": host_system.ip_prefix,
            "network_count": host_system.network_count
        }
        for host_system in host_systems
    ]
    return jsonify(host_systems_list), HTTPStatus.OK
    

if __name__ == '__main__':
    with app.app_context():
        db.create_all()    
    app.run(host='0.0.0.0', debug=True)