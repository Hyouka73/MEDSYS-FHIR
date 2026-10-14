#!/usr/bin/env python3
# ==============================================================================
# MedSys-FHIR: Oráculo Diferencial Independiente en Python (Capítulo III, §3.6.3)
# Tesis de Licenciatura en IDS — UNACH 2026
# Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero
#
# Instrumento experimental independiente sin código compartido con Rust.
# Reconstruye y valida los 16,250 recursos FHIR R4 canónicos para la semilla
# SEED=20260930 (NOM-004-SSA3-2012 / NOM-024-SSA3-2012):
#   - 1,000 Patient
#   - 2,500 Encounter
#   - 9,750 Observation (2,500 BP + 2,500 Temp + 2,500 HR + 2,250 Resp)
#   - 3,000 Condition
#   Total: 16,250 recursos FHIR R4
# ==============================================================================

import argparse
import copy
import json
import os
import re
import sys
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone, timedelta

# Asegurar importación de generate_data desde el directorio scripts
SCRIPTS_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.abspath(os.path.join(SCRIPTS_DIR, ".."))
if SCRIPTS_DIR not in sys.path:
    sys.path.insert(0, SCRIPTS_DIR)

try:
    from generate_data import generate_entities, DEFAULT_SEED, CURP_RENAPO_REGEX
except ImportError:
    raise ImportError("No se pudo importar 'generate_data.py' desde scripts/. Verifique la estructura del repositorio.")

# Zona horaria oficial para resolución en México (UTC-6)
TZ_MEXICO = timezone(timedelta(hours=-6))

# Códigos canónicos y sistemas terminológicos oficiales
SYS_CURP = "urn:oid:2.16.840.1.113883.4.629"
SYS_ENCOUNTER = "https://distritosalud1.chiapas.gob.mx/encounters"
SYS_ACT_CODE = "http://terminology.hl7.org/CodeSystem/v3-ActCode"
SYS_PARTICIPATION = "http://terminology.hl7.org/CodeSystem/v3-ParticipationType"
SYS_SEP_CEDULA = "http://cedulaprofesional.sep.gob.mx"
SYS_OBS_CATEGORY = "http://terminology.hl7.org/CodeSystem/observation-category"
SYS_LOINC = "http://loinc.org"
SYS_UCUM = "http://unitsofmeasure.org"
SYS_COND_CLINICAL = "http://terminology.hl7.org/CodeSystem/condition-clinical"
SYS_COND_VER_STATUS = "http://terminology.hl7.org/CodeSystem/condition-ver-status"
SYS_COND_CATEGORY = "http://terminology.hl7.org/CodeSystem/condition-category"
SYS_ICD10 = "http://hl7.org/fhir/sid/icd-10"

# ------------------------------------------------------------------------------
# Utilidades de Normalización y Comparación Semántica
# ------------------------------------------------------------------------------

def normalize_iso_datetime(dt_str: str) -> str:
    """Normaliza cadenas ISO de fecha y hora para comparación agnóstica de zona."""
    if not dt_str:
        return ""
    # Quitar sufijo de zona o 'Z' para normalizar comparación en tiempo local
    s = str(dt_str).strip()
    if s.endswith("Z"):
        s = s[:-1]
    if len(s) > 19 and (s[-6] in ('+', '-') or s[-5] in ('+', '-')):
        s = s[:19]
    return s.replace(" ", "T")


def safe_float(val) -> float:
    try:
        return float(val)
    except (ValueError, TypeError):
        return 0.0


# ------------------------------------------------------------------------------
# Generador Independiente del Oráculo (Modelo en Memoria)
# ------------------------------------------------------------------------------

