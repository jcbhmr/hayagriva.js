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

test("Library#keys()", () => {
  const lib = hayagriva.from_yaml_str(yaml)
  const keys = [...lib.iter()]
  console.log(keys)
  assert.equal(keys.length, 1)
})

test("Entry#key()", () => {
  const lib = hayagriva.from_yaml_str(yaml)
  const entry = lib.nth(0)
  console.log(entry.key())
})

test("Entry#entry_type()", () => {
  const lib = hayagriva.from_yaml_str(yaml)
  const entry = lib.nth(0)
  console.log(entry.entry_type())
})
