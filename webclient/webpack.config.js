// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

const webpack = require('webpack');
var CompressionPlugin = require('compression-webpack-plugin');

module.exports = {
  entry: './src/bootstrap.tsx',
  output: {
    filename: './bin/webclient.js'
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.js']
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: [
          {
            // Second convert JS/ES6 to JS/ES5.
            // Lots of browsers support ES6 now, but UglifyJs (which is used
            // to minimize the output when using 'webpack -p') doesn't.
            // So, this is sort of a lame hack, that I should get rid of somehow.
            loader: 'babel-loader',
          },
          {
            // First convert TS w/ ES6 syntax to JS (also ES6)
            loader: 'ts-loader',
          }
        ],
      },
    ]
  },
  plugins: [
    new CompressionPlugin({
      algorithm: "gzip",
    }),
    new webpack.BannerPlugin({
			banner: "Copyright 2018 Google LLC\nLicensed under the Apache License, Version 2.0 (the \"License\");\nyou may not use this file except in compliance with the License.\nYou may obtain a copy of the License at\n\n      http://www.apache.org/licenses/LICENSE-2.0\n\n Unless required by applicable law or agreed to in writing, software\ndistributed under the License is distributed on an \"AS IS\" BASIS,\nWITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\nSee the License for the specific language governing permissions and\nlimitations under the License.",
		})
  ],
}
