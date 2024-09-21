# rskit

## rust kit

### cache
You can use cache to store data and support multi-threaded sharing.
```rust
let cache = rskit::cache::Cache::new();
cache.set("name", "iclings");
let name = cache.get::<&str>("name").unwrap();
println!("name is {}", name);
```

### str
- string to base64
```rust
let s = "hello world";
let res = atob(s.as_bytes()).unwrap();
println!("res is: {}", res);
```
- base64 to string
```rust
let s = "hello world";
let ss = atob(s.as_bytes()).unwrap();
let res = btoa(&ss).unwrap();
println!("res is: {}", String::from_utf8(res).unwrap());
```

### lib
- fast_log
```rust
rskit::Log::default().init().unwrap();
println!("init log ...");
```

- config
```rust
let mut config = rskit::Configs::<Settings>::new();
let settings = config.init(None).unwrap();
println!("version: {}", settings.app.version);
```

### serde
- to_bin
- from_bin
- to_json
- from_json

### hash
- sha_256
- sha_512
- hash_256
- hash_512
