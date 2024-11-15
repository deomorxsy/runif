### potalaroc
> poc environment for tracing FFI from Elixir to Rust with honey-potion

Build it with:
```
docker compose -f ./compose.yml --progress=plain build --no-cache potalaroc
```

Run it with:
```
docker compose -f ./compose.yml potalaroc up
```

### References

- [rustler](https://hexdocs.pm/rustler/readme.html#loading-the-nif)
- [rustler Rust crate](https://github.com/rusterlium/rustler)
- [rustler Elixir library](https://hex.pm/packages/rustler)
- [honey-potion](https://github.com/lac-dcc/honey-potion) repo
