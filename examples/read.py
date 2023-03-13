from imbricate import read_parquet

date = read_parquet("../out.parquet")
print(date.day)
