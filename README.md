# ngram_set

A data structure that stores data (usually string data) along with n-grams of the data, allowing a user
to query for similar text using various similarity algorithms.

```rust
let words = ngram_set::new(vec!["spam", "eggs"]);
words.with_threshold(0.5).search('ham'); // => None
words.with_threshold(0.2).search('ham'); // => Some("spam")

ngram_set::compare("spa", "spam"); // => 0.375
ngram_set::compare("ham", "bam"); // => 0.25
ngram_set::compare("spam", "pam"); // => 0.375
ngram_set::compare_with_n("ham", "ams", 1); // => 0.5
```

