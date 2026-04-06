#!/usr/bin/env python3

import requests
from bs4 import BeautifulSoup
import csv
from pathlib import Path

URL = "https://rust-lang.github.io/rust-clippy/rust-1.87.0/index.html"

OUTPUT_PATH = Path("clippy_analysis/metadata/clippy_lint_group_map_rust_1_87_0.csv")
OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)

response = requests.get(URL, timeout=60)
response.raise_for_status()

soup = BeautifulSoup(response.content, "html.parser")

lint_articles = soup.find_all("article")

lint_data = []

for article in lint_articles:
    header = article.find("h2", class_="panel-title")
    if not header:
        continue

    # ---------------------------
    # LINT NAME
    # ---------------------------
    name_span = header.find("span")
    if not name_span:
        continue

    lint_name = name_span.text.strip()
    lint_code = f"clippy::{lint_name}"

    # ---------------------------
    # GROUP (DYNAMIC DETECTION)
    # ---------------------------
    group = None
    for span in header.find_all("span"):
        classes = span.get("class", [])
        for c in classes:
            if c.startswith("label-group-"):
                group = c.replace("label-group-", "")
                break
        if group:
            break

    if not group:
        group = "unknown"

    # ---------------------------
    # DEFAULT LEVEL
    # ---------------------------
    level = None
    for span in header.find_all("span"):
        classes = span.get("class", [])
        for c in classes:
            if c.startswith("label-lint-level-"):
                level = c.replace("label-lint-level-", "")
                break
        if level:
            break

    if not level:
        level = "unknown"

    lint_data.append({
        "lint_code": lint_code,
        "lint_name": lint_name,
        "clippy_group": group,
        "default_level": level
    })

# ---------------------------
# SAVE CSV
# ---------------------------
with open(OUTPUT_PATH, "w", newline="", encoding="utf-8") as f:
    writer = csv.DictWriter(
        f,
        fieldnames=["lint_code", "lint_name", "clippy_group", "default_level"]
    )
    writer.writeheader()
    writer.writerows(sorted(lint_data, key=lambda x: x["lint_code"]))

print(f"✅ Saved {len(lint_data)} lints to {OUTPUT_PATH}")