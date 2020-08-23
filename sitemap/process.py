import json
import os
import glob
import sys

SOURCE = sys.argv[1]
TARGET = sys.argv[2]

with open(os.path.join(SOURCE, "_site.json")) as f:
    SITE = json.load(f)

with open(os.path.join(SOURCE, "_xrefs.json")) as f:
    XREFS = json.load(f)

with open(os.path.join(SOURCE, "index.html.jsondoc")) as f:
    INDEX_JSONDOC = json.load(f)

SITEMAP = {
    "title": SITE["title"],
    "url": "@@XREFLINK:{}@@".format(INDEX_JSONDOC["sourcePath"]),
    "sourcePath": INDEX_JSONDOC["sourcePath"],
    "children": {},
}

for full_path in glob.glob("{}/**/*".format(SOURCE), recursive=True):
    if os.path.isdir(full_path):
        continue

    file_path = os.path.relpath(full_path, SOURCE)
    with open(full_path, "rb") as f:
        content = f.read()

    if os.path.splitext(file_path)[1] == ".jsondoc":
        document = json.loads(content)
        breadcrumbs = [{
            "title": document["title"],
            "sourcePath": document["sourcePath"],
            "description": document["description"] if "description" in document else None,
            "order": document["order"] if "order" in document else None,
            "url": "@@XREFLINK:{}@@".format(document["sourcePath"])
        }]
        if os.path.basename(file_path) == "index.html.jsondoc":
            current = os.path.dirname(os.path.dirname(file_path))
        else:
            current = os.path.dirname(file_path)

        while current != "":
            if os.path.isfile(os.path.join(SOURCE, current, "index.html.jsondoc")):
                with open(os.path.join(SOURCE, current, "index.html.jsondoc"), "r") as f:
                    current_document = json.load(f)
                    breadcrumbs.append({
                        "title": current_document["title"],
                        "sourcePath": current_document["sourcePath"],
                        "description": current_document["description"] if "description" in current_document else None,
                        "order": current_document["order"] if "order" in current_document else None,
                        "url": "@@XREFLINK:{}@@".format(current_document["sourcePath"])
                    })

            current = os.path.dirname(current)

        if file_path == "index.html.jsondoc":
            breadcrumbs = []

        breadcrumbs.reverse()

        current = SITEMAP
        for breadcrumb in breadcrumbs:
            if not breadcrumb["sourcePath"] in current["children"]:
                current["children"][breadcrumb["sourcePath"]] = {
                    "title": breadcrumb["title"],
                    "url": breadcrumb["url"],
                    "description": breadcrumb["description"],
                    "sourcePath": breadcrumb["sourcePath"],
                    "order": breadcrumb["order"],
                    "children": {}
                }
            current = current["children"][breadcrumb["sourcePath"]]

        document["breadcrumbs"] = breadcrumbs

        content = json.dumps(document, sort_keys=True, indent=4, separators=(',', ': ')).encode()

    output_path = os.path.join(TARGET, file_path)
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "wb") as f:
        f.write(content)

def flatten_and_sort_sitemap(sitemap):
    sitemap["children"] = list(sorted(sitemap["children"].values(), key=lambda c: (c["order"] if not c["order"] is None else 999, c["title"])))

    for child in sitemap["children"]:
        flatten_and_sort_sitemap(child)

flatten_and_sort_sitemap(SITEMAP)

with open(os.path.join(TARGET, "_sitemap.json"), "w") as f:
    f.write(json.dumps(SITEMAP, sort_keys=True, indent=4, separators=(',', ': ')))

XREFS.append("_sitemap.json")

with open(os.path.join(TARGET, "_xrefs.json"), "w") as f:
    f.write(json.dumps(XREFS, sort_keys=True, indent=4, separators=(',', ': ')))
