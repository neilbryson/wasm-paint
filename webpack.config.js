const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const isDev = process.env.NODE_ENV === 'development';

module.exports = {
  devServer: {
    client: {
      overlay: {
        errors: true,
      },
    },
    historyApiFallback: true,
    hot: true,
    port: 3000,
    static: {
      directory: path.join(__dirname, 'public'),
    },
  },
  devtool: isDev && 'inline-source-map',
  entry: path.resolve(__dirname, 'src/bootstrap.js'),
  experiments: {
    syncWebAssembly: true,
  },
  mode: isDev ? 'development' : 'production',
  module: {
    rules: [
      {
        include: path.resolve(__dirname, 'src'),
        loader: 'file-loader',
        options: {
          name: '[path][name].[ext]',
        },
        test: /\.(png|jpe?g|gif)$/,
      },
      {
        include: path.resolve(__dirname, 'src'),
        loader: 'ts-loader',
        options: {
          transpileOnly: true,
        },
        test: /\.tsx?$/,
      },
      {
        include: path.resolve(__dirname, 'src'),
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
    new ForkTsCheckerWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.wasm'],
  }
};
