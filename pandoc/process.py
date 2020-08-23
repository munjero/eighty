import sys
import json
import os
import re
import glob
import subprocess

SOURCE = sys.argv[1]
TARGET = sys.argv[2]

TOC_TEMPLATE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "toc-template.html")

def parse_meta(pandoc_raw, name):
    if not name in pandoc_raw["meta"]:
        return None

    value_raw = pandoc_raw["meta"][name]
    value_raw["t"] = "Para"
    value_wrapped = json.dumps({
        "blocks": [value_raw],
        "pandoc-api-version": pandoc_raw["pandoc-api-version"],
        "meta": {}
    })
    value = subprocess.run("pandoc -f json -t plain", shell=True, check=True, input=value_wrapped.encode(), capture_output=True).stdout.decode("utf-8")

    return value.strip()

for full_path in glob.glob("{}/**/*".format(SOURCE), recursive=True):
    if os.path.isdir(full_path):
        continue

    file_path = os.path.relpath(full_path, SOURCE)
    with open(full_path, "rb") as f:
        content = f.read()

    if os.path.splitext(file_path)[1] == ".md":
        pandoc_raw = json.loads(subprocess.run("pandoc -f markdown -t json {}".format(full_path), shell=True, check=True, capture_output=True).stdout)

        title = parse_meta(pandoc_raw, "title")
        description = parse_meta(pandoc_raw, "subtitle")
        order = parse_meta(pandoc_raw, "order")

        if not order is None:
            order = int(order)

        html = subprocess.run("pandoc -f markdown -t html {}".format(full_path), shell=True, check=True, capture_output=True).stdout.decode("utf-8")
        toc = subprocess.run("pandoc --toc -f markdown -t html --template {} {}".format(TOC_TEMPLATE, full_path), shell=True, check=True, capture_output=True).stdout.decode("utf-8")

        content = json.dumps({
            "title": title,
            "description": description,
            "descriptionContent": description,
            "order": order,
            "sourcePath": file_path,
            "content": html,
            "toc": toc,
        }).encode()

        output_path = os.path.splitext(file_path)[0] + ".html.jsondoc"
    else:
        output_path = file_path

    os.makedirs(os.path.dirname(os.path.join(TARGET, output_path)), exist_ok=True)
    with open(os.path.join(TARGET, output_path), "wb") as f:
        f.write(content)
