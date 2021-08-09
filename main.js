console.log("Connecting to socket...");

var socket = new WebSocket("ws://localhost:3005", "protocolOne");

console.log("Connected on: " + socket.url);

socket.onopen = function (event) {
  socket.send("data!");
};
