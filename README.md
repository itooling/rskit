# rust kit

## cache
You can use cache to store data and support multi-threaded sharing.
```rust
let cache = rskit::cache::Cache::new();
cache.set("name", "iclings");
let name = cache.get::<&str>("name").unwrap();
println!("name is {}", name);
```

## str
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

## lib
- fast_log
```rust
use std::{thread, time::Duration};
rskit::Log::new().init_file().unwrap();
rskit::log::info!("init log ...");
thread::sleep(Duration::from_secs(1));
```

- config
```rust
let mut config = rskit::Configs::<Settings>::new();
let settings = config.init(None).unwrap();
println!("version: {}", settings.app.version);
```

## serde
- to_json
```rust
use rskit::sd::*;
let aoo = Aoo {
    name: String::from("ok"),
    age: 18,
    date: Local::now(),
};

match to_json(aoo) {
    Ok(s) => {
        println!("aoo is {}", s);
    }
    Err(e) => println!("to_json error: {}", e),
}
```

- from_json
```rust
use rskit::sd::*;
let s = r#"{"name":"ok","age":18, "date": "2024-08-15 11:00:16.100"}"#;
match from_json::<'_, Aoo>(s) {
    Ok(aoo) => {
        println!("aoo is: {:?}", aoo);
    }
    Err(e) => println!("from_json error: {}", e),
}
```

## hash
- hash_256
```rust
use rskit::str::*;
let res = hash_256("hello world");
println!("hash 256 is {}", res);
```
- hash_512
```rust
use rskit::str::*;
let res = hash_512("hello world");
println!("hash 512 is {}", res);
```

## crypto
- generate aes key
```rust
use rskit::crypto::*;
gen_rand_string(None);
```

- aes cbc 128
```rust
use rskit::crypto::*;
let key = gen_rand_string(Some(16));
let key = key.as_bytes();
let des = encrypt_aes_cbc_128(key, b"hello world");
let res = decrypt_aes_cbc_128(key, des.as_slice());
println!(
    "aes cbc 128 decrypt is {:?}",
    String::from_utf8(res).unwrap()
);
```

- aes cbc 256
```rust
use rskit::crypto::*;
let key = gen_rand_string(Some(32));
let key = key.as_bytes();
let des = encrypt_aes_cbc_256(key, b"hello world");
let res = decrypt_aes_cbc_256(key, des.as_slice());
println!(
    "aes cbc 256 decrypt is {:?}",
    String::from_utf8(res).unwrap()
);
```

- aes gcm 128
```rust
use rskit::crypto::*;
let key = aes_gcm_key_128();
let nonce = aes_gcm_nonce_128();
let des = encrypt_aes_gcm_128(&key, &nonce, b"hello world");
let res = decrypt_aes_gcm_128(&key, &nonce, des.as_slice());
println!(
    "aes gcm 128 decrypt is {:?}",
    String::from_utf8(res).unwrap()
);
```

- aes gcm 256
```rust
use rskit::crypto::*;
let key = aes_gcm_key_256();
let nonce = aes_gcm_nonce_256();
let des = encrypt_aes_gcm_256(&key, &nonce, b"hello world");
let res = decrypt_aes_gcm_256(&key, &nonce, des.as_slice());
println!(
    "aes gcm 256 decrypt is {:?}",
    String::from_utf8(res).unwrap()
);
```

- generate rsa pair
```rust
use rskit::crypto::*;
let res = generate_rsa_pair(None);
println!("key pair: {:?}", res);
```

- rsa
```rust
use rskit::crypto::*;
let (pri_key, pub_key) = generate_rsa_pair(None);
let src = "hello world";
let enc = encrypt_rsa_base(&pub_key, src.as_bytes()).unwrap();
let res = decrypt_rsa_base(&pri_key, &enc).unwrap();
println!("src: {}", src);
println!("res: {}", String::from_utf8(res).unwrap());
```