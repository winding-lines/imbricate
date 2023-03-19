"""Generate a JSON file with 300,000 objects, outputs to stdout."""
from typing import Any, Dict
import imbricate


def main() -> None:
    imbricate.create_json(300000)


if __name__ == "__main__":
    main()