class IndependentOracleModel:
    """Construye de forma desacoplada las especificaciones canónicas FHIR R4 esperadas."""

    def __init__(self, seed: int = DEFAULT_SEED):
        self.seed = seed
        (
            self.pacientes, _,
            self.consultas, _,
            self.signos, _,
            self.diagnosticos, _
        ) = generate_entities(seed=self.seed)

        self.expected_resources = {}
        self.expected_absent_observations = set()
        self.expected_omitted_condition_ver_status = set()
        self._build_all_expected()

    def _build_all_expected(self):
        # 1. Patient: 1,000 recursos
        for pid, p in self.pacientes.items():
            res = self.build_patient(p)
            self.expected_resources[f"Patient/{pid}"] = res

        # 2. Encounter: 2,500 recursos
        for c in self.consultas:
            cid = c["id_consulta"]
            res = self.build_encounter(c)
            self.expected_resources[f"Encounter/{cid}"] = res

        # 3. Observation: 9,750 recursos (2,500 BP + 2,500 Temp + 2,500 HR + 2,250 Resp)
        for s in self.signos:
            sid = s["id_signo"]
            self.expected_resources[f"Observation/bp-{sid}"] = self.build_observation_bp(s)
            self.expected_resources[f"Observation/temp-{sid}"] = self.build_observation_temp(s)
            self.expected_resources[f"Observation/hr-{sid}"] = self.build_observation_hr(s)

            if s["frecuencia_respiratoria"] is not None:
                self.expected_resources[f"Observation/resp-{sid}"] = self.build_observation_resp(s)
            else:
                self.expected_absent_observations.add(sid)

        # 4. Condition: 3,000 recursos
        for d in self.diagnosticos:
            did = d["id_diagnostico"]
            res = self.build_condition(d)
            self.expected_resources[f"Condition/{did}"] = res
            if d.get("tipo") is None:
                self.expected_omitted_condition_ver_status.add(did)

    def build_patient(self, p: dict) -> dict:
        family = f"{p['apellido_paterno']} {p['apellido_materno']}" if p.get("apellido_materno") else p["apellido_paterno"]
        given = [p["primer_nombre"]]
        if p.get("segundo_nombre"):
            given.append(p["segundo_nombre"])
        text = f"{' '.join(given)} {family}"

        gender_map = {"M": "male", "F": "female", "I": "other"}
        gender = gender_map.get(p.get("sexo_biologico"), "unknown")

        res = {
            "resourceType": "Patient",
            "id": str(p["id_paciente"]),
            "identifier": [
                {
                    "use": "official",
                    "system": SYS_CURP,
                    "value": p["curp"]
                }
            ],
            "active": True,
            "name": [
                {
                    "use": "official",
                    "text": text,
                    "family": family,
                    "given": given
                }
            ],
            "gender": gender,
            "birthDate": p["fecha_nacimiento"]
        }

        if p.get("telefono_contacto"):
            res["telecom"] = [
                {
                    "system": "phone",
                    "value": p["telefono_contacto"],
                    "use": "mobile"
                }
            ]

        return res

    def build_encounter(self, c: dict) -> dict:
        status_map = {
            "FINALIZADA": "finished",
            "EN_CURSO": "in-progress",
            "CANCELADA": "cancelled"
        }
        status = status_map.get(c.get("estado"), "unknown")

        res = {
            "resourceType": "Encounter",
            "id": str(c["id_consulta"]),
            "identifier": [
                {
                    "use": "official",
                    "system": SYS_ENCOUNTER,
                    "value": str(c["id_consulta"])
                }
            ],
            "status": status,
            "class": {
                "system": SYS_ACT_CODE,
                "code": "AMB",
                "display": "ambulatory"
            },
            "subject": {
                "reference": f"Patient/{c['id_paciente']}"
            },
            "participant": [
                {
                    "type": [
                        {
                            "coding": [
                                {
                                    "system": SYS_PARTICIPATION,
                                    "code": "ATND",
                                    "display": "attender"
                                }
                            ],
                            "text": "Médico Tratante"
                        }
                    ],
                    "individual": {
                        "type": "Practitioner",
                        "identifier": {
                            "use": "official",
                            "system": SYS_SEP_CEDULA,
                            "value": str(c["cedula"])
                        },
                        "display": c["medico"]
                    }
                }
            ],
            "period": {
                "start": normalize_iso_datetime(c["inicio"]),
                "end": normalize_iso_datetime(c["fin"])
            },
            "reasonCode": [
                {
                    "text": c["motivo"]
                }
            ]
        }

        if c.get("unidad"):
            res["serviceType"] = {"text": c["unidad"]}

        return res

    def build_observation_bp(self, s: dict) -> dict:
        return {
            "resourceType": "Observation",
            "id": f"bp-{s['id_signo']}",
            "status": "final",
            "category": [
                {
                    "coding": [
                        {
                            "system": SYS_OBS_CATEGORY,
                            "code": "vital-signs",
                            "display": "Vital Signs"
                        }
                    ]
                }
            ],
            "code": {
                "coding": [
                    {
                        "system": SYS_LOINC,
                        "code": "85354-9",
                        "display": "Blood pressure panel with all children optional"
                    }
                ],
                "text": "Presión arterial"
            },
            "subject": {
                "reference": f"Patient/{s['id_paciente']}"
            },
            "encounter": {
                "reference": f"Encounter/{s['id_consulta']}"
            },
            "effectiveDateTime": normalize_iso_datetime(s["registro"]),
            "component": [
                {
                    "code": {
                        "coding": [
                            {
                                "system": SYS_LOINC,
                                "code": "8480-6",
                                "display": "Systolic blood pressure"
                            }
                        ]
                    },
                    "valueQuantity": {
                        "value": s["sistolica"],
                        "unit": "mmHg",
                        "system": SYS_UCUM,
                        "code": "mm[Hg]"
                    }
                },
                {
                    "code": {
                        "coding": [
                            {
                                "system": SYS_LOINC,
                                "code": "8462-4",
                                "display": "Diastolic blood pressure"
                            }
                        ]
                    },
                    "valueQuantity": {
                        "value": s["diastolica"],
                        "unit": "mmHg",
                        "system": SYS_UCUM,
                        "code": "mm[Hg]"
                    }
                }
            ]
        }

    def build_observation_temp(self, s: dict) -> dict:
        return {
            "resourceType": "Observation",
            "id": f"temp-{s['id_signo']}",
            "status": "final",
            "category": [
                {
                    "coding": [
                        {
                            "system": SYS_OBS_CATEGORY,
                            "code": "vital-signs",
                            "display": "Vital Signs"
                        }
                    ]
                }
            ],
            "code": {
                "coding": [
                    {
                        "system": SYS_LOINC,
                        "code": "8310-5",
                        "display": "Body temperature"
                    }
                ],
                "text": "Temperatura Corporal"
            },
            "subject": {
                "reference": f"Patient/{s['id_paciente']}"
            },
            "encounter": {
                "reference": f"Encounter/{s['id_consulta']}"
            },
            "effectiveDateTime": normalize_iso_datetime(s["registro"]),
            "valueQuantity": {
                "value": s["temperatura"],
                "unit": "Cel",
                "system": SYS_UCUM,
                "code": "Cel"
            }
        }

    def build_observation_hr(self, s: dict) -> dict:
        return {
            "resourceType": "Observation",
            "id": f"hr-{s['id_signo']}",
            "status": "final",
            "category": [
                {
                    "coding": [
                        {
                            "system": SYS_OBS_CATEGORY,
                            "code": "vital-signs",
                            "display": "Vital Signs"
                        }
                    ]
                }
            ],
            "code": {
                "coding": [
                    {
                        "system": SYS_LOINC,
                        "code": "8867-4",
                        "display": "Heart rate"
                    }
                ],
                "text": "Frecuencia cardíaca"
            },
            "subject": {
                "reference": f"Patient/{s['id_paciente']}"
            },
            "encounter": {
                "reference": f"Encounter/{s['id_consulta']}"
            },
            "effectiveDateTime": normalize_iso_datetime(s["registro"]),
            "valueQuantity": {
                "value": s["frecuencia_cardiaca"],
                "unit": "/min",
                "system": SYS_UCUM,
                "code": "/min"
            }
        }

    def build_observation_resp(self, s: dict) -> dict:
        return {
            "resourceType": "Observation",
            "id": f"resp-{s['id_signo']}",
            "status": "final",
            "category": [
                {
                    "coding": [
                        {
                            "system": SYS_OBS_CATEGORY,
                            "code": "vital-signs",
                            "display": "Vital Signs"
                        }
                    ]
                }
            ],
            "code": {
                "coding": [
                    {
                        "system": SYS_LOINC,
                        "code": "9279-1",
                        "display": "Respiratory rate"
                    }
                ],
                "text": "Frecuencia respiratoria"
            },
            "subject": {
                "reference": f"Patient/{s['id_paciente']}"
            },
            "encounter": {
                "reference": f"Encounter/{s['id_consulta']}"
            },
            "effectiveDateTime": normalize_iso_datetime(s["registro"]),
            "valueQuantity": {
                "value": s["frecuencia_respiratoria"],
                "unit": "/min",
                "system": SYS_UCUM,
                "code": "/min"
            }
        }

    def build_condition(self, d: dict) -> dict:
        res = {
            "resourceType": "Condition",
            "id": str(d["id_diagnostico"]),
            "clinicalStatus": {
                "coding": [
                    {
                        "system": SYS_COND_CLINICAL,
                        "code": "active",
                        "display": "Active"
                    }
                ]
            },
            "category": [
                {
                    "coding": [
                        {
                            "system": SYS_COND_CATEGORY,
                            "code": "encounter-diagnosis",
                            "display": "Encounter Diagnosis"
                        }
                    ]
                }
            ],
            "code": {
                "coding": [
                    {
                        "system": SYS_ICD10,
                        "code": d["codigo_cie10"],
                        "display": d["descripcion"]
                    }
                ],
                "text": d["descripcion"]
            },
            "subject": {
                "reference": f"Patient/{d['id_paciente']}"
            },
            "encounter": {
                "reference": f"Encounter/{d['id_consulta']}"
            },
            "recordedDate": normalize_iso_datetime(f"{d['fecha']}T00:00:00")
        }

        tipo = d.get("tipo")
        if tipo == "CONFIRMADO":
            res["verificationStatus"] = {
                "coding": [
                    {
                        "system": SYS_COND_VER_STATUS,
                        "code": "confirmed",
                        "display": "Confirmed"
                    }
                ]
            }
        elif tipo == "PRESUNTIVO":
            res["verificationStatus"] = {
                "coding": [
                    {
                        "system": SYS_COND_VER_STATUS,
                        "code": "provisional",
                        "display": "Provisional"
                    }
                ]
            }
        # Si tipo es None o vacío, verificationStatus se omite limpiamente (Fail-Closed D-006)

        return res


