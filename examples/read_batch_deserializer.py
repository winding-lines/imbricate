from time import monotonic
from pyarrow.parquet import read_table
from imbricate import BatchDeserializer


def run():
    table = read_table("../out.parquet")
    count = 0
    for batch in table.to_batches():
        for order in BatchDeserializer(batch):
            count += 1
            if count == 1:
                print(order)

    return count


if __name__ == "__main__":
    start = monotonic()
    count = run()
    end = monotonic()
    print(f"Arrow + batch deserializer read {count} records in {end - start:.2f} seconds")
