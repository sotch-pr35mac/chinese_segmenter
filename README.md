# segmenter
##### v0.1.2
---

### About
Segment Chinese sentences into component words using a dictionary-driven largest first matching approach.

### Usage
```rust
extern crate chinese_segmenter;

use chinese_segmenter::ChineseSegmenter;

let segmenter = ChineseSegmenter::new();

let sentence: String = String::from("今天晚上想吃羊肉吗？");
let result: Vec<String> = segmenter.tokenize(sentence);
println!("{:?}", result); // --> ['今天', '晚上', '想', '吃', '羊肉', '吗']
```

### Contributors
- [Preston Wang-Stosur-Bassett](http://stosur.info)

### License
[MIT](https://github.com/sotch-pr35mac/segmenter/blob/master/LICENSE)
