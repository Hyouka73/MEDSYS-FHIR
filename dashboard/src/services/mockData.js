// Datos sintéticos representativos de la NOM-004-SSA3-2012 y sus recursos FHIR correspondientes
export const MOCK_PATIENTS_LEGACY = [
  {
    id_paciente: 1,
    curp: "ROMA900101HCSNN01",
    primer_nombre: "Alberto",
    segundo_nombre: "Manuel",
    apellido_paterno: "Ramos",
    apellido_materno: "Gómez",
    fecha_nacimiento: "1990-01-01",
    sexo_biologico: "M",
    telefono_contacto: "9611234567",
    fecha_registro: "2026-09-18 09:00:00"
  },
  {
    id_paciente: 2,
    curp: "LOPE950512MCSNN02",
    primer_nombre: "Sofía",
    segundo_nombre: null,
    apellido_paterno: "López",
    apellido_materno: "Hernández",
    fecha_nacimiento: "1995-05-12",
    sexo_biologico: "F",
    telefono_contacto: "9617654321",
    fecha_registro: "2026-09-18 09:30:00"
  }
];

export const MOCK_CONSULTAS_LEGACY = [
  {
    id_consulta: 1,
    id_paciente: 1,
    cedula_medico_tratante: "8472910",
    nombre_medico: "Dra. María Elena Cruz Martínez",
    estado_consulta: "FINALIZADA",
    motivo_consulta: "Control trimestral de hipertensión arterial",
    fecha_hora_inicio: "2026-09-18 09:15:00",
    fecha_hora_fin: "2026-09-18 09:40:00",
    unidad_medica: "CESSA Tuxtla Poniente"
  },
  {
    id_consulta: 2,
    id_paciente: 2,
    cedula_medico_tratante: "7291043",
    nombre_medico: "Dr. Jorge Antonio Ruiz Morales",
    estado_consulta: "FINALIZADA",
    motivo_consulta: "Cefalea intensa y dolor retroocular",
    fecha_hora_inicio: "2026-09-18 10:00:00",
    fecha_hora_fin: "2026-09-18 10:25:00",
    unidad_medica: "Centro de Salud Urbano Tuxtla"
  }
];

export const MOCK_SIGNOS_LEGACY = [
  {
    id_signo: 1,
    id_consulta: 1,
    id_paciente: 1,
    presion_sistolica: 130,
    presion_diastolica: 85,
    frecuencia_cardiaca: 76,
    frecuencia_respiratoria: 18,
    temperatura_celsius: "36.60",
    peso_kg: "78.50",
    talla_cm: 172,
    fecha_registro: "2026-09-18 09:18:00"
  },
  {
    id_signo: 2,
    id_consulta: 2,
    id_paciente: 2,
    presion_sistolica: 110,
    presion_diastolica: 70,
    frecuencia_cardiaca: 82,
    frecuencia_respiratoria: 16,
    temperatura_celsius: "38.20",
    peso_kg: "58.00",
    talla_cm: 160,
    fecha_registro: "2026-09-18 10:05:00"
  }
];

export const MOCK_DIAGNOSTICOS_LEGACY = [
  {
    id_diagnostico: 1,
    id_consulta: 1,
    id_paciente: 1,
    codigo_cie10: "I10",
    descripcion_diagnostico: "Hipertensión esencial (primaria)",
    tipo_diagnostico: "CONFIRMADO",
    fecha_diagnostico: "2026-09-18"
  },
  {
    id_diagnostico: 2,
    id_consulta: 2,
    id_paciente: 2,
    codigo_cie10: "G43.9",
    descripcion_diagnostico: "Migraña, no especificada",
    tipo_diagnostico: "PRESUNTIVO",
    fecha_diagnostico: "2026-09-18"
  }
];

