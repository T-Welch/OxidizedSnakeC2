from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()

class HostSystem(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    cpu_vendor = db.Column(db.String(255))
    ip_address = db.Column(db.String(255))
    ip_prefix = db.Column(db.Integer)
    network_count = db.Column(db.Integer)