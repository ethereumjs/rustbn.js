const tape = require('tape')
const rustbnjs = require('../index.js')

tape('Curve operations', function (t) {

  t.test('Executing simple JS function to test build functionality', function (st) {
    st.equal(rustbnjs.add(2, 3), 5)
    st.end()
  })
})