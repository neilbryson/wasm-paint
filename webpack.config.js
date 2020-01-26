const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');

const isDev = process.env.NODE_ENV === 'development';

module.exports = {
  devServer: {
    contentBase: path.join(__dirname, 'src'),
    hot: true,
    overlay: {
      errors: true,
    },
    port: 3000,
  },
  devtool: isDev && 'inline-source-map',
  entry: path.resolve(__dirname, 'src/bootstrap.js'),
  mode: isDev ? 'development' : 'production',
  module: {
    rules: [
      {
        exclude: /node_modules/,
        test: /\.wasm$/,
        type: 'webassembly/experimental',
      },
      {
        exclude: /node_modules/,
        loader: 'ts-loader',
        options: {
          transpileOnly: true,
        },
        test: /\.tsx?$/,
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader'],
      }
    ]
  },
  optimization: {
    splitChunks: {
      chunks: 'all',
    },
  },
  output: {
    chunkFilename: '[name].[chunkhash:8].js',
    filename: '[name].[hash:8].js',
    path: path.resolve(__dirname, 'build'),
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'public/index.html'),
    }),
    new ForkTsCheckerWebpackPlugin({
      async: false,
    }),
  ],
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.wasm'],
  }
};
