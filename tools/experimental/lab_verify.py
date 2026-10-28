"""Verify the dedicated PostgreSQL cluster through Windows loopback."""
import json
from pathlib import Path
import psycopg
from evidence_runners import stamp, write

tables = {
    's0':['tbl_pacientes','tbl_consultas','tbl_signos_vitales','tbl_diagnosticos'],
    's1':['tbl_sabana_consulta_externa','tbl_sabana_diagnosticos'],
    's2':['tbl_pacientes_s2','tbl_consultas_s2','tbl_signos_s2','tbl_diagnosticos_s2'],
    's3':['tbl_pacientes_s3','tbl_consultas_s3','tbl_signos_s3','tbl_diagnosticos_s3'],
}
result={'kind':'isolated_postgresql_lab_verification','checked_utc':stamp(),'host':'127.0.0.1','port':55432,'databases':{}}
for schema,names in tables.items():
    with psycopg.connect(host='127.0.0.1',port=55432,user='postgres',dbname='medsys_eval_'+schema,connect_timeout=10) as connection:
        marker=connection.execute('SELECT run_id FROM experimental_lab_guard').fetchone()[0]
        directory=connection.execute('SHOW data_directory').fetchone()[0]
        assert marker=='reanudacion-2933964'
        assert directory=='/var/lib/postgresql/medsys-eval-2933964'
        counts={name:connection.execute('SELECT count(*) FROM '+name).fetchone()[0] for name in names}
        connection.execute('SET ROLE medsys_ro')
        grants={name:connection.execute('SELECT has_table_privilege(current_user,%s,\'SELECT\'),has_table_privilege(current_user,%s,\'INSERT,UPDATE,DELETE\')',(name,name)).fetchone() for name in names}
        assert all(read and not write for read,write in grants.values())
        result['databases'][schema]={'marker':marker,'data_directory':directory,'counts':counts,'medsys_ro_grants_select_any_dml':grants}
write(Path(__file__).resolve().parents[2]/'auditoria/evidencias/lab-development/WINDOWS_VERIFICATION_25OCT.json',result)
print(json.dumps(result))
