"""Render validation/report.md from the reference table."""
from pathlib import Path

from test_propagation_curves import REFERENCES


def main() -> None:
    lines = ["# Model Validation Report", "",
             "| Case | Computed | Expected | Tolerance | OK |",
             "|---|---|---|---|---|"]
    ok_all = True
    for desc, fn, expected, tol in REFERENCES:
        got = fn()
        ok = abs(got - expected) <= tol
        ok_all &= ok
        lines.append(f"| {desc} | {got:.3f} | {expected} | ±{tol} | {'✅' if ok else '❌'} |")
    Path(__file__).with_name("report.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))
    raise SystemExit(0 if ok_all else 1)


if __name__ == "__main__":
    main()
