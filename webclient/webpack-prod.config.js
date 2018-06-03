const merge = require('webpack-merge');
const webpack = require('webpack');

const template = require('./webpack.config.js');

module.exports = merge(template, {
  plugins: [
    new webpack.DefinePlugin({
      'ENABLE_PREFETCHING': 'false',
      'BUILD_LABEL': '"PROD"',
    }),
  ],
});