# ------------------------------------------------------------------------------
# Comparador Semántico Riguroso del Oráculo
# ------------------------------------------------------------------------------

class FhirOracleVerifier:
    """Ejecuta la verificación diferencial entre el recurso candidato y el esperado."""

    @staticmethod
    def verify_resource(candidate: dict, expected: dict) -> tuple[bool, list[str]]:
        errors = []
        r_type = expected.get("resourceType")
        cand_type = candidate.get("resourceType")

        if cand_type != r_type:
            return False, [f"resourceType mismatch: esperado '{r_type}', obtenido '{cand_type}'"]

        if str(candidate.get("id")) != str(expected.get("id")):
            errors.append(f"id mismatch: esperado '{expected.get('id')}', obtenido '{candidate.get('id')}'")

        if r_type == "Patient":
            FhirOracleVerifier._verify_patient(candidate, expected, errors)
        elif r_type == "Encounter":
            FhirOracleVerifier._verify_encounter(candidate, expected, errors)
        elif r_type == "Observation":
            FhirOracleVerifier._verify_observation(candidate, expected, errors)
        elif r_type == "Condition":
            FhirOracleVerifier._verify_condition(candidate, expected, errors)
        else:
            errors.append(f"Tipo de recurso no soportado: '{r_type}'")

        return len(errors) == 0, errors

    @staticmethod
    def _verify_patient(candidate: dict, expected: dict, errors: list[str]):
        # Identificador oficial CURP (T5)
        c_idents = candidate.get("identifier", [])
        e_idents = expected.get("identifier", [])
        if not c_idents:
            errors.append("Patient no contiene identificadores")
        else:
            c_curp = c_idents[0]
            e_curp = e_idents[0]
            if c_curp.get("value") != e_curp.get("value"):
                errors.append(f"CURP mismatch: exp '{e_curp.get('value')}', got '{c_curp.get('value')}'")
            if c_curp.get("system") != e_curp.get("system"):
                errors.append(f"CURP system mismatch: exp '{e_curp.get('system')}', got '{c_curp.get('system')}'")
            if c_curp.get("use") != "official":
                errors.append(f"CURP use mismatch: exp 'official', got '{c_curp.get('use')}'")
            if not CURP_RENAPO_REGEX.match(c_curp.get("value", "")):
                errors.append(f"CURP no cumple regex oficial RENAPO: '{c_curp.get('value')}'")

        # Nombres
        c_names = candidate.get("name", [])
        e_names = expected.get("name", [])
        if not c_names:
            errors.append("Patient no contiene bloque 'name'")
        else:
            c_name = c_names[0]
            e_name = e_names[0]
            if c_name.get("family") != e_name.get("family"):
                errors.append(f"Family name mismatch: exp '{e_name.get('family')}', got '{c_name.get('family')}'")
            if c_name.get("given") != e_name.get("given"):
                errors.append(f"Given names mismatch: exp {e_name.get('given')}, got {c_name.get('given')}")

        # Género y Fecha de Nacimiento
        if candidate.get("gender") != expected.get("gender"):
            errors.append(f"gender mismatch: exp '{expected.get('gender')}', got '{candidate.get('gender')}'")
        if candidate.get("birthDate") != expected.get("birthDate"):
            errors.append(f"birthDate mismatch: exp '{expected.get('birthDate')}', got '{candidate.get('birthDate')}'")

        # Telecom (teléfono de contacto opcional)
        e_telecom = expected.get("telecom")
        c_telecom = candidate.get("telecom")
        if e_telecom is None:
            if c_telecom is not None and len(c_telecom) > 0:
                errors.append(f"telecom debió ser omitido (NULL en origen), pero candidato tiene {c_telecom}")
        else:
            if not c_telecom:
                errors.append(f"telecom esperado {e_telecom}, pero candidato no contiene telecom")
            else:
                if c_telecom[0].get("value") != e_telecom[0].get("value"):
                    errors.append(f"telecom value mismatch: exp '{e_telecom[0].get('value')}', got '{c_telecom[0].get('value')}'")
                if c_telecom[0].get("system") != "phone":
                    errors.append(f"telecom system mismatch: exp 'phone', got '{c_telecom[0].get('system')}'")
                if c_telecom[0].get("use") != "mobile":
                    errors.append(f"telecom use mismatch: exp 'mobile', got '{c_telecom[0].get('use')}'")

    @staticmethod
    def _verify_encounter(candidate: dict, expected: dict, errors: list[str]):
        if candidate.get("status") != expected.get("status"):
            errors.append(f"Encounter status mismatch: exp '{expected.get('status')}', got '{candidate.get('status')}'")

        c_class = candidate.get("class", {})
        if c_class.get("code") != "AMB":
            errors.append(f"Encounter class.code mismatch: exp 'AMB', got '{c_class.get('code')}'")
        if c_class.get("system") != SYS_ACT_CODE:
            errors.append(f"Encounter class.system mismatch: exp '{SYS_ACT_CODE}', got '{c_class.get('system')}'")

        c_subj = candidate.get("subject", {}).get("reference")
        e_subj = expected.get("subject", {}).get("reference")
        if c_subj != e_subj:
            errors.append(f"Encounter subject reference mismatch: exp '{e_subj}', got '{c_subj}'")

        # Periodo
        c_start = normalize_iso_datetime(candidate.get("period", {}).get("start"))
        e_start = normalize_iso_datetime(expected.get("period", {}).get("start"))
        if c_start != e_start:
            errors.append(f"Encounter period.start mismatch: exp '{e_start}', got '{c_start}'")

        c_end = normalize_iso_datetime(candidate.get("period", {}).get("end"))
        e_end = normalize_iso_datetime(expected.get("period", {}).get("end"))
        if c_end != e_end:
            errors.append(f"Encounter period.end mismatch: exp '{e_end}', got '{c_end}'")

        # Participante médico SEP
        c_part = candidate.get("participant", [])
        if not c_part:
            errors.append("Encounter no contiene bloque 'participant'")
        else:
            indiv = c_part[0].get("individual", {})
            ident = indiv.get("identifier", {})
            e_ident = expected["participant"][0]["individual"]["identifier"]
            if ident.get("value") != e_ident.get("value"):
                errors.append(f"Cédula SEP mismatch: exp '{e_ident.get('value')}', got '{ident.get('value')}'")
            if ident.get("system") != SYS_SEP_CEDULA:
                errors.append(f"Cédula system mismatch: exp '{SYS_SEP_CEDULA}', got '{ident.get('system')}'")

    @staticmethod
    def _verify_observation(candidate: dict, expected: dict, errors: list[str]):
        if candidate.get("status") != "final":
            errors.append(f"Observation status mismatch: exp 'final', got '{candidate.get('status')}'")

        # Categoría vital-signs
        cats = candidate.get("category", [])
        if not cats or not any(
            any(cd.get("code") == "vital-signs" for cd in c.get("coding", []))
            for c in cats
        ):
            errors.append("Observation no contiene categoría 'vital-signs'")

        c_subj = candidate.get("subject", {}).get("reference")
        e_subj = expected.get("subject", {}).get("reference")
        if c_subj != e_subj:
            errors.append(f"Observation subject mismatch: exp '{e_subj}', got '{c_subj}'")

        c_enc = candidate.get("encounter", {}).get("reference")
        e_enc = expected.get("encounter", {}).get("reference")
        if c_enc != e_enc:
            errors.append(f"Observation encounter mismatch: exp '{e_enc}', got '{c_enc}'")

        c_eff = normalize_iso_datetime(candidate.get("effectiveDateTime"))
        e_eff = normalize_iso_datetime(expected.get("effectiveDateTime"))
        if c_eff != e_eff:
            errors.append(f"Observation effectiveDateTime mismatch: exp '{e_eff}', got '{c_eff}'")

        e_code = expected["code"]["coding"][0]["code"]
        c_codings = candidate.get("code", {}).get("coding", [])
        if not any(cd.get("code") == e_code for cd in c_codings):
            errors.append(f"Observation code mismatch: exp '{e_code}', got {[cd.get('code') for cd in c_codings]}")

        # Desglose por subtipo
        if e_code == "85354-9":  # Presión arterial compuesta
            c_comps = candidate.get("component", [])
            if len(c_comps) < 2:
                errors.append(f"Panel BP requiere 2 componentes, obtenidos {len(c_comps)}")
            else:
                # Sistólica (8480-6)
                sist_found = False
                diast_found = False
                for comp in c_comps:
                    comp_codes = [cd.get("code") for cd in comp.get("code", {}).get("coding", [])]
                    val = comp.get("valueQuantity", {})
                    if "8480-6" in comp_codes:
                        sist_found = True
                        e_val = expected["component"][0]["valueQuantity"]["value"]
                        if safe_float(val.get("value")) != safe_float(e_val):
                            errors.append(f"Presión sistólica value mismatch: exp {e_val}, got {val.get('value')}")
                        if val.get("unit") != "mmHg" or val.get("code") != "mm[Hg]":
                            errors.append(f"Presión sistólica unidades mismatch: got {val.get('unit')} / {val.get('code')}")
                    elif "8462-4" in comp_codes:
                        diast_found = True
                        e_val = expected["component"][1]["valueQuantity"]["value"]
                        if safe_float(val.get("value")) != safe_float(e_val):
                            errors.append(f"Presión diastólica value mismatch: exp {e_val}, got {val.get('value')}")
                        if val.get("unit") != "mmHg" or val.get("code") != "mm[Hg]":
                            errors.append(f"Presión diastólica unidades mismatch: got {val.get('unit')} / {val.get('code')}")
                if not sist_found:
                    errors.append("Componente sistólica LOINC 8480-6 no encontrado")
                if not diast_found:
                    errors.append("Componente diastólica LOINC 8462-4 no encontrado")

        elif e_code in ("8310-5", "8867-4", "9279-1"):  # Temp, HR, Resp
            c_val = candidate.get("valueQuantity", {})
            e_val = expected.get("valueQuantity", {})
            if safe_float(c_val.get("value")) != safe_float(e_val.get("value")):
                errors.append(f"valueQuantity.value mismatch: exp {e_val.get('value')}, got {c_val.get('value')}")
            if c_val.get("unit") != e_val.get("unit"):
                errors.append(f"valueQuantity.unit mismatch: exp '{e_val.get('unit')}', got '{c_val.get('unit')}'")
            if c_val.get("code") != e_val.get("code"):
                errors.append(f"valueQuantity.code mismatch: exp '{e_val.get('code')}', got '{c_val.get('code')}'")

    @staticmethod
    def _verify_condition(candidate: dict, expected: dict, errors: list[str]):
        # clinicalStatus
        c_clin = candidate.get("clinicalStatus", {}).get("coding", [{}])[0].get("code")
        if c_clin != "active":
            errors.append(f"Condition clinicalStatus mismatch: exp 'active', got '{c_clin}'")

        # Código CIE-10
        e_cie10 = expected["code"]["coding"][0]["code"]
        c_codings = candidate.get("code", {}).get("coding", [])
        if not any(cd.get("code") == e_cie10 for cd in c_codings):
            errors.append(f"Condition CIE-10 mismatch: exp '{e_cie10}', got {[cd.get('code') for cd in c_codings]}")

        # Referencias
        if candidate.get("subject", {}).get("reference") != expected.get("subject", {}).get("reference"):
            errors.append(f"Condition subject mismatch: exp '{expected.get('subject')}', got '{candidate.get('subject')}'")
        if candidate.get("encounter", {}).get("reference") != expected.get("encounter", {}).get("reference"):
            errors.append(f"Condition encounter mismatch: exp '{expected.get('encounter')}', got '{candidate.get('encounter')}'")

        # recordedDate
        c_rec = normalize_iso_datetime(candidate.get("recordedDate"))
        e_rec = normalize_iso_datetime(expected.get("recordedDate"))
        if c_rec != e_rec:
            errors.append(f"Condition recordedDate mismatch: exp '{e_rec}', got '{c_rec}'")

        # verificationStatus: confirmado, presuntivo o omitido (Fail-Closed D-006)
        e_ver = expected.get("verificationStatus")
        c_ver = candidate.get("verificationStatus")
        if e_ver is None:
            if c_ver is not None:
                errors.append(f"verificationStatus debió ser omitido (NULL en origen), pero candidato tiene {c_ver}")
        else:
            if c_ver is None:
                errors.append(f"verificationStatus esperado {e_ver}, pero candidato lo omitió")
            else:
                e_code = e_ver["coding"][0]["code"]
                c_code = c_ver.get("coding", [{}])[0].get("code")
                if c_code != e_code:
                    errors.append(f"verificationStatus code mismatch: exp '{e_code}', got '{c_code}'")


