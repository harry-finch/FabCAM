/**
 * ivaCAM ShopBot MVP. Authored here, GPL-3.0-or-later.
 * Command sources: docs/shopbot-modernization/shopbot/OPEN_SBP_COMMANDS.md.
 * Metric XYZ milling; manual spindle and tool changes; XY arcs linearized
 * at 0.01 mm, including qualified Ramp Z interpolation. Helix and combined
 * Ramp/tabs/leads remain explicitly rejected. No relay/VFD/probe/ATC assumptions before machine qualification.
 */
description = "ShopBot OpenSBP MVP (manual spindle, metric)";
vendor = "ivaCAM";
legal = "GPL-3.0-or-later";
extension = "sbp";
capabilities = CAPABILITY_MILLING;
unit = MM;
tolerance = 0.01;
allowedCircularPlanes = 0; // explicitly linearize; never emit unqualified CG
allowHelicalMoves = false;
var xyz = createFormat({decimals:4, forceDecimal:true, minDigitsRight:4});
var speed = createFormat({decimals:6, forceDecimal:true, minDigitsRight:6});
var secondsFormat = createFormat({decimals:3, forceDecimal:true, minDigitsRight:3});
// Opt-in for the operator-supplied Sorbonne MTC wiring, not a generic ShopBot default.
properties = {
  labSpindle1Control: {
    title: "Sorbonne MTC spindle 1 (preloaded tool)",
    description: "Reviewed lab wiring: TR RPM, SO4 enable, SO1 spindle; 2s startup. Prepare tool 1 and XYZ zero before running. No C9, probing or tool changes.",
    type: "boolean", value: false, scope: "post"
  }
};
var labSpindle1;
var safeZ;
var position;
var lastFeed;

