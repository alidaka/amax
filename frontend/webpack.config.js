const path = require('path');

module.exports = {
  context: path.resolve(__dirname, 'src'),

  entry: {
    app: [
      './index.js'
    ]
  },

  output: {
    filename: '[name].js',
  },

  module: {
    rules: [
      {
        test:    /\.elm$/,
        exclude: [/elm-stuff/, /node_modules/],
        loader:  'elm-webpack-loader?verbose=true',
      },
      {
        test:    /\.html$/,
        exclude: /node_modules/,
        loader:  'file-loader?name=[name].[ext]',
      },
    ],

    noParse: /\.elm$/,
  },

  devServer: {
    inline: true,
    stats: { colors: true },
  }
};
