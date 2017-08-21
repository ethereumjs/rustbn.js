const tape = require('tape')
const rustbnjs = require('../index.js')

tape('Curve operations', function (t) {

  t.test('should do the right thing', function (st) {
    st.equal(rustbnjs._add(2, 3), 5)
    st.end()
  })
})