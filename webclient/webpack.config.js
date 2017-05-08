module.exports = {
  entry: './src/bootstrap.tsx',
  output: {
    filename: './bin/webclient.js'
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.js']
  },
  module: {
    loaders: [
      {
        test: /\.tsx?$/,
        loader: 'awesome-typescript-loader',
      },
    ]
  }
}
