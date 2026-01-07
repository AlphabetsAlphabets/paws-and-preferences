# Serving

```
rm -rf target/server-release/
dx build --release --ssg
npx http-server target/dx/server_dog/release/web/public -o -c-1
```
