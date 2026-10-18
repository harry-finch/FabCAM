#!/usr/bin/env python3
"""Build outside the watchdog budget; print only the executable on stdout."""
import json, os, subprocess, sys
r=subprocess.run(['cargo','test','--locked','--release','-p','ivac-core','--features','cps,profiling','--test','test1_properties','--no-run','--message-format=json'],env=dict(os.environ,RUSTC_WRAPPER=''),stdout=subprocess.PIPE,text=True)
for line in r.stdout.splitlines():
 value=json.loads(line)
 if value.get('reason')=='compiler-message':print(value['message'].get('rendered',''),file=sys.stderr)
 if value.get('executable') and value.get('target',{}).get('name')=='test1_properties':print(value['executable'])
raise SystemExit(r.returncode)
