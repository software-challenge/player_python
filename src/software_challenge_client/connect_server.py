from lxml import objectify, etree
import lxml.etree
import socket

protocol = etree.Element("protocol")
protocol.append(etree.Element("join"))

print(lxml.etree.tostring(protocol, pretty_print=True))

# Create a TCP/IP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# Connect the socket to the port where the server is listening
server_address = ('localhost', 13050)
print('connecting to %s port %s' % server_address)
sock.connect(server_address)

# Send a xString message to the server with the protocol tag
message = str(lxml.etree.tostring(protocol, pretty_print=True))
print('sending "%s"' % message)
sock.sendall(message.encode())

while True:
    # Receive the data from the server
    data = sock.recv(1024)
    print('received "%s"' % data)
    if not data:
        break

sock.close()
