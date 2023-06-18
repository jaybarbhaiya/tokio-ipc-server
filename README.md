# tokio-ipc-server

Creates an IPC server using tokio::net::UnixListener.
This will create a socket in the /tmp folder.
Upon pressing CRTL+C, a shutdown signal is triggered which terminates the server as well as removes the socket file.
