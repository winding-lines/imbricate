== Imbricate

"overlapping like tiles, as scales or leaves"

Imbricate is a library that materialize Apache Arrow data as rows in the Python interpreter.

Loosely related work:

- https://data-apis.org/blog/array_api_v2022_release/

== Getting started

Create a virtual environment, activate it and install maturin. Remember to activate the environment in future sessions.

```
python3 -m venv create .venv
source .venv/bin/activate
python3 -m pip install maturin
```

Use the 2 `examples/generate_*` python files to create the file `../out.parquet`.

The run the various readers, the current times are below. This is WIP.

- 12.02 sec `python3 examples/read_pandas.py`
- 42.01 sec `python3 examples/read_batch_deserializer.py`
- 60.89 sec `python3 examples/read_native.py`
