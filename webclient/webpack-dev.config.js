const merge = require('webpack-merge');
const webpack = require('webpack');

const template = require('./webpack.config.js');

module.exports = merge(template, {
  plugins: [
    new webpack.DefinePlugin({
      'process.env.NODE_ENV': '"development"',
      'ENABLE_PREFETCHING': 'true',
      'BUILD_LABEL': '"DEV"',
    }),
  ],
});
