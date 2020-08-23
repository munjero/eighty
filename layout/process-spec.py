from template import render_spec_index, render_spec_redirect

import sys
import json
import os
import re
import glob

SOURCES = sys.argv[1:-1]
TARGET = sys.argv[-1]

SPECS = []

for source in SOURCES:
    with open(source) as f:
        for spec in json.load(f):
            SPECS.append(spec)


index_path = os.path.join(TARGET, "index.html")
with open(index_path, "w") as f:
    f.write(render_spec_index(SPECS))

for spec in SPECS:
    upper_path = os.path.join(TARGET, spec["id"].upper(), "index.html")
    lower_path = os.path.join(TARGET, spec["id"].lower(), "index.html")
    content = render_spec_redirect(spec)
    os.makedirs(os.path.dirname(upper_path), exist_ok=True)
    os.makedirs(os.path.dirname(lower_path), exist_ok=True)
    with open(upper_path, "w") as f:
        f.write(content)
    with open(lower_path, "w") as f:
        f.write(content)
