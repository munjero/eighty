from template import render_document

import sys
import json
import os
import re
import glob

SOURCE = sys.argv[1]
TARGET = sys.argv[2]

with open(os.path.join(SOURCE, "_site.json")) as f:
    SITE = json.load(f)

with open(os.path.join(SOURCE, "_sitemap.json")) as f:
    SITEMAP = json.load(f)

def populate_sitemap_info(document, sitemap, depth=0):
    if sitemap["sourcePath"] == document["sourcePath"]:
        document["children"] = sitemap["children"]
        document["depth"] = depth
        return

    for child in sitemap["children"]:
        populate_sitemap_info(document, child, depth + 1)

for full_path in glob.glob("{}/**/*".format(SOURCE), recursive=True):
    if os.path.isdir(full_path):
        continue

    file_path = os.path.relpath(full_path, SOURCE)
    with open(full_path, "rb") as f:
        content = f.read()

    if os.path.splitext(file_path)[1] == ".jsondoc":
        document = json.loads(content)
        if "author" in SITE and not "author" in document:
            document["author"] = SITE["author"]
        if "email" in SITE and not "email" in document:
            document["email"] = SITE["email"]
        output_path = os.path.splitext(file_path)[0]

        populate_sitemap_info(document, SITEMAP)

        content = render_document(document, SITE, SITEMAP).encode()
    else:
        output_path = file_path

    os.makedirs(os.path.dirname(os.path.join(TARGET, output_path)), exist_ok=True)
    with open(os.path.join(TARGET, output_path), "wb") as f:
        f.write(content)
