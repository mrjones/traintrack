const { merge } = require('webpack-merge');
const webpack = require('webpack');

const template = require('./webpack.config.js');

module.exports = merge(template, {
  plugins: [
    new webpack.DefinePlugin({
      'ENABLE_PREFETCHING': 'true',
      'BUILD_LABEL': process.env.BUILD_LABEL ? process.env.BUILD_LABEL : '"PROD"',
    }),
  ],
});
