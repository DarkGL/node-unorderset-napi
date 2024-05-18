import test from 'ava'

import { UnorderedSet } from '../index.js'

test('insert', (t) => {
  const set = new UnorderedSet()

  set.insert(1)
  set.insert(2)
  set.insert(3)

  t.is(set.has(1), true)
  t.is(set.has(2), true)
  t.is(set.has(3), true)
  t.is(set.has(4), false)
})
