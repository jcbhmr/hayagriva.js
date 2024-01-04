# Hayagriva for JavaScript

🦀 Rusty bibliography management for JavaScript

## Installation

```sh
npm install hayagriva
```

```js
import * as hayagriva from "npm:hayagriva"
```

```html
<script type="module">
  import * as hayagriva from "https://esm.sh/hayagriva@0"
</script>
```

## Usage

```js
import * as hayagriva from "hayagriva"

const yaml = `
crazy-rich:
    type: Book
    title: Crazy Rich Asians
    author: Kwan, Kevin
    date: 2014
    publisher: Anchor Books
    location: New York, NY, US
`

const lib = hayagriva.from_yaml_str(yaml);
const entry = lib.get("my-book")
console.log(entry.title().value)
//=> 'Crazy Rich Asians'
```

## Development

```sh
./just test
```
