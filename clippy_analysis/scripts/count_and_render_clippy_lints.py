#!/usr/bin/env python3

import csv
from collections import defaultdict
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.pagesizes import landscape, letter
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import inch
from reportlab.platypus import SimpleDocTemplate, Table, TableStyle, Paragraph, Spacer

LINT_CSV = Path("clippy_analysis/summary/clippy_lints_detailed.csv")
LOC_CSV  = Path("clippy_analysis/summary/loc_by_problem_and_translation.csv")

OUTPUT_DIR      = Path("clippy_analysis/summary")
DETAIL_CSV      = OUTPUT_DIR / "clippy_raw_and_per_kloc_detailed.csv"
MATRIX_CSV      = OUTPUT_DIR / "clippy_raw_and_per_kloc_table.csv"
LOC_SUMMARY_CSV = OUTPUT_DIR / "clippy_total_loc_summary.csv"
PDF_OUTPUT      = OUTPUT_DIR / "clippy_warning_tables.pdf"

TRANSLATION_ORDER = ["specs_only", "code_only", "code_and_spec"]
CATEGORY_ORDER    = ["correctness", "suspicious", "complexity", "perf", "style", "pedantic"]

DISPLAY_TRANSLATION = {
    "specs_only":   "Specs Only",
    "code_only":    "Code Only",
    "code_and_spec": "Code+Spec",
}


def safe_int(value):
    try:
        return int(value)
    except (TypeError, ValueError):
        return 0


def load_total_loc_by_translation():
    totals = defaultdict(lambda: {"total_c_loc": 0, "total_rust_loc": 0})
    with LOC_CSV.open("r", encoding="utf-8", newline="") as f:
        for row in csv.DictReader(f):
            tt = (row.get("translation_type") or "").strip()
            if tt not in TRANSLATION_ORDER:
                continue
            totals[tt]["total_c_loc"]   += safe_int(row.get("c_loc"))
            totals[tt]["total_rust_loc"] += safe_int(row.get("rust_loc"))
    return totals


def load_warning_counts():
    warning_counts = defaultdict(int)
    with LINT_CSV.open("r", encoding="utf-8", newline="") as f:
        for row in csv.DictReader(f):
            tt  = (row.get("translation_type") or "").strip()
            cat = (row.get("clippy_category")  or "").strip()
            if tt in TRANSLATION_ORDER and cat in CATEGORY_ORDER:
                warning_counts[(tt, cat)] += 1
    return warning_counts


def build_detailed_rows(total_loc_by_translation, warning_counts):
    rows = []
    for tt in TRANSLATION_ORDER:
        rust_loc = total_loc_by_translation[tt]["total_rust_loc"]
        c_loc    = total_loc_by_translation[tt]["total_c_loc"]

        # Size inflation factor (how much bigger the Rust code is vs C)
        inflation_factor = (rust_loc / c_loc) if c_loc > 0 else 0.0

        for cat in CATEGORY_ORDER:
            total_warnings = warning_counts.get((tt, cat), 0)

            # Metric 1: warnings per Rust KLOC  (existing)
            per_rust_kloc = (total_warnings / (rust_loc / 1000.0)) if rust_loc > 0 else 0.0

            # Metric 2: inflation-adjusted = per_rust_kloc * inflation_factor
            #           = warnings / (rust_loc/1000) * (rust_loc/c_loc)
            #           = warnings / (c_loc/1000)   i.e. warnings per C KLOC
            per_c_kloc = (total_warnings / (c_loc / 1000.0)) if c_loc > 0 else 0.0

            rows.append({
                "translation_type":    tt,
                "clippy_category":     cat,
                "total_warnings":      total_warnings,
                "total_c_loc":         c_loc,
                "total_rust_loc":      rust_loc,
                "inflation_factor":    round(inflation_factor, 4),
                "per_rust_kloc":       round(per_rust_kloc, 4),
                "per_c_kloc":          round(per_c_kloc, 4),
            })
    return rows


def fmt_cell(raw, per_rust, per_c):
    """raw | /Rust KLOC | /C KLOC (inflation-adjusted)"""
    return f"{raw} | {per_rust:.2f} | {per_c:.2f}"


def write_detailed_csv(rows):
    fields = [
        "translation_type", "clippy_category", "total_warnings",
        "total_c_loc", "total_rust_loc",
        "inflation_factor", "per_rust_kloc", "per_c_kloc",
    ]
    with DETAIL_CSV.open("w", encoding="utf-8", newline="") as f:
        w = csv.DictWriter(f, fieldnames=fields)
        w.writeheader()
        w.writerows(rows)


