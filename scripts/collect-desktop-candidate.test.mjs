import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { createHash } from 'node:crypto';
import { collectCandidate, targets } from './collect-desktop-candidate.mjs';
function fixture(t, platform) {
  const root=mkdtempSync(join(tmpdir(),'fabcam-candidate-'));
  t.after(()=>rmSync(root,{recursive:true,force:true}));
  const spec=targets[platform];
  const bundleDir=join(root,'bundle'), outputDir=join(root,'out');
  mkdirSync(join(bundleDir,spec.folder),{recursive:true});
  return {platform,target:spec.target,version:'0.9.0-beta.1',bundleDir,outputDir,provenance:{commit:'fixture'}};
}
for (const platform of Object.keys(targets)) test(`${platform}: exact artifact bytes, checksum and unqualified status`, t=>{
  const args=fixture(t,platform),spec=targets[platform];
  const name=`FabCAM_${args.version}_fixture${spec.suffix}`,bytes=Buffer.from('installer test bytes');
  writeFileSync(join(args.bundleDir,spec.folder,name),bytes);
  const result=collectCandidate(args);
  assert.deepEqual(readFileSync(join(args.outputDir,name)),bytes);
  assert.equal(result.artifact.sha256,createHash('sha256').update(bytes).digest('hex'));
  assert.equal(result.installation_qualified,false);
  assert.equal(result.runtime_qualified,false);
  const manifestHash=createHash('sha256').update(readFileSync(join(args.outputDir,'candidate.json'))).digest('hex');
  assert.equal(readFileSync(join(args.outputDir,'SHA256SUMS.txt'),'utf8'), `${result.artifact.sha256}  ${name}\n${manifestHash}  candidate.json\n`);
  if(platform==='macos') assert.equal(result.apple_notarized,false);
  assert.throws(()=>collectCandidate(args),/destination must be empty/);
});
test('reject missing, wrong version, wrong target and ambiguous stale installers',t=>{
  const args=fixture(t,'macos'),dir=join(args.bundleDir,'dmg');
  assert.throws(()=>collectCandidate(args),/exactly one/);
  writeFileSync(join(dir,'FabCAM_0.5.0_arm64.dmg'),'old');
  assert.throws(()=>collectCandidate(args),/exactly one/);
  writeFileSync(join(dir,'FabCAM_0.9.0-beta.1_arm64.dmg'),'new');
  assert.throws(()=>collectCandidate(args),/exactly one/);
  assert.throws(()=>collectCandidate({...args,target:'x86_64-apple-darwin'}),/platform\/target/);
});
