document.addEventListener('DOMContentLoaded', function() {
  var app = Elm.Main.embed(document.getElementById('view'));

  const socket = new WebSocket('ws://localhoset:3000');

  socket.addEventListener('open', function(event) {
  });

  socket.addEventListener('message', function(event) {
    console.log('message from server: ' event.data);
    app.ports.websocketOut.send(event.data);
  });

  app.ports.websocketIn.subscribe(function(str) {
    console.log("got from Elm:", str);
    socket.send(str);
  });
}, false);
