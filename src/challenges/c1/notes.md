- `TinyMap` is more performant being an `Struct-of-Arrays` (SoA) than an `Array-of-Structs` (AoS, storing objects like `KeyValuePair`), because:
  - More keys can be stored per cache-line
  - A modification of a value doesn't invalidate the cache-line for the keys

- `inlining` both `get` and `insert` unlock huge performance improvements

- A valid hashing method (mapping key into an index) is crucial to minimize probings
   - multiplying the key by a large number improves variance of the resulting hash
     - You might lose some cache locality (as keys will be stored sparsely in the array)
     - However, more variability decreases likelihood of collisions, which is more important

- Despite heap-allocations, `HashMap` is still faster than `TinyMap` prbably because:
  - `SIMD` support
  - Better hashing method (more collision-resistant)
  - Better probing method (quadratic probing vs linear probing)