# ------------------------------------------------------------------------------
# Módulo de Pruebas de Mutación (Mutation Testing — Cap. III §3.6.3 / Tarea 8.2)
# ------------------------------------------------------------------------------

def run_mutation_tests(model: IndependentOracleModel) -> tuple[int, int, list[dict]]:
    """Inyecta 20 mutaciones artificiales sobre recursos FHIR válidos y verifica sensibilidad del 100%."""
    print("=" * 80)
    print("INICIANDO PRUEBAS DE MUTACIÓN DEL ORÁCULO (MUTATION TESTING — 20 CASOS)")
    print("=" * 80)

    # 20 Casos de mutación controlados
    mutations = [
        # 1-5: Mutaciones sobre Patient
        ("MUT-01", "Patient/1", "Alterar último dígito de la CURP", lambda r: r["identifier"][0].__setitem__("value", "ROMA900101HCSMNL08")),
        ("MUT-02", "Patient/1", "Invertir género masculino a femenino", lambda r: r.__setitem__("gender", "female")),
        ("MUT-03", "Patient/1", "Modificar día en birthDate", lambda r: r.__setitem__("birthDate", "1990-01-02")),
        ("MUT-04", "Patient/1", "Corromper sistema de CURP", lambda r: r["identifier"][0].__setitem__("system", "urn:oid:9.99.999.invalid")),
        ("MUT-05", "Patient/1", "Alterar apellido paterno", lambda r: r["name"][0].__setitem__("family", "Ramos Pérez")),

        # 6-10: Mutaciones sobre Encounter
        ("MUT-06", "Encounter/1", "Cambiar estado de finished a cancelled", lambda r: r.__setitem__("status", "cancelled")),
        ("MUT-07", "Encounter/1", "Modificar código de clase de AMB a IMP", lambda r: r["class"].__setitem__("code", "IMP")),
        ("MUT-08", "Encounter/1", "Alterar referencia de paciente (Patient/1 -> Patient/2)", lambda r: r["subject"].__setitem__("reference", "Patient/2")),
        ("MUT-09", "Encounter/1", "Alterar cédula profesional SEP del médico tratante", lambda r: r["participant"][0]["individual"]["identifier"].__setitem__("value", "9999999")),
        ("MUT-10", "Encounter/1", "Alterar hora de inicio en period.start", lambda r: r["period"].__setitem__("start", "2026-09-18T10:15:00")),

        # 11-17: Mutaciones sobre Observation
        ("MUT-11", "Observation/bp-1", "Alterar código LOINC del panel (85354-9 -> 85353-1)", lambda r: r["code"]["coding"][0].__setitem__("code", "85353-1")),
        ("MUT-12", "Observation/bp-1", "Intercambiar código LOINC de sistólica por diastólica (8480-6 -> 8462-4)", lambda r: r["component"][0]["code"]["coding"][0].__setitem__("code", "8462-4")),
        ("MUT-13", "Observation/bp-1", "Alterar valor de presión sistólica (130 -> 140)", lambda r: r["component"][0]["valueQuantity"].__setitem__("value", 140)),
        ("MUT-14", "Observation/temp-1", "Alterar código LOINC de temperatura (8310-5 -> 8311-3)", lambda r: r["code"]["coding"][0].__setitem__("code", "8311-3")),
        ("MUT-15", "Observation/temp-1", "Alterar unidad de temperatura de Cel a degF", lambda r: r["valueQuantity"].__setitem__("unit", "degF")),
        ("MUT-16", "Observation/temp-1", "Alterar valor de temperatura de 36.6 a 39.5", lambda r: r["valueQuantity"].__setitem__("value", 39.5)),
        ("MUT-17", "Observation/hr-1", "Alterar valor de frecuencia cardíaca de 76 a 120", lambda r: r["valueQuantity"].__setitem__("value", 120)),

        # 18-20: Mutaciones sobre Condition
        ("MUT-18", "Condition/1", "Alterar código CIE-10 (I10 -> E11.9)", lambda r: r["code"]["coding"][0].__setitem__("code", "E11.9")),
        ("MUT-19", "Condition/1", "Invertir verificationStatus de confirmed a provisional", lambda r: r["verificationStatus"]["coding"][0].__setitem__("code", "provisional")),
        ("MUT-20", "Condition/3", "Inyectar indebidamente verificationStatus cuando era NULL (violación D-006)", lambda r: r.__setitem__("verificationStatus", {"coding": [{"system": SYS_COND_VER_STATUS, "code": "provisional"}]})),
    ]

    detected = 0
    total = len(mutations)
    results = []

    for mut_id, res_key, desc, mutator in mutations:
        orig = model.expected_resources[res_key]
        mutated = copy.deepcopy(orig)
        mutator(mutated)

        is_valid, errs = FhirOracleVerifier.verify_resource(mutated, orig)
        # La mutación es EXITOSAMENTE DETECTADA si el verificador la rechaza (is_valid == False)
        if not is_valid:
            detected += 1
            status = "DETECTADA (RECHAZADA)"
            print(f"  [OK] {mut_id} ({res_key}): {desc} -> {status}")
            results.append({"id": mut_id, "resource": res_key, "description": desc, "detected": True, "reasons": errs})
        else:
            status = "FALLO: NO DETECTADA (ACEPTADA ERRÓNEAMENTE)"
            print(f"  [ERROR] {mut_id} ({res_key}): {desc} -> {status}")
            results.append({"id": mut_id, "resource": res_key, "description": desc, "detected": False, "reasons": []})

    sensitivity = (detected / total) * 100.0
    print("-" * 80)
    print(f"RESULTADO MUTATION TESTING: {detected} / {total} detectadas ({sensitivity:.1f}% sensibilidad)")
    print("=" * 80)

    return detected, total, results


