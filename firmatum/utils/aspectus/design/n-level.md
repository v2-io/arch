# Nth-level tree

Depth is generations **below** the root, not counting the root.

| `--depth` | What you see |
|---|---|
| `1` | the place and its children |
| `2` | plus grandchildren (the two-level look) |
| `3` | plus great-grandchildren |
| `0` | no limit |

Built-in default is **2**. Config (`depth`) and `--depth` use the caller stack. Still one line per shown node. No summarization on this row.

The old “two-level / no grandchildren” picture was a miscount: that is depth **1**.
