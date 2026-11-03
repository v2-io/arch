"""Step 1: does the API accept Claude Code's OAuth token from outside Claude Code?"""
import json, sys
from api import Run

run = Run(sys.argv[1])
status, data = run.call("smoke", {
    "model": "claude-haiku-5-5",
    "max_tokens": 64,
    "messages": [{"role": "user", "content": "Reply with the single word OK."}],
})
print("status:", status)
print(json.dumps(data, indent=2))
