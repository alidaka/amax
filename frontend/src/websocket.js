'use strict';

document.addEventListener('DOMContentLoaded', function() {
  var app = window.app;

  const socket = new WebSocket('ws://localhost:3001');

  socket.addEventListener('open', function(event) {
  });

  socket.addEventListener('message', function(event) {
    console.log('message from server: ', event.data);
    app.ports.fromSocket.send(event.data);
  });

  app.ports.toSocket.subscribe(function(str) {
    console.log("got from Elm:", str);
    socket.send(str);
  });
}, false);
