#!/usr/bin/env python3
"""
Script de adaptación en 1 paso para instalar el Sistema Universal Antigravity 2.0
junto con el perfil y el Sprint 10 de MedSys-FHIR en la raíz del repositorio MEDSYS-FHIR.
"""

import shutil
import sys
from pathlib import Path

def main() -> int:
    here = Path(__file__).resolve().parent
    universal_root = here.parent.parent
    target_root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else universal_root

    shutil.copy2(here / "project-profile.md", target_root / ".agents/rules/project-profile.md")
    backlog_dir = target_root / ".agents/backlog"
    backlog_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy2(here / "sprint_10_automatizacion_experimentos_cap4.md", backlog_dir / "sprint_10_automatizacion_experimentos_cap4.md")
    print(f"[OK] Perfil MedSys-FHIR y Sprint 10 aplicados en: {target_root}")
    return 0

if __name__ == "__main__":
    sys.exit(main())
