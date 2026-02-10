#!/usr/bin/env python3
"""Remove mcp__* entries from settings.local.json permissions.allow for baseagent."""
import json

path = "/workspace/.claude/settings.local.json"
with open(path) as f:
    data = json.load(f)

if "permissions" in data and "allow" in data["permissions"]:
    data["permissions"]["allow"] = [
        p for p in data["permissions"]["allow"] if not (isinstance(p, str) and p.startswith("mcp__"))
    ]

with open(path, "w") as f:
    json.dump(data, f, indent=2)
