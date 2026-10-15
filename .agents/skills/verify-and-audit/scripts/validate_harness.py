#!/usr/bin/env python3
"""
validate_harness.py — Validador de integridad y consistencia del sistema de orquestación
autónoma para Google Antigravity 2.0.
Comprueba existencia de archivos, metadatos YAML frontmatter, referencias internas
y ausencia de rutas absolutas privadas.
"""

import argparse
import json
import re
import sys
from pathlib import Path

REQUIRED_FILES = [
    "AGENTS.md",
    "GEMINI.md",
    ".gemini/config.json",
    "STATE.md",
    "BACKLOG.md",
    "DECISIONS.md",
    ".agents/rules/00-orchestrator-core.md",
    ".agents/rules/01-safety-and-autonomy.md",
    ".agents/rules/02-delegation-and-concurrency.md",
    ".agents/rules/03-quota-and-context.md",
    ".agents/rules/project-profile.md",
    ".agents/agents/planner-analyst/agent.md",
    ".agents/agents/architect-researcher/agent.md",
    ".agents/agents/implementer/agent.md",
    ".agents/agents/tester-qa/agent.md",
    ".agents/agents/reviewer-auditor/agent.md",
    ".agents/agents/documenter-integrator/agent.md",
    ".agents/skills/init-from-plan/SKILL.md",
    ".agents/skills/init-from-plan/scripts/discover_plan.py",
    ".agents/skills/reconcile-state/SKILL.md",
    ".agents/skills/reconcile-state/scripts/reconcile_status.py",
    ".agents/skills/delegate-subagent/SKILL.md",
    ".agents/skills/delegate-subagent/resources/delegation_contract_template.md",
    ".agents/skills/verify-and-audit/SKILL.md",
    ".agents/skills/verify-and-audit/scripts/validate_harness.py",
    ".agents/skills/recover-session/SKILL.md",
    ".agents/skills/close-milestone/SKILL.md",
    ".agents/workflows/continue.md",
    ".agents/workflows/init.md",
    ".agents/workflows/plan.md",
    ".agents/workflows/ask.md",
    ".agents/workflows/verify.md",
    ".agents/workflows/status.md",
]

FORBIDDEN_PATTERNS = [
    re.compile(r"C:\\Users\\", re.IGNORECASE),
    re.compile(r"/Users/[a-zA-Z0-9_-]+/"),
    re.compile(r"AIza[0-9A-Za-z-_]{35}"),
    re.compile(r"ghp_[0-9A-Za-z]{36}"),
]


def has_valid_frontmatter(path: Path, required_keys: list[str]) -> tuple[bool, str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return False, f"{path.as_posix()} no inicia con YAML frontmatter '---'"
    end_idx = text.find("\n---", 4)
    if end_idx == -1:
        return False, f"{path.as_posix()} no cierra el bloque YAML frontmatter '---'"
    fm_block = text[4:end_idx]
    for key in required_keys:
        if not re.search(rf"^{re.escape(key)}\s*:", fm_block, flags=re.MULTILINE):
            return False, f"{path.as_posix()} carece del campo obligatorio '{key}:' en su frontmatter"
    return True, "OK"


def validate_workspace(root: Path) -> dict:
    errors = []
    warnings = []

    # 1. Existencia de archivos requeridos
    for rel in REQUIRED_FILES:
        p = root / rel
        if not p.exists():
            errors.append(f"Archivo requerido faltante: {rel}")

    # 2. Validación de JSON en .gemini/config.json
    cfg_path = root / ".gemini/config.json"
    if cfg_path.exists():
        try:
            json.loads(cfg_path.read_text(encoding="utf-8"))
        except Exception as e:
            errors.append(f".gemini/config.json inválido: {e}")

    # 3. Validación de Frontmatters en rules, skills, agents y workflows
    for rule_file in (root / ".agents/rules").glob("*.md"):
        ok, msg = has_valid_frontmatter(rule_file, ["trigger"])
        if not ok:
            errors.append(msg)

    for skill_file in (root / ".agents/skills").glob("*/SKILL.md"):
        ok, msg = has_valid_frontmatter(skill_file, ["name", "description"])
        if not ok:
            errors.append(msg)

    for agent_file in (root / ".agents/agents").glob("*/agent.md"):
        ok, msg = has_valid_frontmatter(agent_file, ["name", "description", "kind", "subagent"])
        if not ok:
            errors.append(msg)

    for wf_file in (root / ".agents/workflows").glob("*.md"):
        ok, msg = has_valid_frontmatter(wf_file, ["description"])
        if not ok:
            errors.append(msg)

    # 4. Escaneo de rutas absolutas privadas o secretos en archivos del sistema
    scan_targets = [root / "AGENTS.md", root / "GEMINI.md", root / "STATE.md", root / "BACKLOG.md", root / "DECISIONS.md"]
    scan_targets.extend((root / ".agents").rglob("*.md"))
    for fpath in scan_targets:
        if not fpath.exists():
            continue
        content = fpath.read_text(encoding="utf-8", errors="ignore")
        for pat in FORBIDDEN_PATTERNS:
            if pat.search(content):
                errors.append(f"Patrón prohibido (ruta absoluta local o secreto) detectado en {fpath.relative_to(root).as_posix()}")

    return {
        "valid": len(errors) == 0,
        "checked_files_count": len(REQUIRED_FILES),
        "errors": errors,
        "warnings": warnings,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Valida la consistencia del harness Antigravity 2.0.")
    parser.add_argument("--root", default=".", help="Directorio raíz a validar")
    parser.add_argument("--json", action="store_true", help="Emitir salida JSON")
    args = parser.parse_args()

    res = validate_workspace(Path(args.root).resolve())
    if args.json:
        print(json.dumps(res, indent=2, ensure_ascii=False))
    else:
        if res["valid"]:
            print(f"[OK] Harness Antigravity 2.0 validado exitosamente ({res['checked_files_count']} componentes clave verificados, 0 errores).")
        else:
            print("[ERROR] Se detectaron inconsistencias en la configuración:")
            for err in res["errors"]:
                print(f"  - {err}")
    return 0 if res["valid"] else 1


if __name__ == "__main__":
    sys.exit(main())
