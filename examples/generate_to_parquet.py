"""Translate the JSONL file to Parquet format, both input and output are hardcoded."""

import pandas as pd


def to_parquet() -> None:
    data = pd.read_json("out.jsonl", lines=True)
    data.to_parquet("out.parquet")


if __name__ == "__main__":
    to_parquet()
