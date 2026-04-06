#!/usr/bin/env python3

from pathlib import Path
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

INPUT_CSV  = Path("clippy_analysis/summary/unsafe_constructs_summary.csv")
OUTPUT_DIR = Path("clippy_analysis/summary")
OUTPUT_PNG = OUTPUT_DIR / "unsafe_constructs_table_clean.png"

ORDER = ["specs_only", "code_only", "code_and_spec"]
DISPLAY_NAMES = {
    "specs_only":    "Spec-only",
    "code_only":     "Code-only",
    "code_and_spec": "Code+Spec",
}

RAW_METRICS = [
    ("total_unsafe_blocks", "Unsafe Blocks"),
    ("total_unsafe_lines",  "Unsafe Lines"),
    ("total_type_casts",    "Type Casts"),
    ("total_unsafe_calls",  "Unsafe Calls"),
]

PER_C_KLOC_METRICS = [
    ("unsafe_blocks_per_c_kloc", "Unsafe Blocks / C KLOC"),
    ("unsafe_lines_per_c_kloc",  "Unsafe Lines / C KLOC"),
    ("type_casts_per_c_kloc",    "Type Casts / C KLOC"),
    ("unsafe_calls_per_c_kloc",  "Unsafe Calls / C KLOC"),
]


def clean_numeric(series):
    return pd.to_numeric(series.astype(str).str.replace(r"\s+", "", regex=True), errors="coerce")


def load_data():
    df = pd.read_csv(INPUT_CSV)
    for col in df.columns:
        if col != "translation_type":
            df[col] = clean_numeric(df[col])
    df["translation_type"] = pd.Categorical(df["translation_type"], categories=ORDER, ordered=True)
    return df.sort_values("translation_type").reset_index(drop=True)


def build_matrix(df, metrics):
    rows, index = [], []
    for col, label in metrics:
        row = []
        for t in ORDER:
            val = df.loc[df["translation_type"] == t, col]
            row.append(float(val.iloc[0]) if not val.empty else np.nan)
        rows.append(row)
        index.append(label)
    return pd.DataFrame(rows, index=index, columns=[DISPLAY_NAMES[t] for t in ORDER])


def build_loc_matrix(df):
    return build_matrix(df, [
        ("total_c_loc",      "C LOC"),
        ("total_rust_loc",   "Rust LOC"),
        ("inflation_factor", "R/C ratio"),
    ])


def style_table(table, font_size=10, yscale=1.15):
    table.auto_set_font_size(False)
    table.set_fontsize(font_size)
    table.scale(1.0, yscale)
    for (_, _), cell in table.get_celld().items():
        cell.set_linewidth(0.6)


def highlight_best_per_row(table, data):
    for i, row in enumerate(data.values):
        clean_row = [v for v in row if not np.isnan(v)]
        if not clean_row:
            continue
        best_val = min(clean_row)
        for j, v in enumerate(row):
            if not np.isnan(v) and np.isclose(v, best_val):
                table[(i + 1, j)].set_text_props(weight="bold", color="#009E73")


def draw_table(ax, data, title, fmt, font_size=10, yscale=1.15):
    ax.axis("off")
    use_per_row = isinstance(fmt, list)
    table_data = []
    for i, row in enumerate(data.values):
        row_fmt = fmt[i] if use_per_row else fmt
        table_data.append([
            "NA" if np.isnan(v) else format(v, row_fmt) for v in row
        ])
    table = ax.table(
        cellText=table_data,
        rowLabels=data.index,
        colLabels=data.columns,
        loc="center",
        cellLoc="center",
        rowLoc="center",
    )
    style_table(table, font_size=font_size, yscale=yscale)
    highlight_best_per_row(table, data)
    ax.set_title(title, fontsize=11, weight="bold", pad=4)


def main():
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    df       = load_data()
    loc_df   = build_loc_matrix(df)
    raw_df   = build_matrix(df, RAW_METRICS)
    per_c_df = build_matrix(df, PER_C_KLOC_METRICS)

    fig = plt.figure(figsize=(7.2, 6.2))
    gs  = fig.add_gridspec(3, 1, height_ratios=[0.85, 1.35, 1.35])

    draw_table(fig.add_subplot(gs[0]), loc_df,   "LOC & Size Inflation (R/C)",
               fmt=[".0f", ".0f", ".3f"], font_size=10, yscale=1.05)
    draw_table(fig.add_subplot(gs[1]), raw_df,   "A. Unsafe Construct Counts (raw)",
               fmt=".0f", font_size=9.5, yscale=1.08)
    draw_table(fig.add_subplot(gs[2]), per_c_df, "B. Unsafe Constructs per C KLOC (TSI-adjusted)",
               fmt=".2f", font_size=9.5, yscale=1.08)

    fig.subplots_adjust(left=0.26, right=0.98, top=0.96, bottom=0.04, hspace=0.18)
    plt.savefig(OUTPUT_PNG, dpi=300, bbox_inches="tight", pad_inches=0.03)
    print(f"Saved: {OUTPUT_PNG}")


if __name__ == "__main__":
    main()