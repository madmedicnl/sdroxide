#!/usr/bin/env python3
"""Find the ATS Mini by its control port (60000) on the local /24."""
import concurrent.futures as cf
import ipaddress
import socket
import sys

PORT = int(sys.argv[2]) if len(sys.argv) > 2 else 60000
net = ipaddress.ip_network(sys.argv[1] if len(sys.argv) > 1 else "192.168.1.0/24", strict=False)


def probe(ip):
    s = socket.socket()
    s.settimeout(0.35)
    try:
        s.connect((str(ip), PORT))
        return str(ip)
    except OSError:
        return None
    finally:
        s.close()


with cf.ThreadPoolExecutor(max_workers=128) as ex:
    for res in ex.map(probe, net.hosts()):
        if res:
            print(f"open {res}:{PORT}")
