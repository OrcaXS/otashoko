module.exports = {
  root: true,
  env: {
    node: true
  },
  extends: [
    'plugin:vue/recommended',
    '@vue/airbnb'
  ],
  settings: {
    'import/resolver': {
      webpack: 'webpack.config.js',
      node: {
        'extensions': [
          '.js',
          '.vue',
        ],
      },
    },
  },
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'semi': ['error', 'always', { 'omitLastInOneLineBlock': true }],
    'import/extensions': ['error', 'always', {
      'js': 'never',
      'vue': 'never',
    }],
  },
  parserOptions: {
    parser: 'babel-eslint'
  }
}
