const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const TinyTest = require('./vendor/tinytest.js');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
  module: {
    rules: [
      {
        test: /\.csv$/i,
        use: 'raw-loader',
      },
      {
        test: require.resolve('./vendor/tinytest.js'),
        loader: 'exports-loader',
        options: {
          exports: 'default TinyTest',
        },
      },
    ],
  },
};