def write_matrix_csv(rows):
    matrix_rows = []
    for cat in CATEGORY_ORDER:
        row = {"category": cat}
        for tt in TRANSLATION_ORDER:
            entry = next(r for r in rows if r["clippy_category"] == cat and r["translation_type"] == tt)
            row[f"{tt}_raw"]       = entry["total_warnings"]
            row[f"{tt}_per_rust"]  = entry["per_rust_kloc"]
            row[f"{tt}_per_c"]     = entry["per_c_kloc"]
        matrix_rows.append(row)

    fields = ["category"] + [
        f"{tt}_{m}" for tt in TRANSLATION_ORDER for m in ("raw", "per_rust", "per_c")
    ]
    with MATRIX_CSV.open("w", encoding="utf-8", newline="") as f:
        w = csv.DictWriter(f, fieldnames=fields)
        w.writeheader()
        w.writerows(matrix_rows)


def write_loc_summary_csv(total_loc_by_translation):
    rows = []
    for tt in TRANSLATION_ORDER:
        c    = total_loc_by_translation[tt]["total_c_loc"]
        rust = total_loc_by_translation[tt]["total_rust_loc"]
        rows.append({
            "translation_type":  tt,
            "total_c_loc":       c,
            "total_rust_loc":    rust,
            "inflation_factor":  round(rust / c, 4) if c > 0 else 0.0,
        })
    with LOC_SUMMARY_CSV.open("w", encoding="utf-8", newline="") as f:
        w = csv.DictWriter(f, fieldnames=["translation_type", "total_c_loc", "total_rust_loc", "inflation_factor"])
        w.writeheader()
        w.writerows(rows)


# ── PDF helpers ──────────────────────────────────────────────────────────────

def make_table(data, col_widths):
    table = Table(data, colWidths=col_widths, repeatRows=1)
    table.setStyle(TableStyle([
        ("BACKGROUND",    (0, 0), (-1, 0),  colors.whitesmoke),
        ("FONTNAME",      (0, 0), (-1, 0),  "Helvetica-Bold"),
        ("FONTNAME",      (0, 1), (-1, -1), "Helvetica"),
        ("FONTSIZE",      (0, 0), (-1, -1), 8),
        ("LEADING",       (0, 0), (-1, -1), 10),
        ("ALIGN",         (1, 0), (-1, -1), "CENTER"),
        ("ALIGN",         (0, 0), (0, -1),  "LEFT"),
        ("VALIGN",        (0, 0), (-1, -1), "MIDDLE"),
        ("GRID",          (0, 0), (-1, -1), 0.5, colors.black),
        ("LINEBELOW",     (0, 0), (-1, 0),  0.8, colors.black),
        ("TOPPADDING",    (0, 0), (-1, -1), 5),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 5),
        ("LEFTPADDING",   (0, 0), (-1, -1), 5),
        ("RIGHTPADDING",  (0, 0), (-1, -1), 5),
    ]))
    return table


def build_loc_pdf_data(total_loc_by_translation):
    data = [["Translation", "C LOC", "Rust LOC", "Inflation (R/C)"]]
    for tt in TRANSLATION_ORDER:
        c    = total_loc_by_translation[tt]["total_c_loc"]
        rust = total_loc_by_translation[tt]["total_rust_loc"]
        inf  = rust / c if c > 0 else 0.0
        data.append([DISPLAY_TRANSLATION[tt], f"{c:,}", f"{rust:,}", f"{inf:.3f}×"])
    return data


def build_warning_pdf_data(rows):
    # Three sub-columns per translation: Raw | /Rust KLOC | /C KLOC
    header1 = ["", "Specs Only", "", "", "Code Only", "", "", "Code+Spec", "", ""]
    header2 = ["Category"] + ["Raw", "/Rust KLOC", "/C KLOC"] * 3
    data = [header1, header2]

    for cat in CATEGORY_ORDER:
        row = [cat]
        for tt in TRANSLATION_ORDER:
            entry = next(r for r in rows if r["clippy_category"] == cat and r["translation_type"] == tt)
            row += [
                str(entry["total_warnings"]),
                f"{entry['per_rust_kloc']:.2f}",
                f"{entry['per_c_kloc']:.2f}",
            ]
        data.append(row)
    return data


