import sys
import json
import os
import re
import glob
import subprocess
from datetime import datetime

GITDIR = sys.argv[1]
SOURCE = sys.argv[2]
TARGET = sys.argv[3]

GIT_JSON_FILE_LOG_EXEC = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    "git-json-file-log.sh"
)

for full_path in glob.glob("{}/**/*".format(SOURCE), recursive=True):
    if os.path.isdir(full_path):
        continue

    file_path = os.path.relpath(full_path, SOURCE)
    with open(full_path, "rb") as f:
        content = f.read()

    if os.path.splitext(file_path)[1] == ".jsondoc":
        document = json.loads(content)
        executed = subprocess.run("sh {} -- {}".format(GIT_JSON_FILE_LOG_EXEC, document["sourcePath"]),
                                  cwd=GITDIR, shell=True, capture_output=True)
        assert executed.returncode == 0
        commits = json.loads(executed.stdout)

        if not len(commits) == 0:
            created = datetime.fromisoformat(commits[-1]["date"]).strftime('%Y-%m-%d')
            updated = datetime.fromisoformat(commits[0]["date"]).strftime('%Y-%m-%d')

            document.setdefault("revision", {})
            document["revision"].setdefault("created", created)
            document["revision"].setdefault("updated", updated)

        content = json.dumps(document, sort_keys=True, indent=4, separators=(',', ': ')).encode()

    output_path = file_path

    os.makedirs(os.path.dirname(os.path.join(TARGET, output_path)), exist_ok=True)
    with open(os.path.join(TARGET, output_path), "wb") as f:
        f.write(content)
