#!/usr/bin/env python3

import csv
from collections import defaultdict
from pathlib import Path

IN_CSV  = Path("clippy_analysis/summary/unsafe_constructs_detailed.csv")
LOC_CSV = Path("clippy_analysis/summary/loc_by_problem_and_translation.csv")
OUT_CSV = Path("clippy_analysis/summary/unsafe_constructs_summary.csv")

ORDER = ["specs_only", "code_only", "code_and_spec"]


def to_int(x):
    try:
        return int(x)
    except (TypeError, ValueError):
        return 0


def load_c_loc_by_translation():
    """Sum C LOC per translation type from the shared LOC CSV."""
    totals = defaultdict(int)
    with LOC_CSV.open("r", encoding="utf-8", newline="") as f:
        for row in csv.DictReader(f):
            tt = (row.get("translation_type") or "").strip()
            if tt in ORDER:
                totals[tt] += to_int(row.get("c_loc"))
    return totals


def main():
    # ── Aggregate unsafe metrics by translation type ──────────────────
    agg = defaultdict(lambda: {
        "total_rust_loc":      0,
        "total_unsafe_blocks": 0,
        "total_unsafe_lines":  0,
        "total_type_casts":    0,
        "total_unsafe_calls":  0,
    })

    with IN_CSV.open("r", encoding="utf-8", newline="") as f:
        for row in csv.DictReader(f):
            t = row["translation_type"]
            agg[t]["total_rust_loc"]      += to_int(row["rust_loc"])
            agg[t]["total_unsafe_blocks"] += to_int(row["unsafe_blocks"])
            agg[t]["total_unsafe_lines"]  += to_int(row["unsafe_lines"])
            agg[t]["total_type_casts"]    += to_int(row["type_casts"])
            agg[t]["total_unsafe_calls"]  += to_int(row["unsafe_calls"])

    # ── Load C LOC for TSI-adjusted normalization ─────────────────────
    c_loc_by_tt = load_c_loc_by_translation()

    rows = []
    for t in ORDER:
        v        = agg[t]
        rust_loc = v["total_rust_loc"]
        c_loc    = c_loc_by_tt[t]

        rust_kloc = rust_loc / 1000 if rust_loc else 0
        c_kloc    = c_loc    / 1000 if c_loc    else 0

        # Inflation factor (how much larger the Rust code is vs C)
        inflation = (rust_loc / c_loc) if c_loc > 0 else 0.0

        def per_rust(n): return round(n / rust_kloc, 4) if rust_kloc else 0.0
        def per_c(n):    return round(n / c_kloc,    4) if c_kloc    else 0.0
        # per_c(n) == per_rust(n) * inflation  — warnings per original C KLOC

        rows.append({
            "translation_type":              t,
            "total_c_loc":                   c_loc,
            "total_rust_loc":                rust_loc,
            "inflation_factor":              round(inflation, 4),

            # Raw counts
            "total_unsafe_blocks":           v["total_unsafe_blocks"],
            "total_unsafe_lines":            v["total_unsafe_lines"],
            "total_type_casts":              v["total_type_casts"],
            "total_unsafe_calls":            v["total_unsafe_calls"],

            # Per Rust KLOC (baseline normalisation)
            "unsafe_blocks_per_rust_kloc":   per_rust(v["total_unsafe_blocks"]),
            "unsafe_lines_per_rust_kloc":    per_rust(v["total_unsafe_lines"]),
            "type_casts_per_rust_kloc":      per_rust(v["total_type_casts"]),
            "unsafe_calls_per_rust_kloc":    per_rust(v["total_unsafe_calls"]),

            # Per C KLOC (TSI-adjusted: penalises code bloat)
            "unsafe_blocks_per_c_kloc":      per_c(v["total_unsafe_blocks"]),
            "unsafe_lines_per_c_kloc":       per_c(v["total_unsafe_lines"]),
            "type_casts_per_c_kloc":         per_c(v["total_type_casts"]),
            "unsafe_calls_per_c_kloc":       per_c(v["total_unsafe_calls"]),
        })

    with OUT_CSV.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)

    # ── Console summary ───────────────────────────────────────────────
    metrics = [
        ("unsafe_blocks", "Unsafe blocks"),
        ("unsafe_lines",  "Unsafe lines"),
        ("type_casts",    "Type casts"),
        ("unsafe_calls",  "Unsafe calls"),
    ]

    print(f"\n{'Translation':<15} {'C LOC':>8} {'Rust LOC':>9} {'R/C':>6}")
    print("-" * 42)
    for r in rows:
        print(f"{r['translation_type']:<15} {r['total_c_loc']:>8,} "
              f"{r['total_rust_loc']:>9,} {r['inflation_factor']:>6.3f}×")

    for key, label in metrics:
        print(f"\n=== {label} (raw | /Rust KLOC | /C KLOC) ===")
        print(f"{'Translation':<15} {'Raw':>6} {'/ Rust':>9} {'/ C':>9}")
        print("-" * 43)
        for r in rows:
            print(f"{r['translation_type']:<15} "
                  f"{r['total_'+key]:>6} "
                  f"{r[key+'_per_rust_kloc']:>9.2f} "
                  f"{r[key+'_per_c_kloc']:>9.2f}")

    print(f"\nSaved to {OUT_CSV}")


if __name__ == "__main__":
    main()