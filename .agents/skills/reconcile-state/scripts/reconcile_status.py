#!/usr/bin/env python3
"""
reconcile_status.py — Analiza BACKLOG.md y STATE.md para reportar el progreso exacto
y determinar la siguiente tarea prioritaria al ejecutar «continúa».
"""

import argparse
import json
import re
import sys
from pathlib import Path

TASK_PATTERN = re.compile(r"^\s*-\s*\[(?P<state>[ /~vx!])\]\s*(?P<title>.+)$")


def parse_backlog(backlog_path: Path) -> dict:
    counts = {"PENDING": 0, "IN_PROGRESS": 0, "IMPLEMENTED": 0, "VERIFIED": 0, "COMPLETED": 0, "BLOCKED": 0}
    mapping = {
        " ": "PENDING",
        "/": "IN_PROGRESS",
        "~": "IMPLEMENTED",
        "v": "VERIFIED",
        "x": "COMPLETED",
        "!": "BLOCKED",
    }
    next_tasks = []
    current_section = "General"

    if not backlog_path.exists():
        return {"error": "BACKLOG.md no encontrado", "counts": counts, "next_actionable": None}

    for line_no, raw_line in enumerate(backlog_path.read_text(encoding="utf-8").splitlines(), start=1):
        if raw_line.startswith("## ") or raw_line.startswith("### "):
            current_section = raw_line.lstrip("#").strip()
            continue
        m = TASK_PATTERN.match(raw_line)
        if m:
            st_char = m.group("state")
            st_name = mapping.get(st_char, "PENDING")
            counts[st_name] += 1
            if st_name in ("IMPLEMENTED", "IN_PROGRESS", "VERIFIED", "PENDING"):
                next_tasks.append({
                    "line": line_no,
                    "section": current_section,
                    "state": st_name,
                    "symbol": f"[{st_char}]",
                    "title": m.group("title").strip(),
                })

    # Prioridad: 1) VERIFIED (cerrar a [x]), 2) IMPLEMENTED (verificar a [v]), 3) IN_PROGRESS (retomar), 4) PENDING
    priority_order = {"VERIFIED": 0, "IMPLEMENTED": 1, "IN_PROGRESS": 2, "PENDING": 3}
    next_tasks.sort(key=lambda t: (priority_order.get(t["state"], 9), t["line"]))

    total = sum(counts.values())
    pct = round((counts["COMPLETED"] / total) * 100, 1) if total > 0 else 0.0

    return {
        "total_tasks": total,
        "completed_percentage": pct,
        "counts": counts,
        "next_actionable": next_tasks[0] if next_tasks else None,
        "parallel_candidates": [t for t in next_tasks[:3] if t["state"] == "PENDING"],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Analiza BACKLOG.md y devuelve la siguiente tarea prioritaria.")
    parser.add_argument("--root", default=".", help="Raíz del repositorio")
    parser.add_argument("--json", action="store_true", help="Salida JSON")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    res = parse_backlog(root / "BACKLOG.md")
    if args.json:
        print(json.dumps(res, indent=2, ensure_ascii=False))
    else:
        print(f"Progreso total: {res['counts']['COMPLETED']} / {res.get('total_tasks', 0)} ({res.get('completed_percentage', 0)}%)")
        print(f"Desglose: {res['counts']}")
        if res.get("next_actionable"):
            na = res["next_actionable"]
            print(f"Siguiente tarea prioritaria (Línea {na['line']} - {na['section']}): {na['symbol']} {na['title']}")
        else:
            print("Todas las tareas están completadas [x] o bloqueadas [!].")
    return 0


if __name__ == "__main__":
    sys.exit(main())
