'use strict';

document.addEventListener('DOMContentLoaded', function() {
  var app = window.app;

  const socket = new WebSocket('ws://localhost:3001');

  socket.addEventListener('open', function(event) {
  });

  socket.addEventListener('message', function(event) {
    console.log('message from server: ', event.data);
    app.ports.websocketIn.send(event.data);
  });

  app.ports.websocketOut.subscribe(function(str) {
    console.log("got from Elm:", str);
    socket.send(str);
  });
}, false);
