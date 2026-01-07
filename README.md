# Serving

```
rm -rf target/server-release/
dx build --release --ssg
npx http-server target/dx/paws_and_preferences/release/web/public -o -c-1
```
