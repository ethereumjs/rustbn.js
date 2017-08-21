var exp = require('./lib/index.asm.js')

exp.add = exp._add
exp.bn128 = exp._bn128

module.exports = exp