function fail(message) { throw new Error("ShopBot MVP: " + message); }
function finite(n) { return typeof n == "number" && isFinite(n); }
function point(p) {
  if (!p || !finite(p.x) || !finite(p.y) || !finite(p.z)) fail("non-finite XYZ");
}
function feed(f) {
  if (!finite(f) || f <= 0 || Number(speed.format(f / 60)) <= 0)
    fail("feed must be positive and representable");
}
function comment(text) {
  // One ASCII comment line, including hostile user strings. Never executable.
  writeln("' " + String(text).replace(/[^ -~]/g, " "));
}
function pause(text) { comment(text); writeln("PAUSE"); }
function dwell(s) {
  if (!finite(s) || s < 0) fail("invalid dwell");
}
// Reuse the existing read-only host probe. Throwing aborts the entire callback;
// Rust classifies cancellation from the real token and never returns partial NC.
function checkPreflightAbort() {
  if (__ivac.abortCheck()) throw new Error("__IVAC_CANCELLED__");
}
function cycleCheck(r) {
  if (["drilling", "counter-boring", "deep-drilling"].indexOf(r.cycleType) < 0)
    fail("unsupported drill cycle " + r.cycleType);
  var c = r.params;
  var keys = ["feedrate", "clearance", "retract", "depth", "bottom", "dwell", "incrementalDepth"];
  for (var key in c) {
    if (keys.indexOf(key) < 0 || !finite(c[key])) fail("unsupported cycle parameter " + key);
  }
  feed(c.feedrate);
  if (!finite(c.clearance) || !finite(c.retract) || c.clearance <= 0 ||
      c.retract <= 0 || c.retract > c.clearance) fail("invalid cycle clearance");
  if (r.cycleType == "deep-drilling" &&
      (!finite(c.incrementalDepth) || c.incrementalDepth <= 0)) fail("invalid peck");
  if (c.dwell !== undefined) dwell(c.dwell);
  for (var i = 0; i < r.points.length; i++) {
    if ((i & 31) == 0) checkPreflightAbort();
    point(r.points[i]);
    if (r.points[i].z >= c.retract) fail("drill depth must be below retract");
  }
}
function preflight() {
  checkPreflightAbort();
  // ivaCAM-owned post uses versioned IR to catch records the general CPS
  // dispatcher can omit (machine rapids, unknown/reserved record kinds).
  var machineText = getGlobalParameter("ivacam:machine-config");
  if (typeof machineText != "string") fail("missing machine scaffold context");
  var machine = JSON.parse(machineText);
  if (machine.mode != "mill" || machine.tool_change != "manual_m0_pause")
    fail("requires milling and manual_m0_pause tool changes");
  if (machine.toolchange_xy || machine.park_xy || machine.park_at_home || machine.use_tool_length_offsets ||
      machine.post_change_z || machine.post_profile)
    fail("probing, controller offsets, stations, parking and custom scaffold unsupported");
  safeZ = getGlobalParameter("ivacam:clearance-z");
  if (!finite(safeZ) || safeZ <= 0) fail("positive authored clearance Z required");
  labSpindle1 = getProperty("labSpindle1Control");
  if (typeof labSpindle1 != "boolean") fail("labSpindle1Control must be boolean");
  var program = __ivacRun.program;
  if (program.version != 1 || program.machine) fail("unsupported IR or kinematics");
  if (!program.sections.length) fail("no milling sections");
  for (var i = 0; i < program.sections.length; i++) {
    checkPreflightAbort();
    var s = program.sections[i];
    if (["contour2d", "pocket2d", "drill", "milling", "planar_surface", "vcarve"].indexOf(s.strategy) < 0)
      fail("unsupported strategy " + s.strategy);
    if (s.strategy == "planar_surface" && s.tool.toolType != TOOL_MILLING_END_FLAT)
      fail("planar surfacing requires a flat end mill");
    if (s.strategy == "vcarve") {
      if (s.tool.toolType != TOOL_MILLING_CHAMFER)
        fail("V-Carve requires its qualified V-bit strategy/tool pairing");
    } else if ([TOOL_MILLING_END_FLAT, TOOL_DRILL].indexOf(s.tool.toolType) < 0)
      fail("only flat end mills and drills supported outside qualified V-Carve");
    if (s.tool.coolant != COOLANT_DISABLED || !s.spindleClockwise ||
        !finite(s.spindleRpm) || s.spindleRpm <= 0) fail("requires CW spindle, positive RPM, no coolant");
    if (labSpindle1 && (s.tool.number != 1 || s.tool.toolType != TOOL_MILLING_END_FLAT || s.spindleRpm > 18000 || s.spindleRpm != Math.floor(s.spindleRpm)))
      fail("lab spindle mode requires preloaded flat end mill 1 and integer RPM <= 18000; tool changes unsupported");
    if (s.workOffset != 1) fail("only one operator-established work zero supported");
    var identity = [[1,0,0],[0,1,0],[0,0,1]];
    for (var row = 0; row < 3; row++) for (var col = 0; col < 3; col++)
      if (s.workPlane[row][col] != identity[row][col]) fail("rotated work plane unsupported");
    var entry = "direct", entryTabs = false, entryLeads = false, pathHelix = false, entryStartZ;
    for (var k = 0; k < (s.parameters || []).length; k++) {
      if ((k & 31) == 0) checkPreflightAbort();
      var param = s.parameters[k];
      if (param.name == "ivacam:entry-strategy") entry = param.value;
      if (param.name == "ivacam:entry-start-z") entryStartZ = param.value;
      if (["ivacam:wcs-x", "ivacam:wcs-y", "ivacam:wcs-z"].indexOf(param.name) >= 0 && param.value !== 0)
        fail("operation " + s.id + ": translated work origins are not qualified by the current coordinate pipeline; use a zero numeric G54 offset and explicitly position geometry. Operation parameters were kept");
      if (param.name == "ivacam:entry-has-tabs") entryTabs = param.value;
      if (param.name == "ivacam:entry-has-leads") entryLeads = param.value;
      if (param.name == "ivacam:path-helix") pathHelix = param.value;
      if (s.parameters[k].name == "ivacam:uses-static-tool-z-shift" && s.parameters[k].value)
        fail("static tool Z shifts incompatible with manual work Z zero");
    }
    if (entry == "helix" || pathHelix) {
      var helixZ = s.initialPosition.z, hasHelicalMotion = false;
      for (var h = 0; h < s.records.length; h++) {
        if ((h & 31) == 0) checkPreflightAbort();
        var motion = s.records[h];
        if (motion.kind == "comment" && String(motion.text).indexOf("ENTRY: requested Helix; resolved") == 0)
          fail("operation " + s.id + ": requested Helix was not geometrically realized (entry radius/space or open path); generic motion substituted Ramp/Direct. ShopBot refuses this fallback. Choose another entry explicitly. Operation parameters were kept");
        if (motion.kind == "circular") {
          if (motion.end.z != helixZ) hasHelicalMotion = true;
          helixZ = motion.end.z;
        } else if (motion.kind == "linear" || motion.kind == "rapid") helixZ = motion.z;
      }
      if (!hasHelicalMotion)
        fail("operation " + s.id + ": requested Helix was not geometrically realized (entry radius/space, open path or composition); generic motion substituted Ramp/Direct. ShopBot refuses this fallback. Choose another entry explicitly. Operation parameters were kept");
      fail("operation " + s.id + ": helical entry/descent is not qualified for this post (contour re-plunge and complete region clearance remain unqualified); choose Ramp (without tabs/leads) or Direct. Operation parameters were kept");
    }
    if (entry == "ramp" && (entryTabs || entryLeads))
      fail("operation " + s.id + ": Ramp with tabs or leads is not qualified; the generic emitter can substitute direct entry. Choose Direct explicitly, or remove tabs/leads. Operation parameters were kept");
    if (entry == "ramp" && (!finite(entryStartZ) || entryStartZ >= safeZ))
      fail("operation " + s.id + ": Ramp requires a finite start depth below authored clearance");
    point(s.initialPosition); point(s.finalPosition);
    var p = s.initialPosition;
    for (var j = 0; j < s.records.length; j++) {
      // A cycle record can contain many points: cycleCheck polls that inner loop.
      // This 32-item bound is independent of the dispatcher's emission cadence.
      if ((j & 31) == 0) checkPreflightAbort();
      var r = s.records[j];
      switch (r.kind) {
      case "rapid": case "linear":
        point(r);
        if (r.kind == "linear") feed(r.feed);
        if (entry == "ramp" && r.kind == "linear" && r.z < entryStartZ && r.z < p.z &&
            xyz.format(r.x) == xyz.format(p.x) && xyz.format(r.y) == xyz.format(p.y))
          fail("operation " + s.id + ": requested Ramp became a direct plunge (path too short or entry combination unsupported). Choose Direct explicitly or revise the entry. Operation parameters were kept");
        if (r.kind == "rapid" && (r.x != p.x || r.y != p.y) && r.z <= 0 && p.z <= 0)
          fail("lateral rapid below stock");
        p = r;
        break;
      case "circular":
        if (s.strategy == "vcarve") fail("V-Carve requires qualified linear XYZ; arcs are unsupported");
        point(r.center); point(r.end); feed(r.feed);
        if ((r.end.z != p.z && entry != "ramp") || r.normal[0] != 0 || r.normal[1] != 0 || r.normal[2] != 1)
          fail("only planar XY arcs supported");
        p = r.end;
        break;
      case "cycle":
        cycleCheck(r);
        if (r.points.length) {
          var last = r.points[r.points.length - 1];
          p = {x:last.x, y:last.y, z:r.params.clearance};
        }
        break;
      case "cycleEnd": case "comment": break;
      case "dwell": dwell(r.seconds); break;
      case "coolant":
        if (r.mode != COOLANT_DISABLED) fail("coolant unsupported");
        break;
      case "command":
        if (labSpindle1) fail("operator stop commands unsupported in preloaded lab spindle mode");
        if (r.command != COMMAND_STOP && r.command != COMMAND_OPTIONAL_STOP)
          fail("unsupported command " + getCommandStringId(r.command));
        break;
      case "spindleSpeed":
        fail("operation " + s.id + ": unsupported record spindleSpeed (intra-section RPM/direction change). Use equal rough/finish RPM explicitly for a single tool, or separately qualified manual tool/section boundaries. Tool and cutting settings were kept");
        break;
      default: fail("unsupported record " + r.kind);
      }
    }
  }
}
function onOpen() {
  preflight(); // whole program checked before any output
  checkPreflightAbort();
  warning(labSpindle1
    ? "Sorbonne spindle 1 mode: prepare tool 1 and XYZ zero before running. TR/SO4/SO1 wiring from operator macros; 2s startup. Repeat controller/physical qualification for this revision."
    : "ShopBot MVP uses manual spindle start/stop and RPM; verify every operator pause. Arcs linearized at 0.01 mm. No controller/hardware qualification.");
  comment(labSpindle1 ? "ivaCAM ShopBot MVP - metric XYZ - Sorbonne spindle 1" : "ivaCAM ShopBot MVP - metric XYZ - manual spindle");
  comment(programComment);
  writeln("VD,,1");
  writeln("SA");
  if (labSpindle1) {
    comment("Tool 1 installed and XYZ work zero verified BEFORE RUN. Controller jog settings retained.");
    writeln("SO,1,0");
  } else {
    pause("Stop spindle. Verify work zero, stock, clearances and controller jog speeds before continuing.");
  }
}
var provenanceSection = 0;
function onSection() {
  comment("FABTOOL/1 REQUEST " + provenanceSection + " " + tool.number);
  // Establish Z before any XY, then require explicit tool/zero verification.
  writeln("JZ," + xyz.format(safeZ));
  if (labSpindle1) {
    // Exact spindle-1 branch of supplied MTC/MTCon.sbp; no motion/offset macros.
    writeln("TR," + spindleSpeed);
    writeln("SO,4,1");
    writeln("SO,1,1");
    writeln("PAUSE 2.000");
  } else {
    pause("Spindle OFF and stationary. Load tool " + tool.number +
          " D=" + xyz.format(tool.diameter) + "; set work Z zero for this tool. Return to clearance before OK.");
    writeln("JZ," + xyz.format(safeZ));
    pause("Start spindle CW manually at " + spindleSpeed + " RPM. Wait until ready before OK.");
  }
  comment("FABTOOL/1 ACTIVE " + provenanceSection + " " + tool.number);
  provenanceSection++;
  comment(getParameter("operation-comment", "Operation"));
  position = {x:NaN, y:NaN, z:safeZ};
  lastFeed = undefined;
}
function onRapid(x,y,z) {
  point({x:x,y:y,z:z});
  var xy = x != position.x || y != position.y;
  if (xy && position.z <= 0 && z <= 0) fail("lateral rapid below stock");
  // Ordered Z lift -> XY -> Z descent. Do not assume synchronized rapid axes.
  if (z > position.z) writeln("JZ," + xyz.format(z));
  if (xy) writeln("J2," + xyz.format(x) + "," + xyz.format(y));
  if (z < position.z) writeln("JZ," + xyz.format(z));
  position = {x:x,y:y,z:z};
}
function onLinear(x,y,z,f) {
  feed(f);
  if (f != lastFeed) {
    var value = speed.format(f / 60); // core mm/min -> OpenSBP mm/s
    if (Number(value) <= 0) fail("feed below output precision");
    writeln("MS," + value + "," + value);
    lastFeed = f;
  }
  writeln("M3," + xyz.format(x) + "," + xyz.format(y) + "," + xyz.format(z));
  position = {x:x,y:y,z:z};
}
function onCircular() { linearize(tolerance); }
function onCyclePoint(x,y,z) { expandCyclePoint(x,y,z); }
function onDwell(s) { dwell(s); if (s > 0) writeln("PAUSE " + secondsFormat.format(s)); }
function onComment(text) { comment(text); }
function onCommand(c) {
  if (c != COMMAND_STOP && c != COMMAND_OPTIONAL_STOP) fail("unsupported command");
  pause("Operator stop requested. Verify spindle and setup before continuing.");
}
function onPassThrough() { fail("raw commands unsupported"); }
function onSectionEnd() {
  writeln("JZ," + xyz.format(safeZ));
  if (labSpindle1) {
    // The reviewed MTCoff turns SO1 off. No secondary-spindle/drill output writes.
    writeln("SO,1,0");
  } else {
    pause("Stop spindle manually and wait until stationary before OK.");
  }
}
function onClose() { writeln("END"); }