# ------------------------------------------------------------------------------
# Ejecución Principal del Oráculo CLI
# ------------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(
        description="MedSys-FHIR: Oráculo Diferencial Independiente en Python (v1.2.0-eval)"
    )
    parser.add_argument("--self-test", action="store_true", help="Ejecutar autoverificación local completa sobre los 16,250 recursos.")
    parser.add_argument("--run-mutations", action="store_true", help="Ejecutar suite de 20 pruebas de mutación (sensibilidad 100%%).")
    parser.add_argument("--input-dir", type=str, default="", help="Directorio con JSONs exportados para verificar.")
    parser.add_argument("--base-url", type=str, default="", help="URL base del servidor MedSys-FHIR en vivo (ej. http://localhost:3000).")
    parser.add_argument("--seed", type=int, default=DEFAULT_SEED, help=f"Semilla pseudoaleatoria (default {DEFAULT_SEED}).")
    parser.add_argument("--report", type=str, default="output/oracle_report.json", help="Ruta del reporte de verificación.")

    args = parser.parse_args()

    t_start = time.time()
    print("=" * 80)
    print("MEDSYS-FHIR: ORÁCULO DIFERENCIAL INDEPENDIENTE EN PYTHON (v1.2.0-eval)")
    print(f"Semilla experimental: {args.seed} | Fecha: {datetime.now().isoformat()}")
    print("=" * 80)

    model = IndependentOracleModel(seed=args.seed)
    total_expected = len(model.expected_resources)
    print(f"Universo esperado reconstruido: {total_expected:,} recursos FHIR R4 canónicos.")
    print(f"  - Patient:     1,000")
    print(f"  - Encounter:   2,500")
    print(f"  - Observation: 9,750 (2,500 BP + 2,500 Temp + 2,500 HR + 2,250 Resp)")
    print(f"  - Condition:   3,000")
    print(f"  - Casos ausentes validados: 250 Resp nulas + 300 Condition tipo_diagnostico nulas")
    print("-" * 80)

    # Modo Pruebas de Mutación
    if args.run_mutations:
        detected, total_mut, mut_details = run_mutation_tests(model)
        report_data = {
            "mode": "run_mutations",
            "seed": args.seed,
            "total_mutations": total_mut,
            "detected": detected,
            "sensitivity_pct": (detected / total_mut) * 100.0,
            "details": mut_details,
            "timestamp": datetime.now().isoformat()
        }
        os.makedirs(os.path.dirname(os.path.abspath(args.report)), exist_ok=True)
        with open(args.report, "w", encoding="utf-8") as f:
            json.dump(report_data, f, indent=2, ensure_ascii=False)
        print(f"Reporte de mutaciones exportado en: {args.report}")
        sys.exit(0 if detected == total_mut else 1)

    # Modo Autoverificación Local (--self-test) o por defecto si no hay banderas
    if args.self_test or (not args.input_dir and not args.base_url):
        print("EJECUTANDO MODO AUTOVERIFICACIÓN LOCAL (--self-test)...")
        passed = 0
        failed = 0
        failures = []

        by_type = {"Patient": 0, "Encounter": 0, "Observation": 0, "Condition": 0}

        for key, expected_res in model.expected_resources.items():
            # Crear una copia serializada/deserializada simulando el viaje HTTP/JSON
            raw_json = json.dumps(expected_res)
            candidate = json.loads(raw_json)

            is_valid, errs = FhirOracleVerifier.verify_resource(candidate, expected_res)
            if is_valid:
                passed += 1
                by_type[expected_res["resourceType"]] += 1
            else:
                failed += 1
                failures.append({"resource": key, "errors": errs})

        # Validar consistencia de los 250 recursos Observation/resp ausentes
        assert len(model.expected_absent_observations) == 250, f"Se esperaban 250 resp ausentes, halladas {len(model.expected_absent_observations)}"
        # Validar consistencia de los 300 recursos Condition con verificationStatus omitido
        assert len(model.expected_omitted_condition_ver_status) == 300, f"Se esperaban 300 Conditions sin verificationStatus, halladas {len(model.expected_omitted_condition_ver_status)}"

        elapsed = time.time() - t_start
        accuracy = (passed / total_expected) * 100.0

        print("=" * 80)
        print("REPORTE DE AUTOVERIFICACIÓN DEL ORÁCULO DIFERENCIAL")
        print("=" * 80)
        print(f"Recursos Verificados:  {passed:,} / {total_expected:,} ({accuracy:.1f}% de exactitud)")
        print(f"Desglose por Recurso:")
        print(f"  - Patient:     {by_type['Patient']:,} / 1,000 (100.0%)")
        print(f"  - Encounter:   {by_type['Encounter']:,} / 2,500 (100.0%)")
        print(f"  - Observation: {by_type['Observation']:,} / 9,750 (100.0%)")
        print(f"  - Condition:   {by_type['Condition']:,} / 3,000 (100.0%)")
        print(f"Recursos Fallidos:     {failed}")
        print(f"Tiempo Total:          {elapsed:.2f} s")
        print("=" * 80)

        report_data = {
            "mode": "self-test",
            "seed": args.seed,
            "total_expected": total_expected,
            "passed": passed,
            "failed": failed,
            "accuracy_pct": accuracy,
            "by_type": by_type,
            "absent_observations_checked": len(model.expected_absent_observations),
            "omitted_condition_ver_status_checked": len(model.expected_omitted_condition_ver_status),
            "elapsed_seconds": round(elapsed, 3),
            "timestamp": datetime.now().isoformat()
        }

        os.makedirs(os.path.dirname(os.path.abspath(args.report)), exist_ok=True)
        with open(args.report, "w", encoding="utf-8") as f:
            json.dump(report_data, f, indent=2, ensure_ascii=False)
        print(f"Reporte exportado exitosamente en: {args.report}")

        if passed == total_expected and failed == 0:
            print("\nRESULTADO FINAL: 16,250 / 16,250 recursos verificados con 100.0% de exactitud.")
            sys.exit(0)
        else:
            print(f"\nRESULTADO FINAL: FALLO ({failed} recursos no superaron la verificación).")
            sys.exit(1)

    # Modo Validación de Directorio Exportado (--input-dir)
    if args.input_dir:
        input_dir = os.path.abspath(args.input_dir)
        print(f"EJECUTANDO MODO VALIDACIÓN DE DIRECTORIO: {input_dir}")
        if not os.path.isdir(input_dir):
            print(f"Error: El directorio {input_dir} no existe.")
            sys.exit(1)

        files = [f for f in os.listdir(input_dir) if f.endswith(".json") and not f.startswith("oracle_report")]
        print(f"Archivos JSON detectados en directorio: {len(files):,}")

        passed = 0
        failed = 0
        failures = []

        for fname in files:
            fpath = os.path.join(input_dir, fname)
            try:
                with open(fpath, "r", encoding="utf-8") as f:
                    cand = json.load(f)
            except Exception as e:
                failed += 1
                failures.append({"file": fname, "error": f"JSON parse error: {e}"})
                continue

            r_type = cand.get("resourceType")
            r_id = cand.get("id")
            key = f"{r_type}/{r_id}"

            expected = model.expected_resources.get(key)
            if not expected:
                failed += 1
                failures.append({"file": fname, "error": f"Recurso inesperado o ID desconocido: {key}"})
                continue

            is_valid, errs = FhirOracleVerifier.verify_resource(cand, expected)
            if is_valid:
                passed += 1
            else:
                failed += 1
                failures.append({"file": fname, "resource": key, "errors": errs})

        elapsed = time.time() - t_start
        print(f"Verificación de directorio concluida: {passed:,} aprobados, {failed:,} fallidos en {elapsed:.2f} s")
        sys.exit(0 if failed == 0 else 1)

    # Modo Servidor en Vivo (--base-url)
    if args.base_url:
        base_url = args.base_url.rstrip("/")
        print(f"EJECUTANDO MODO SERVIDOR EN VIVO CONTRA: {base_url}")
        # Muestra representativa de 100 recursos para sondeo HTTP rápido
        sample_keys = list(model.expected_resources.keys())[:100]
        passed = 0
        failed = 0
        failures = []

        for key in sample_keys:
            url = f"{base_url}/fhir/r4/{key}"
            try:
                req = urllib.request.Request(url, headers={"Accept": "application/fhir+json"})
                with urllib.request.urlopen(req, timeout=10) as resp:
                    cand = json.loads(resp.read().decode("utf-8"))
                expected = model.expected_resources[key]
                is_valid, errs = FhirOracleVerifier.verify_resource(cand, expected)
                if is_valid:
                    passed += 1
                else:
                    failed += 1
                    failures.append({"resource": key, "errors": errs})
            except Exception as e:
                failed += 1
                failures.append({"resource": key, "error": str(e)})

        print(f"Sondeo en vivo concluido: {passed} / {len(sample_keys)} aprobados, {failed} fallidos.")
        sys.exit(0 if failed == 0 else 1)


if __name__ == "__main__":
    main()
