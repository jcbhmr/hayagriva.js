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

test("hayagriva.from_yaml_str()", () => {
  const bib = hayagriva.from_yaml_str(yaml)
  console.log(bib)
  const entry = bib.get("crazy-rich")
  console.log(entry)
  const title = entry.title()
  console.log(title)
  console.log(title.value)
})
