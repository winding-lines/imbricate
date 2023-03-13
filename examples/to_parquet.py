import pandas as pd


def main() -> None:
    """Za main."""
    data = pd.read_json("out.jsonl", lines=True)
    data.to_parquet("out.parquet")


if __name__ == "__main__":
    main()
