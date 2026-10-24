"""Offline provenance/hash and Git-visibility audit. --write establishes reviewed fixture hashes."""
from pathlib import Path
import hashlib,json,subprocess,sys
root=Path(__file__).resolve().parents[1]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
if '--write' in sys.argv:
 files=[]
 for base in ['fixtures/beta','validation/oracle/src','validation/oracle/test','validation/oracle/libs','licenses']:
  for p in sorted((root/base).rglob('*')):
   if p.is_file():files.append(p)
 for p in ['validation/oracle/package.json','validation/oracle/package-lock.json','validation/oracle/tsconfig.json','validation/oracle/raycast-env.d.ts','assets/change-case.png','src/unicode.rs','data/parity.json']:files.append(root/p)
 records=[]
 for p in files:
  path=p.relative_to(root).as_posix()
  if path.startswith('fixtures/beta/'):
   origin={'repository':'Tezbar','commit':'fd300e276356587726566026f072563eb9c34cb1','path':path.removeprefix('fixtures/beta/')}
  elif path.startswith('validation/oracle/src/') or path.startswith('validation/oracle/test/') or path in ['validation/oracle/'+s for s in ['package.json','package-lock.json','tsconfig.json','raycast-env.d.ts']]:
   origin={'repository':'https://github.com/almatkai/change-case','commit':'b33e675ea3945e08f87dc5ffc8428f5b7dc9d12c','path':path.removeprefix('validation/oracle/')}
  elif path.startswith('validation/oracle/libs/'):
   parts=path.split('/');library=parts[3];version=json.loads((root/'validation/oracle/libs'/library/'package.json').read_text())['version'];origin={'npm':library,'version':version,'path':'/'.join(parts[4:])}
  elif path=='assets/change-case.png':origin={'repository':'https://github.com/raycast/extensions','commit':'6734d85992290066fd31a7f5bee3cca3d3545a1a','path':'extensions/change-case/assets/command-icon.png'}
  elif path=='src/unicode.rs':origin={'generator':'validation/generate-unicode.mjs then rustfmt','node':'26.7.0','unicode':'17.0','locale':'en-US'}
  elif path=='data/parity.json':origin={'generator':'validation/oracle/generate.mjs','source':'actual own TS and exact locked JS libraries','node':'26.7.0','locale':'en-US','vectors':52668}
  else:origin={'license':'See NOTICES.md and npm release gitHeads in validation/oracle/libs/*/registry.json'}
  records.append({'path':path,'bytes':p.stat().st_size,'sha256':sha(p),'origin':origin})
 (root/'provenance.json').write_text(json.dumps({'schemaVersion':1,'aggregateSPDX':'MIT AND Apache-2.0 AND Unicode-3.0','files':records},indent=2)+'\n')
records=json.loads((root/'provenance.json').read_text())['files']
for record in records:
 p=root/record['path'];assert p.is_file(),record['path'];assert sha(p)==record['sha256'],record['path'];assert p.stat().st_size==record['bytes']
# A packaged extract has no .git; Git visibility is checked only in the actual clone.
if (root.parent/'.git').exists():
 for record in records:
  p=root/record['path'];result=subprocess.run(['git','check-ignore','--no-index',str(p)],cwd=root,capture_output=True,text=True);assert result.returncode==1,(record['path'],result.stdout,result.stderr)
icon=root/'assets/change-case.png';assert icon.stat().st_size==218405;assert sha(icon)=='9c2ae5a995722826df828565dc991aff954d30b376fc3fce6014372c4528ff6b'
print('Provenance hashes/paths:',len(records),'PASS; exact icon and Git visibility: PASS')
