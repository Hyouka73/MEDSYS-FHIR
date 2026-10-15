#!/usr/bin/env python3
"""
discover_plan.py — Descubridor automático del Plan Maestro (.md) e inspector de stack
para el Sistema Universal de Orquestación Autónoma en Google Antigravity 2.0.
Opera en modo 100% de solo lectura (nunca modifica el plan original del usuario).
"""

import argparse
import json
import os
import re
import sys
from pathlib import Path

SYSTEM_MD_FILES = {
    "AGENTS.md",
    "GEMINI.md",
    "CLAUDE.md",
    "STATE.md",
    "BACKLOG.md",
    "DECISIONS.md",
    "INSTALL_AND_USAGE.md",
    "ARCHITECTURE.md",
    "COMANDOS_AGENTE.md",
}

PLAN_KEYWORDS = [
    r"objetivo", r"alcance", r"arquitectura", r"requisito", r"fase", r"sprint",
    r"tarea", r"criterio", r"aceptaci[oó]n", r"dise[ñn]o", r"pruebas", r"plan",
    r"objective", r"scope", r"architecture", r"requirement", r"phase", r"task",
]


def score_markdown_file(path: Path) -> dict:
    try:
        text = path.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return {"path": str(path), "score": -1}

    headings = re.findall(r"^#{1,4}\s+(.+)$", text, flags=re.MULTILINE)
    keyword_hits = sum(
        1 for kw in PLAN_KEYWORDS if re.search(kw, text, flags=re.IGNORECASE)
    )
    name_lower = path.name.lower()
    name_bonus = 20 if any(k in name_lower for k in ["plan", "master", "spec", "proyecto", "tesis", "roadmap"]) else 0
    score = name_bonus + (keyword_hits * 5) + min(len(headings), 30) + min(len(text) // 500, 25)
    return {
        "path": path.as_posix(),
        "size_bytes": len(text.encode("utf-8")),
        "headings_count": len(headings),
        "sample_headings": headings[:12],
        "keyword_hits": keyword_hits,
        "score": score,
    }


def detect_stack(root: Path) -> dict:
    indicators = {
        "rust": (root / "Cargo.toml").exists(),
        "node_js": (root / "package.json").exists(),
        "python": (root / "pyproject.toml").exists() or (root / "requirements.txt").exists() or any(root.glob("scripts/*.py")),
        "docker": (root / "docker-compose.yml").exists() or (root / "Dockerfile").exists(),
        "sql": any(root.glob("**/*.sql")),
        "k6": any(root.glob("**/k6/*.js")),
    }
    active = [k for k, v in indicators.items() if v]
    return {"detected_technologies": active, "is_existing_codebase": len(active) > 0}


def main() -> int:
    parser = argparse.ArgumentParser(description="Descubre el plan maestro .md e inspecciona el stack del proyecto.")
    parser.add_argument("--root", default=".", help="Directorio raíz del proyecto")
    parser.add_argument("--json", action="store_true", help="Emitir salida en formato JSON")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    candidates = []
    for md_path in root.rglob("*.md"):
        rel = md_path.relative_to(root)
        if any(part.startswith(".") or part in ("node_modules", "target", "dist", "build", "examples") for part in rel.parts):
            continue
        if md_path.name in SYSTEM_MD_FILES:
            continue
        scored = score_markdown_file(md_path)
        scored["relative_path"] = rel.as_posix()
        candidates.append(scored)

    candidates.sort(key=lambda x: x["score"], reverse=True)
    best = candidates[0] if candidates else None
    stack_info = detect_stack(root)

    result = {
        "project_root": root.as_posix(),
        "best_master_plan_candidate": best,
        "other_markdown_candidates": candidates[1:5],
        "stack_summary": stack_info,
    }

    if args.json:
        print(json.dumps(result, indent=2, ensure_ascii=False))
    else:
        print(f"Proyecto: {root.name}")
        if best:
            print(f"Plan Maestro detectado: {best['relative_path']} (score={best['score']}, secciones={best['headings_count']})")
        else:
            print("No se encontró un archivo .md externo de plan maestro (usando BACKLOG.md existente).")
        print(f"Tecnologías detectadas: {', '.join(stack_info['detected_technologies']) or 'Ninguna (Proyecto en blanco)'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
