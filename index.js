if (global.WebAssembly !== undefined) {
  const wasmModule = require('fs').readFileSync('./lib/rustbn.wasm')
  const bn128 = WebAssembly.instantiate(wasmModule, { env: {} }).then(function (bn128) {
    const ec_add = bn128.instance.exports.ec_add
    const ec_mul = bn128.instance.exports.ec_mul
    const ec_pairing = bn128.instance.exports.ec_pairing
    let memory = bn128.instance.exports.memory

    function findFreeMemoryPtr () {
       // FIXME: implement
       return 42
    }

    function writeHexString (buffer, startOffset, data) {
      // FIXME: implement
      console.log('writing to', startOffset)

      let bufferView = new Uint8Array(buffer)
      data.copy(bufferView, startOffset)
    }

    function extractHexString (buffer, startOffset) {
      console.log('reading from', startOffset)

      const bufferView = new Uint8Array(buffer)

      let endOffset = startOffset
      for (; bufferView[endOffset] !== 0; endOffset++) { }

      return Buffer.from(bufferView.slice(startOffset, endOffset), 'hex')
    }

    let ptr = findFreeMemoryPtr()
    writeHexString(memory.buffer, ptr, Buffer.from('00112200', 'hex'))
    let retPtr = ec_add(ptr)
    let ret = extractHexString(memory.buffer, retPtr)

    console.log('returned', ret.toString('hex'))
  })
} else {
  const bn128 = require('./lib/index.asm.js')
  const ec_add = bn128.cwrap('ec_add', 'string', ['string'])
  const ec_mul = bn128.cwrap('ec_mul', 'string', ['string'])
  const ec_pairing = bn128.cwrap('ec_pairing', 'string', ['string'])
}

function bn128add (input) {
  return Buffer.from(ec_add(input.toString('hex')), 'hex')
}

function bn128mul (input) {
  return Buffer.from(ec_mul(input.toString('hex')), 'hex')
}

function bn128pairing (input) {
  return Buffer.from(ec_pairing(input.toString('hex')), 'hex')
}

module.exports = {
  add: bn128add,
  mul: bn128mul,
  pairing: bn128pairing
}
