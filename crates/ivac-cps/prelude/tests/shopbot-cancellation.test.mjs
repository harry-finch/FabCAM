import assert from 'node:assert/strict';
import test from 'node:test';
import { readFileSync } from 'node:fs';
import { createRuntime } from './harness.mjs';
const post = readFileSync(new URL('../../posts/shopbot.cps', import.meta.url), 'utf8');
function setup() {
  const rt = createRuntime();
  rt.eval(post);
  rt.eval(`
    var cancelled = false, visited = 0, lastVisited = -1, armed = false, trigger = 65;
    var originalPreflight = preflight;
    preflight = function() { armed = true; return originalPreflight(); };
    __ivac.abortCheck = function() { return cancelled; };
    var recs = Array.from({length: 200}, function(_, i) {
      var r = {kind:'linear', x:i/10, y:0, z:-1, feed:600, movement:2};
      return r;
    });
    var section = {id:1,strategy:'contour2d',tool:{number:1,diameter:6,toolType:5,coolant:0},
      spindleRpm:18000,spindleClockwise:true,workOffset:1,workPlane:[[1,0,0],[0,1,0],[0,0,1]],
      initialPosition:{x:0,y:0,z:5},finalPosition:{x:20,y:0,z:5},records:recs,parameters:[]};
    var program = {version:1, header:{programName:'cancel',programComment:'',toleranceMm:0.01,
      parameters:[{name:'ivacam:machine-config',value:'{"mode":"mill","tool_change":"manual_m0_pause"}'},
        {name:'ivacam:clearance-z',value:5}]}, sections:[section]};
    function watch(array) {
      for (var i = 0; i < array.length; i++) (function(index, value) {
        Object.defineProperty(array, index, {get:function() {
          if (armed) { visited++; lastVisited = index; if (index == trigger) cancelled = true; } return value;
        }});
      })(i, array[i]);
    }
  `);
  return rt;
}
for (const mode of ['records','drill-points','helix-scan','parameters']) {
  test(`ShopBot ${mode} observes cancellation within 32 subsequent work items`, () => {
    const rt = setup();
    if (mode === 'records') rt.eval('watch(recs);');
    if (mode === 'helix-scan') rt.eval(`section.parameters=[{name:'ivacam:entry-strategy',value:'helix'}]; watch(recs);`);
    if (mode === 'parameters') rt.eval(`section.parameters=Array.from({length:200},function(){return {name:'notes',value:'test'};});watch(section.parameters);`);
    if (mode === 'drill-points') rt.eval(`
      var points=Array.from({length:200},function(_,i){return {x:i,y:0,z:-1};});
      section.records=[{kind:'cycle',cycleType:'drilling',params:{feedrate:600,clearance:5,retract:2},points:points}];
      watch(points);
    `);
    assert.throws(() => rt.eval('__ivacExecute(program, {});'), /__IVAC_CANCELLED__/);
    assert.equal(rt.output(), '');
    // parameters are read twice per item and drill points twice; bound index,
    // independently of how often a validator reads the same item.
    assert.ok(rt.eval('cancelled'));
    assert.ok(rt.eval('lastVisited') >= 65 && rt.eval('lastVisited') < 97, `${mode}: ${rt.eval('lastVisited')}`);
  });
}
test('last-record refusal still occurs before any ShopBot output', () => {
  const rt=setup();
  rt.eval(`recs[199]={kind:'passThrough',text:'SO,1,1'};`);
  assert.throws(() => rt.eval('__ivacExecute(program, {});'), /unsupported record passThrough/);
  assert.equal(rt.output(), '');
});
