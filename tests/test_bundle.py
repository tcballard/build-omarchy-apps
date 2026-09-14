from __future__ import annotations
import hashlib, importlib.util, json, os, shutil, subprocess, sys, tempfile, unittest, zipfile
from pathlib import Path
REPO=Path(__file__).resolve().parent.parent
sys.path.insert(0,str(REPO/'scripts'))
import package_submission as package
import sync_openai_adapter as sync
import validate_agent_plugin as portable
import install_agent_skills as installer

def run(*args,cwd=REPO):
 return subprocess.run(args,cwd=cwd,text=True,capture_output=True)

class BundleTests(unittest.TestCase):
 def test_versions_and_provider_separation(self):
  self.assertEqual([],portable.validate_tree(REPO))
  version=(REPO/'VERSION').read_text().strip()
  for path in ['plugin.json','.claude-plugin/plugin.json','plugins/build-omarchy-apps/.codex-plugin/plugin.json']:
   self.assertEqual(version,json.loads((REPO/path).read_text())['version'])
  self.assertEqual([],sync.compare(REPO/'skills',REPO/'plugins/build-omarchy-apps/skills'))

 def test_each_host_project_install_is_idempotent(self):
  locations={'agents':'.agents/skills','codex':'.agents/skills','cursor':'.cursor/skills','gemini':'.gemini/skills','claude':'.claude/skills','opencode':'.opencode/skills'}
  for host,path in locations.items():
   with self.subTest(host=host),tempfile.TemporaryDirectory() as temp:
    command=[sys.executable,str(REPO/'scripts/install_agent_skills.py'),'--target',host,'--scope','project','--json']
    first=run(*command,cwd=Path(temp));self.assertEqual(0,first.returncode,first.stdout+first.stderr)
    dest=Path(temp)/path
    self.assertEqual(10,len(list(dest.glob('*/SKILL.md'))));self.assertFalse(list(dest.glob('*/agents/openai.yaml')))
    second=run(*command,cwd=Path(temp));self.assertEqual(10,len(json.loads(second.stdout)['unchanged']))

 def test_adapter_detects_and_repairs_drift_without_losing_metadata(self):
  with tempfile.TemporaryDirectory() as temp:
   dest=Path(temp)/'skills';shutil.copytree(REPO/'plugins/build-omarchy-apps/skills',dest)
   changed=dest/'omarchy-app-design/SKILL.md';changed.write_text('different')
   metadata=dest/'omarchy-app-design/agents/openai.yaml';before=metadata.read_bytes()
   self.assertTrue(sync.compare(REPO/'skills',dest));sync.write_adapter(REPO/'skills',dest)
   self.assertEqual([],sync.compare(REPO/'skills',dest));self.assertEqual(before,metadata.read_bytes())

 def test_build_outputs_are_excluded_from_install_and_sync(self):
  with tempfile.TemporaryDirectory() as temp:
   root=Path(temp);(root/'target').mkdir();(root/'target/debug-app').write_bytes(b'build output')
   (root/'SKILL.md').write_text('source')
   self.assertEqual(['SKILL.md'],[str(p) for p in installer.snapshot_tree(root)])
   self.assertEqual(['SKILL.md'],[str(p) for p in sync.portable_files(root)])

 def test_provider_metadata_is_rejected_in_portable_tree(self):
  with tempfile.TemporaryDirectory() as temp:
   root=Path(temp);shutil.copytree(REPO/'skills',root/'skills')
   meta=root/'skills/omarchy-app-design/agents';meta.mkdir();(meta/'openai.yaml').write_text('interface: {}\n')
   self.assertTrue(portable.validate_tree(root))

class PackageTests(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.temp=tempfile.TemporaryDirectory();cls.root=Path(cls.temp.name)/'source'
  shutil.copytree(REPO,cls.root,ignore=shutil.ignore_patterns('.git','target','dist','__pycache__'))
  for args in [('init','-b','main'),('config','user.name','Fixture'),('config','user.email','fixture@example.invalid'),('add','.'),('commit','-m','fixture')]:
   p=run('git',*args,cwd=cls.root);assert p.returncode==0,p.stderr
 @classmethod
 def tearDownClass(cls):cls.temp.cleanup()
 def test_committed_packages_are_deterministic_and_installable(self):
  a=Path(self.temp.name)/'a';b=Path(self.temp.name)/'b'
  one=package.build(self.root,a,'HEAD',True);two=package.build(self.root,b,'HEAD',True)
  self.assertEqual(one['files'],two['files'])
  for name in one['files']:self.assertEqual((a/name).read_bytes(),(b/name).read_bytes(),name)
  manifest=json.loads((a/'SOURCE-MANIFEST.json').read_text())
  self.assertTrue(manifest['source']['commit'])
  for artifact in one['artifacts']:
   with zipfile.ZipFile(a/artifact['name']) as z:
    names=z.namelist();self.assertFalse(any('/.git/' in n or '/target/' in n for n in names))
    if artifact['kind'] in ['agent-plugin','claude-plugin']:
     folder=Path(self.temp.name)/artifact['kind'];z.extractall(folder)
     app=folder/'build-omarchy-apps';dest=folder/'installed'
     result=run(sys.executable,str(app/'scripts/install_agent_skills.py'),'--target','generic','--destination',str(dest),'--json')
     self.assertEqual(0,result.returncode,result.stdout+result.stderr)
     self.assertEqual(10,len(list(dest.glob('*/SKILL.md'))))
    if artifact['kind']=='agent-plugin':self.assertFalse(any('/agents/openai.yaml' in n for n in names))
    if artifact['kind']=='openai-plugin':self.assertEqual(10,sum(n.endswith('/agents/openai.yaml') for n in names))
 def test_dirty_tree_refused_and_clean_input_remains_exact(self):
  extra=self.root/'ambient.txt';extra.write_text('not a committed input')
  try:
   with self.assertRaises(package.PackageError):package.build(self.root,Path(self.temp.name)/'dirty','HEAD',True)
   _,_,blobs=package.load_tree(self.root,'HEAD');self.assertNotIn(package.PurePosixPath('ambient.txt'),blobs)
  finally:extra.unlink()
 def test_archive_rejects_unsafe_paths_and_case_collisions(self):
  with self.assertRaises(package.PackageError):package._safe_relative('../outside')
  a=package.Blob(package.PurePosixPath('a'),'100644','unused',b'a')
  with self.assertRaises(package.PackageError):package._archive_bytes([(a,package.PurePosixPath('a')),(a,package.PurePosixPath('A'))])
if __name__=='__main__':unittest.main()
