# segmenter
##### v1.0.0
---

### About
Segment Chinese sentences into component words using a dictionary-driven largest first matching approach.

### Usage
```rust
extern crate chinese_segmenter;

use chinese_segmenter::{initialize, tokenize};

let sentence = "今天晚上想吃羊肉吗？";
initialize(); // Optional intialization to load data
let result: Vec<&str> = tokenize(sentence);
println!("{:?}", result); // --> ['今天', '晚上', '想', '吃', '羊肉', '吗']
```

### Contributors
- [Preston Wang-Stosur-Bassett](http://stosur.info)

### License
[MIT](https://github.com/sotch-pr35mac/segmenter/blob/master/LICENSE)
