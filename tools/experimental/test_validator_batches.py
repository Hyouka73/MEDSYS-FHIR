"""Regression for Windows command-length overflow in actual validator invocations."""
import subprocess
from pathlib import Path
import tempfile
import unittest
from evidence_runners import digest, stage_validation_inputs, validator_batches, windows_command_length


class WindowsBatchRegression(unittest.TestCase):
    def test_large_census_is_partitioned_without_losing_or_reordering_paths(self):
        inputs = [{'path':'D:\\Synthetic lab with spaces\\'+'long-directory-'*12+str(i)+'.json'} for i in range(500)]
        prefix = ['java', '-Xmx768m', '-jar', 'D:\\Synthetic lab\\validator_cli.jar']
        suffix = ['-output','D:\\Synthetic lab\\outcomes.json']
        self.assertGreater(len(subprocess.list2cmdline(prefix+[r['path'] for r in inputs]+suffix)),32767)
        batches = list(validator_batches(inputs,500,prefix,suffix))
        self.assertGreater(len(batches),1)
        self.assertEqual([row for batch in batches for row in batch],inputs)
        self.assertTrue(all(len(subprocess.list2cmdline(prefix+[r['path'] for r in batch]+suffix))<=24000 for batch in batches))

    def test_single_unusable_path_is_explicit_failure(self):
        with self.assertRaisesRegex(ValueError,'single validator input path'):
            list(validator_batches([{'path':'x'*25000}],500,['java'],[]))

    def test_windows_limit_counts_utf16_units_for_non_bmp_paths(self):
        inputs=[{'path':'D:\\'+chr(0x1f600)*100+str(i)+'.json'} for i in range(150)]
        batches=list(validator_batches(inputs,500,['java'],[]))
        self.assertGreater(len(batches),1)
        self.assertTrue(all(windows_command_length(['java']+[i['path'] for i in batch])<=24000 for batch in batches))

    def test_staging_retains_original_bytes_and_identity_mapping(self):
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp)/'original.json'
            source.write_bytes(b'{"resourceType":"Patient","id":"a","value":1.2300}\n')
            row = {'path':str(source),'resource':'Patient/a','sha256':digest(source)}
            mapping = stage_validation_inputs([row],Path(temp)/'staged')
            staged, recorded = next(iter(mapping.items()))
            self.assertEqual(Path(staged).read_bytes(),source.read_bytes())
            self.assertEqual(recorded['path'],str(source))
            self.assertEqual(recorded['sha256'],row['sha256'])

    def test_staging_rejects_changed_source_instead_of_validating_new_bytes(self):
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp)/'original.json'
            source.write_bytes(b'{"resourceType":"Patient"}')
            with self.assertRaisesRegex(ValueError,'differs'):
                stage_validation_inputs([{'path':str(source),'sha256':'0'*64}],Path(temp)/'staged')


if __name__=='__main__':
    unittest.main()