def write_pdf(rows, total_loc_by_translation):
    doc = SimpleDocTemplate(
        str(PDF_OUTPUT),
        pagesize=landscape(letter),
        leftMargin=0.5 * inch, rightMargin=0.5 * inch,
        topMargin=0.5 * inch,  bottomMargin=0.5 * inch,
    )

    styles = getSampleStyleSheet()

    title_style = ParagraphStyle("T", parent=styles["Title"],
                                 fontName="Helvetica-Bold", fontSize=13, spaceAfter=8)
    head_style  = ParagraphStyle("H", parent=styles["Heading2"],
                                 fontName="Helvetica-Bold", fontSize=10, spaceAfter=4)
    note_style  = ParagraphStyle("N", parent=styles["BodyText"],
                                 fontName="Helvetica", fontSize=8, spaceAfter=6)

    W = landscape(letter)[0] - inch  # usable width

    elements = [
        Paragraph("Clippy Warning Summary", title_style),
        Paragraph(
            "Columns: Raw count | Warnings per Rust KLOC | Inflation-adjusted (warnings per C KLOC = raw/Rust KLOC × Rust LOC/C LOC).",
            note_style,
        ),
        Paragraph("LOC & Size Inflation", head_style),
        make_table(build_loc_pdf_data(total_loc_by_translation),
                   col_widths=[1.6*inch, 1.2*inch, 1.2*inch, 1.4*inch]),
        Spacer(1, 0.2*inch),
        Paragraph("Clippy Warnings by Category", head_style),
    ]

    # Warning table: 1 category col + 9 data cols (3 metrics × 3 translations)
    cat_w  = 1.3 * inch
    data_w = (W - cat_w) / 9
    warning_data = build_warning_pdf_data(rows)

    # Span the translation-group headers across their 3 sub-columns
    warn_table = Table(warning_data,
                       colWidths=[cat_w] + [data_w] * 9,
                       repeatRows=2)
    warn_table.setStyle(TableStyle([
        # Span translation headers
        ("SPAN",          (1, 0), (3, 0)),
        ("SPAN",          (4, 0), (6, 0)),
        ("SPAN",          (7, 0), (9, 0)),
        ("BACKGROUND",    (0, 0), (-1, 1),  colors.whitesmoke),
        ("FONTNAME",      (0, 0), (-1, 1),  "Helvetica-Bold"),
        ("FONTNAME",      (0, 2), (-1, -1), "Helvetica"),
        ("FONTSIZE",      (0, 0), (-1, -1), 8),
        ("LEADING",       (0, 0), (-1, -1), 10),
        ("ALIGN",         (1, 0), (-1, -1), "CENTER"),
        ("ALIGN",         (0, 0), (0, -1),  "LEFT"),
        ("VALIGN",        (0, 0), (-1, -1), "MIDDLE"),
        ("GRID",          (0, 0), (-1, -1), 0.4, colors.black),
        ("LINEBELOW",     (0, 1), (-1, 1),  0.8, colors.black),
        # Light vertical dividers between translation groups
        ("LINEAFTER",     (3, 0), (3, -1),  1.0, colors.grey),
        ("LINEAFTER",     (6, 0), (6, -1),  1.0, colors.grey),
        ("TOPPADDING",    (0, 0), (-1, -1), 4),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 4),
        ("LEFTPADDING",   (0, 0), (-1, -1), 4),
        ("RIGHTPADDING",  (0, 0), (-1, -1), 4),
    ]))

    elements.append(warn_table)
    doc.build(elements)


def print_console_tables(rows, total_loc_by_translation):
    print("\n=== LOC & Size Inflation ===")
    print(f"{'Translation':<15} {'C LOC':>10} {'Rust LOC':>10} {'R/C':>8}")
    for tt in TRANSLATION_ORDER:
        c    = total_loc_by_translation[tt]["total_c_loc"]
        rust = total_loc_by_translation[tt]["total_rust_loc"]
        inf  = rust / c if c > 0 else 0.0
        print(f"{tt:<15} {c:>10,} {rust:>10,} {inf:>8.3f}×")

    print("\n=== Clippy Warnings (Raw | /Rust KLOC | /C KLOC) ===")
    col = 22
    print(f"{'Category':<13}" + "".join(f"{DISPLAY_TRANSLATION[tt]:>{col}}" for tt in TRANSLATION_ORDER))
    print("-" * (13 + col * 3))
    for cat in CATEGORY_ORDER:
        line = f"{cat:<13}"
        for tt in TRANSLATION_ORDER:
            entry = next(r for r in rows if r["clippy_category"] == cat and r["translation_type"] == tt)
            cell = fmt_cell(entry["total_warnings"], entry["per_rust_kloc"], entry["per_c_kloc"])
            line += f"{cell:>{col}}"
        print(line)


def main():
    if not LINT_CSV.exists():
        raise FileNotFoundError(f"Missing: {LINT_CSV}")
    if not LOC_CSV.exists():
        raise FileNotFoundError(f"Missing: {LOC_CSV}")

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    total_loc  = load_total_loc_by_translation()
    warn_counts = load_warning_counts()
    rows        = build_detailed_rows(total_loc, warn_counts)

    write_detailed_csv(rows)
    write_matrix_csv(rows)
    write_loc_summary_csv(total_loc)
    write_pdf(rows, total_loc)
    print_console_tables(rows, total_loc)

    print(f"\nSaved: {DETAIL_CSV}, {MATRIX_CSV}, {LOC_SUMMARY_CSV}, {PDF_OUTPUT}")


if __name__ == "__main__":
    main()