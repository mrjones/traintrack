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
            loader: 'awesome-typescript-loader',
          }
        ],
      },
    ]
  }
}
