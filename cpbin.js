const { join } = require('path');
const { copyFileSync, mkdirSync } = require('fs');

mkdirSync('dist');
copyFileSync(
  join(__dirname, 'native', 'index.node'),
  join(__dirname, 'dist', 'index.node'),
);
