"""Validate the complete pinned schema plus explicit semantic coverage and negative mutations."""
import copy,json,pathlib
from jsonschema import Draft202012Validator
root=pathlib.Path(__file__).resolve().parents[1]
schema=json.loads((root/'fixtures/beta/schemas/extension-v2.schema.json').read_text());Draft202012Validator.check_schema(schema)
v=Draft202012Validator(schema);manifest=json.loads((root/'manifest.json').read_text());v.validate(manifest)
commands=json.loads((root/'validation/oracle/package.json').read_text())['commands'];assert [c['name'] for c in manifest['commands']]==[c['name'] for c in commands];assert len(commands)==22
assert manifest['id']=='com.tezbar.native.change-case' and manifest['replaces']==['raycast.change-case']
assert all(c['runtime']=='rust' and c['platforms']==['macOS','Windows'] for c in manifest['commands'])
assert {(x['platform'],x['state']) for x in manifest['verification']}=={('macOS','not-run'),('Windows','not-run')}
negative=[]
for key in schema['required']:
 d=copy.deepcopy(manifest);del d[key];negative.append(d)
for key,value in [('schemaVersion',1),('id','raycast.change-case'),('slug','../escape'),('name',''),('version',''),('icon','/tmp/icon.png'),('runtime','javascript'),('replaces',[]),('replaces',['raycast.x','raycast.x']),('platforms',['Solaris']),('platforms',['macOS','macOS']),('preferences','bad'),('commands',[]),('verification',[]),('author','unsupported')]:
 d=copy.deepcopy(manifest);d[key]=value;negative.append(d)
for key,value in [('name',''),('platforms',[]),('platforms',['Solaris']),('platforms',['Windows','Windows']),('runtime','js')]:
 d=copy.deepcopy(manifest);d['commands'][0][key]=value;negative.append(d)
for key in ['name','platforms','runtime']:
 d=copy.deepcopy(manifest);del d['commands'][0][key];negative.append(d)
for key,value in [('platform','Solaris'),('state','verified')]:
 d=copy.deepcopy(manifest);d['verification'][0][key]=value;negative.append(d)
for key in ['platform','state']:
 d=copy.deepcopy(manifest);del d['verification'][0][key];negative.append(d)
for d in negative:assert list(v.iter_errors(d)),d
print('Complete pinned schema: PASS;',len(negative),'negative cases; 22-command/platform/replacement/verification coverage: PASS')
