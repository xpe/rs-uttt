# LruCache

Links:

* http://contain-rs.github.io/lru-cache/lru_cache/
* http://contain-rs.github.io/lru-cache/lru_cache/struct.LruCache.html

## Get

```rs
fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
```
## Insert

```rs
fn insert(&mut self, k: K, v: V) -> Option<V>
```
