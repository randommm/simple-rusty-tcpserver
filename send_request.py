import socket

address = ("127.0.0.1", 20200)
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
# s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
# s.bind( ("127.0.0.2", 9190) )
s.connect(address)
s.sendall(b'Hi there!')
data = s.recv(500)
print('Got:', data.decode(errors='backslashreplace'))
s.close()

# open many connections and send many requests
sl = []
for i in range(5):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect(address)
    s.sendall(b'Hi there!')
    sl.append(s)
[s.recv(500) for s in sl]
[s.close() for s in sl]
