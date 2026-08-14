---
slug: norm-version-line
form: norm
type: formulation
max: decided
state: influx
---

# Norm: the version line is name + semver

`--version` prints the tool name and a SemVer (`MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]`). Add a commit SHA when this build is not a tagged release.

Not bound: date, compiler, OS/arch, and the rest of the convention display dump. Self-update machinery waits on a published binary.

Provenance (gather, not authority): [[../influx/cli-conventions/versioning-and-updates#Version Display|version display axis]] · [[../influx/cli-conventions/versioning-and-updates#Version Format|semver]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
