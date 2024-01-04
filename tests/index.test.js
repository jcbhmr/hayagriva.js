import test from "node:test"
import assert from "node:assert/strict"
import * as hayagriva from "../pkg/hayagriva.js"

const yaml = `
crazy-rich:
    type: Book
    title: Crazy Rich Asians
    author: Kwan, Kevin
    date: 2014
    publisher: Anchor Books
    location: New York, NY, US
`

test("Library#iter()", () => {
  const lib = hayagriva.from_yaml_str(yaml)
  const entries = [...lib.iter()]
  console.log(entries)
  assert.equal(entries.length, 1)
})
