'use strict';

const bin = require('./');
const spawn = require('child_process').spawn;

const input = process.argv.slice(2);

if (bin !== null) {
  spawn(bin, input, { stdio: 'inherit' }).on('exit', process.exit);
} else {
  throw new Error(
    `Platform "${process.platform} (${process.arch})" not supported.`,
  );
}
