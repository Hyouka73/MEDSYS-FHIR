#!/usr/bin/env python3
"""
Suite de pruebas automatizadas de consistencia para el Sistema Universal de Orquestación
Autónoma en Google Antigravity 2.0 (Verifica los 12 puntos de la Sección 12 de la Misión Maestra).
"""

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


class TestAntigravityV2UniversalHarness(unittest.TestCase):
    def test_01_validate_harness_script_passes_zero_errors(self):
        script = ROOT / ".agents/skills/verify-and-audit/scripts/validate_harness.py"
        res = subprocess.run([sys.executable, str(script), "--root", str(ROOT), "--json"], capture_output=True, text=True, check=True)
        data = json.loads(res.stdout)
        self.assertTrue(data["valid"], f"Errores en validate_harness.py: {data['errors']}")
        self.assertEqual(len(data["errors"]), 0)

    def test_02_universal_core_has_no_medsys_coupling(self):
        universal_files = [
            ROOT / "AGENTS.md",
            ROOT / "GEMINI.md",
            ROOT / ".agents/rules/00-orchestrator-core.md",
            ROOT / ".agents/rules/01-safety-and-autonomy.md",
            ROOT / ".agents/rules/02-delegation-and-concurrency.md",
            ROOT / ".agents/rules/03-quota-and-context.md",
        ]
        for f in universal_files:
            text = f.read_text(encoding="utf-8")
            self.assertNotIn("MedSys-FHIR", text, f"El archivo universal {f.name} no debe estar acoplado a MedSys-FHIR")
            self.assertNotIn("NOM-004", text, f"El archivo universal {f.name} no debe contener normativa específica de la tesis")

    def test_03_discover_plan_preserves_original_md_untouched(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            tmp = Path(tmpdir)
            plan_file = tmp / "MI_PLAN_UNICO.md"
            original_content = "# Plan de Proyecto X\n\n## 1. Objetivo\nConstruir API.\n\n## 2. Fases y Tareas\n- Fase 1\n"
            plan_file.write_text(original_content, encoding="utf-8")

            script = ROOT / ".agents/skills/init-from-plan/scripts/discover_plan.py"
            res = subprocess.run([sys.executable, str(script), "--root", str(tmp), "--json"], capture_output=True, text=True, check=True)
            out = json.loads(res.stdout)
            self.assertIsNotNone(out["best_master_plan_candidate"])
            self.assertEqual(out["best_master_plan_candidate"]["relative_path"], "MI_PLAN_UNICO.md")
            # Verificar que el plan original jamás fue modificado
            self.assertEqual(plan_file.read_text(encoding="utf-8"), original_content)

    def test_04_reconcile_status_distinguishes_all_six_states(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            tmp = Path(tmpdir)
            backlog = tmp / "BACKLOG.md"
            backlog.write_text(
                "# Backlog\n"
                "## Fase 1\n"
                "- [x] Tarea 1 completada\n"
                "- [v] Tarea 2 verificada\n"
                "- [~] Tarea 3 implementada\n"
                "- [/] Tarea 4 en curso\n"
                "- [ ] Tarea 5 pendiente\n"
                "- [!] Tarea 6 bloqueada\n",
                encoding="utf-8",
            )
            script = ROOT / ".agents/skills/reconcile-state/scripts/reconcile_status.py"
            res = subprocess.run([sys.executable, str(script), "--root", str(tmp), "--json"], capture_output=True, text=True, check=True)
            data = json.loads(res.stdout)
            self.assertEqual(data["total_tasks"], 6)
            self.assertEqual(data["counts"]["COMPLETED"], 1)
            self.assertEqual(data["counts"]["VERIFIED"], 1)
            self.assertEqual(data["counts"]["IMPLEMENTED"], 1)
            self.assertEqual(data["counts"]["IN_PROGRESS"], 1)
            self.assertEqual(data["counts"]["PENDING"], 1)
            self.assertEqual(data["counts"]["BLOCKED"], 1)
            # La prioridad número 1 debe ser cerrar la VERIFIED [v] antes de abrir nuevas
            self.assertEqual(data["next_actionable"]["state"], "VERIFIED")

    def test_05_rules_token_budget_under_limit(self):
        total_chars = sum(len(p.read_text(encoding="utf-8")) for p in (ROOT / ".agents/rules").glob("*.md"))
        estimated_tokens = total_chars // 4
        self.assertLess(estimated_tokens, 5000, "Las reglas deben mantenerse ligeras (< 5,000 tokens de los 20,000 permitidos)")


if __name__ == "__main__":
    unittest.main(verbosity=2)
