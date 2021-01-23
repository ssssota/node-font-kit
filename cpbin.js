const { join } = require('path');
const { copyFileSync, mkdirSync, existsSync } = require('fs');

const binPath = join(__dirname, 'native', 'index.node');
if (!existsSync('dist')) mkdirSync('dist');
if (existsSync(join(binPath))) copyFileSync(binPath, join(__dirname, 'dist', 'index.node'));
