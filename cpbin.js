const { join } = require('path');
const { copyFileSync, mkdirSync, existsSync } = require('fs');

if (!existsSync('dist')) mkdirSync('dist');
copyFileSync(
  join(__dirname, 'native', 'index.node'),
  join(__dirname, 'dist', 'index.node'),
);
