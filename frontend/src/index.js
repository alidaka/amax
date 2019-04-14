'use strict';

require('./index.html');
require('./websocket.js');

var Elm = require('./Main.elm').Elm;
window.app = Elm.Main.init({node: document.getElementById('elm-app')});
