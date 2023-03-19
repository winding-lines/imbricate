from imbricate import ArrowReader, BatchDeserializer
from time import monotonic

start = monotonic()
batches = ArrowReader("../out.parquet")
count = 0
for batch in batches:
    for order in batch:
        count += 1
        if count == 1:
            print(order)
end = monotonic()
print(f"Native read {count} records in {end - start:.2f} seconds")
