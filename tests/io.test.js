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

const biblatex = `
@book{crazy-rich,
  author = {Kwan, Kevin},
  title = {Crazy Rich Asians},
  year = {2014},
  publisher = {Anchor Books},
  address = {New York, NY, US},
  type = {book}
}
`

test("from_yaml_str()", () => {
  const bib = hayagriva.from_yaml_str(yaml)
  console.log(bib)
})

test("from_biblatex_str() then to_yaml_str()", () => {
  const bib = hayagriva.from_biblatex_str(biblatex)
  console.log(bib)
  const yaml = hayagriva.to_yaml_str(bib)
  console.log(yaml)
})
