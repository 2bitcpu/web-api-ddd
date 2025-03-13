以下の順番で作る
```
models
interfaces
repositories
usecases
handlers
```

必要なクレート
```
cargo add tokio --features macros,rt-multi-thread --no-default-features
cargo add serde --features derive --no-default-features
cargo add serde_json --features std --no-default-features
cargo add chrono --features serde --no-default-features
cargo add async-trait --no-default-features
cargo add sqlx --features runtime-tokio-native-tls,chrono,derive --no-default-features
cargo add axum --features macros

cargo add validator --features derive --no-default-features
```

sqliteはbundledするかしないかをfeatureフラグで指定可能に
```
[features]
default = [ "sqlite" ]
sqlite = [ "sqlx/sqlite-unbundled" ]
sqlite-bundled = [ "sqlx/sqlite" ]
```

```
curl -X POST http://localhost:3000/service/contents/post -H "Content-Type: application/json" -d '{"title": "My Title", "body": "This is the body of the content."}'

curl http://localhost:3000/service/contents/find/1

curl -X POST http://localhost:3000/service/auth/signin -H "Content-Type: application/json" -d '{"username":"tester","password":"p@55w0rd"}'

curl -i -X POST \
  http://localhost:3000/service/contents/post \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJzaW1wbGUtand0Iiwic3ViIjoidGVzdGVyIiwiaWF0IjoxNzQxODU3MTIwLCJleHAiOjE3NDE4NjA3MjAsImp0aSI6IjFlZmQxMjkxLTVjMzktNDBjNy1iNjBiLWI2YzBmY2M2MjE2MyJ9.S1Gx6oCAP_fum7nUJSO-TUOJ3s0ZeYv226GwYTWUvfI' \
  -d '{"title": "example title", "content": "example content"}'
```