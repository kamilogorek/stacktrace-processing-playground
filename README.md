- Use https://github.com/getsentry/symbolic/tree/feat/smcache branch as your local env.
- Update paths to `symbolic` in `rust/Cargo.toml` and Python command below.
- Run `make build && make header` in `symbolic-cabi` and `pip install` mentioned below again, whenever you are updating FFI code.

# Rust

```
cd rust
cargo run
```

# Python

```
cd python
python3 -m venv env
source env/bin/activate
SYMBOLIC_DEBUG=1 python -m pip install --editable ../../../../repositories/symbolic/py
python run.py
```