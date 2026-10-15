#!/usr/bin/env python3
"""Serial, alternating-order process pairs; never run alongside compilation."""
import argparse
import pathlib
import subprocess
import sys

p = argparse.ArgumentParser()
p.add_argument('--control', required=True)
p.add_argument('--candidate', required=True)
p.add_argument('--requests', required=True)
p.add_argument('--output', required=True)
p.add_argument('--work', required=True)
p.add_argument('--rounds', type=int, default=3)
a = p.parse_args()
for iteration in range(a.rounds):
    order = [('control',a.control),('candidate',a.candidate)]
    if iteration % 2: order.reverse()
    for case in ['text-Test-medial-cps','profile-many-segments-cps','pocket-islands-cps']:
        for label,binary in order:
            relative = pathlib.Path(str(iteration))/label
            subprocess.run([sys.executable, 'scripts/perf1-native.py', '--binary',binary,
                '--requests',a.requests,'--output',str(pathlib.Path(a.output)/relative),
                '--work',str(pathlib.Path(a.work)/relative),'--repeats','7','--case',case],check=True)
