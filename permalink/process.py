import sys
import json
import os
import re
import glob

SOURCE = sys.argv[1]
TARGET = sys.argv[2]

with open(os.path.join(SOURCE, "_xrefs.json")) as f:
    XREFS = json.load(f)

with open(os.path.join(SOURCE, "_site.json")) as f:
    SITE = json.load(f)

FILES = {}

for full_path in glob.glob("{}/**/*".format(SOURCE), recursive=True):
    if os.path.isdir(full_path):
        continue

    file_path = os.path.relpath(full_path, SOURCE)

    if file_path == "xrefs.json":
        continue

    if os.path.splitext(file_path)[1] == ".jsondoc":
        with open(os.path.join(SOURCE, file_path)) as f:
            source_path = json.load(f)["sourcePath"]
        if os.path.basename(os.path.splitext(file_path)[0]) == "index.html":
            output_path = file_path
        elif os.path.splitext(os.path.splitext(file_path)[0])[1] == ".html":
            output_path = os.path.join(
                os.path.splitext(os.path.splitext(file_path)[0])[0],
                "index.html.jsondoc"
            )
        else:
            output_path = file_path

        output_path = re.sub(r"^_posts/(\d\d\d\d)-(\d\d)-(\d\d)-", "\\1/\\2/\\3/", output_path)
        output_path = re.sub(r"/_posts/(\d\d\d\d)-(\d\d)-(\d\d)-", "/\\1/\\2/\\3/", output_path)

        final_path = os.path.splitext(output_path)[0]
        is_jsondoc = True
    else:
        source_path = file_path
        final_path = file_path
        output_path = file_path
        is_jsondoc = False

    if os.path.splitext(final_path)[1] == ".html":
        basename = os.path.basename(final_path)
        if basename == "index.html":
            final_path = final_path
        else:
            final_path = os.path.join(os.path.splitext(final_path)[0], "index.html")

    if os.path.basename(final_path) == "index.html":
        if os.path.dirname(final_path) == "":
            bare_link = ""
        else:
            bare_link = os.path.dirname(final_path) + "/"
    else:
        bare_link = final_path
    link = SITE["baseUrl"] + bare_link

    full_base = SITE["url"]
    if not full_base.endswith("/"):
        full_base = full_base + "/"
    full_link = full_base + bare_link

    FILES[file_path] = {
        "source_path": source_path, # Original path of the file, eg. "sedbin.adoc"
        "final_path": final_path, # Final path in the result website, eg. "sedbin/index.html"
        "output_path": output_path, # Output path for the operation, eg. "sedbin/index.html.jsondoc"
        "link": link, # Site specific link, eg. "/sedbin/"
        "full_link": full_link, # Non-site specific link, eg. "https://corepaper.org/sedbin/"
        "is_jsondoc": is_jsondoc, # Whether it's jsondoc, eg. True
    }

def find_file_by_source_path(source_path):
    for file_path in FILES:
        if FILES[file_path]["source_path"] == source_path:
            return FILES[file_path]
    return None

def process_macros(content):
    for match in re.findall(r"@@XREFLINK:([^#@]*)@@", content):
        full_match = "@@XREFLINK:{}@@".format(match)
        xref_item = find_file_by_source_path(match)
        if xref_item is None:
            raise Exception("XREF item not found {}".format(match))
        xref_link = xref_item["link"]
        content = content.replace(full_match, xref_link)
    for match in re.findall(r"@@XREFPATH:([^#@]*)@@", content):
        full_match = "@@XREFPATH:{}@@".format(match)
        xref_item = find_file_by_source_path(match)
        if xref_item is None:
            raise Exception("XREF item not found {}".format(match))
        xref_path = xref_item["path"]
        content = content.replace(full_match, xref_path)
    for match in re.findall(r"@@XREFFULLLINK:([^#@]*)@@", content):
        full_match = "@@XREFFULLLINK:{}@@".format(match)
        xref_item = find_file_by_source_path(match)
        if xref_item is None:
            raise Exception("XREF item not found {}".format(match))
        xref_full_link = xref_item["full_link"]
        content = content.replace(full_match, xref_full_link)

    return content

for file_path in FILES:
    item = FILES[file_path]
    full_file_path = os.path.join(SOURCE, file_path)

    with open(full_file_path, "rb") as f:
        content = f.read()
    if file_path in XREFS:
        content = content.decode()
        content = process_macros(content)
        content = content.encode()
    if item["is_jsondoc"]:
        document = json.loads(content)
        document["url"] = item["full_link"]
        content = json.dumps(document, sort_keys=True, indent=4, separators=(',', ': ')).encode()

    os.makedirs(os.path.dirname(os.path.join(TARGET, item["output_path"])), exist_ok=True)
    with open(os.path.join(TARGET, item["output_path"]), "wb") as f:
        f.write(content)