export const MOCK_FULL_COMPARISONS = {
  1: {
    paciente_legado: MOCK_PATIENTS_LEGACY[0],
    consultas_legadas: [MOCK_CONSULTAS_LEGACY[0]],
    signos_vitales_legados: [MOCK_SIGNOS_LEGACY[0]],
    diagnosticos_legados: [MOCK_DIAGNOSTICOS_LEGACY[0]],
    fhir_patient: {
      resourceType: "Patient",
      id: "1",
      identifier: [
        {
          use: "official",
          system: "urn:oid:2.16.840.1.113883.4.629",
          value: "ROMA900101HCSNN01"
        }
      ],
      name: [
        {
          use: "official",
          family: "Ramos Gómez",
          given: ["Alberto", "Manuel"]
        }
      ],
      telecom: [
        {
          system: "phone",
          value: "9611234567",
          use: "mobile"
        }
      ],
      gender: "male",
      birthDate: "1990-01-01"
    },
    fhir_encounters: [
      {
        resourceType: "Encounter",
        id: "1",
        status: "finished",
        class: {
          system: "http://terminology.hl7.org/CodeSystem/v3-ActCode",
          code: "AMB",
          display: "ambulatory"
        },
        subject: {
          reference: "Patient/1"
        },
        participant: [
          {
            individual: {
              identifier: {
                system: "http://cedulaprofesional.sep.gob.mx",
                value: "8472910"
              },
              display: "Dra. María Elena Cruz Martínez"
            }
          }
        ],
        period: {
          start: "2026-09-18T09:15:00",
          end: "2026-09-18T09:40:00"
        }
      }
    ],
    fhir_observations: [
      {
        resourceType: "Observation",
        id: "bp-1",
        status: "final",
        category: [
          {
            coding: [
              {
                system: "http://terminology.hl7.org/CodeSystem/observation-category",
                code: "vital-signs",
                display: "Vital Signs"
              }
            ]
          }
        ],
        code: {
          coding: [
            {
              system: "http://loinc.org",
              code: "85354-9",
              display: "Blood pressure panel with all children optional"
            }
          ],
          text: "Panel de Presión Arterial"
        },
        subject: {
          reference: "Patient/1"
        },
        encounter: {
          reference: "Encounter/1"
        },
        component: [
          {
            code: {
              coding: [
                {
                  system: "http://loinc.org",
                  code: "8480-6",
                  display: "Systolic blood pressure"
                }
              ],
              text: "Presión Sistólica"
            },
            valueQuantity: {
              value: 130,
              unit: "mmHg",
              system: "http://unitsofmeasure.org",
              code: "mm[Hg]"
            }
          },
          {
            code: {
              coding: [
                {
                  system: "http://loinc.org",
                  code: "8462-4",
                  display: "Diastolic blood pressure"
                }
              ],
              text: "Presión Diastólica"
            },
            valueQuantity: {
              value: 85,
              unit: "mmHg",
              system: "http://unitsofmeasure.org",
              code: "mm[Hg]"
            }
          }
        ]
      },
      {
        resourceType: "Observation",
        id: "temp-1",
        status: "final",
        category: [
          {
            coding: [
              {
                system: "http://terminology.hl7.org/CodeSystem/observation-category",
                code: "vital-signs",
                display: "Vital Signs"
              }
            ]
          }
        ],
        code: {
          coding: [
            {
              system: "http://loinc.org",
              code: "8310-5",
              display: "Body temperature"
            }
          ],
          text: "Temperatura Corporal"
        },
        subject: {
          reference: "Patient/1"
        },
        encounter: {
          reference: "Encounter/1"
        },
        valueQuantity: {
          value: 36.6,
          unit: "Cel",
          system: "http://unitsofmeasure.org",
          code: "Cel"
        }
      }
    ],
    fhir_conditions: [
      {
        resourceType: "Condition",
        id: "cond-1",
        clinicalStatus: {
          coding: [
            {
              system: "http://terminology.hl7.org/CodeSystem/condition-clinical",
              code: "active",
              display: "Active"
            }
          ]
        },
        verificationStatus: {
          coding: [
            {
              system: "http://terminology.hl7.org/CodeSystem/condition-ver-status",
              code: "confirmed",
              display: "Confirmed"
            }
          ]
        },
        code: {
          coding: [
            {
              system: "http://hl7.org/fhir/sid/icd-10",
              code: "I10",
              display: "Hipertensión esencial (primaria)"
            }
          ],
          text: "Hipertensión esencial (primaria)"
        },
        subject: {
          reference: "Patient/1"
        },
        encounter: {
          reference: "Encounter/1"
        },
        recordedDate: "2026-09-18"
      }
    ]
  }
};
