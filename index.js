const bn128 = require('./lib/index.asm.js')

const ec_add = bn128.cwrap('ec_add', 'string', ['string'])
const ec_mul = bn128.cwrap('ec_mul', 'string', ['string'])
const ec_pairing = bn128.cwrap('ec_pairing', 'string', ['string'])

function bn128add (input) {
  return ec_add(input)
}

function bn128mul (input) {
  return ec_mul(input)
}

function bn128pairing (input) {
  return ec_pairing(input)
}

module.exports = {
  add: bn128add,
  mul: bn128mul,
  pairing: bn128pairing
}
