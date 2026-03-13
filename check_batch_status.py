#!/usr/bin/env python3
"""
Check the status of an OpenAI batch job.
"""

from __future__ import annotations

import os

from dotenv import load_dotenv
from openai import OpenAI


# =========================
# Configure these values
# =========================
BATCH_ID = "batch_69b2efb68b248190a9079d1c8905a202"


def main() -> None:
    load_dotenv()
    api_key = os.environ.get("OPENAI_API_KEY", "").strip()
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set.")
    if not BATCH_ID or BATCH_ID == "batch_...":
        raise ValueError("Set BATCH_ID in the script before running it.")

    client = OpenAI(api_key=api_key)
    batch = client.batches.retrieve(BATCH_ID)

    print(f"Batch ID: {batch.id}")
    print(f"Status: {batch.status}")
    print(f"Created at: {batch.created_at}")
    print(f"Input file ID: {batch.input_file_id}")
    print(f"Output file ID: {batch.output_file_id}")
    print(f"Error file ID: {batch.error_file_id}")
    print(f"Completion window: {batch.completion_window}")
    print(f"Endpoint: {batch.endpoint}")

    request_counts = getattr(batch, "request_counts", None)
    if request_counts is not None:
        print("Request counts:")
        print(f"  Total: {getattr(request_counts, 'total', None)}")
        print(f"  Completed: {getattr(request_counts, 'completed', None)}")
        print(f"  Failed: {getattr(request_counts, 'failed', None)}")

    errors = getattr(batch, "errors", None)
    if errors:
        print(f"Errors: {errors}")


if __name__ == "__main__":
    main()
