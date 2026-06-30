#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd)"

# Can have a diff under env: 
#   Control Group OpenJDK Version: 
#       openjdk version "25.0.3" 2026-04-21
#       OpenJDK Runtime Environment (build 25.0.3)
#       OpenJDK 64-Bit Server VM (build 25.0.3, mixed mode, sharing)
#   GraalVM JDK Version: 
#       openjdk version "25.0.3-internal" 2026-04-21
#       OpenJDK Runtime Environment GraalVM CE 25.1.0-dev.internal-adhoc.syc.labs-openjdk.1 (build 25.0.3-internal-adhoc.syc.labs-openjdk)
#       OpenJDK 64-Bit Server VM GraalVM CE 25.1.0-dev.internal-adhoc.syc.labs-openjdk.1 (build 25.0.3-internal-adhoc.syc.labs-openjdk, mixed mode, sharing)
#   GraalVM Commit: d91a2b7e4647d6639d689d878f7255c9cd83ed32
#   LabsOpenJDK Commit:  a04ba0087a48ce0091be170d845fe24769590943 (HEAD, tag: jvmci-25.1-b18)
#   MX Commit: cf215dfd24f26a31cea19416599c041ce6d664ff (HEAD -> master, tag: 7.82.3, origin/master, origin/HEAD)
#   Build GCC: gcc-10 (Arch Linux 10.5.0-2) 10.5.0
#   OS: Linux 7.0.11-arch1-1
RUN_A='/home/syc/Desktop/code/jvm/jdk25u-graal/build/bin/java -ea -esa -XX:+UseJVMCICompiler -Dgraal.CompilationFailureAction=ExitVM -Xmixed --add-modules jdk.incubator.vector -Dtest.jdk=/home/syc/Desktop/code/jvm/jdk25u-graal/build/ -Djdk.test.lib.random.seed=1 {input}'
RUN_B='/usr/lib/jvm/java-25-openjdk/bin/java -ea -esa --add-modules jdk.incubator.vector {input}'

cargo run --manifest-path "${REPO_ROOT}/Cargo.toml" -- reduce \
  --lang java \
  --input "${SCRIPT_DIR}/Test.java" \
  --output "${SCRIPT_DIR}/Test.min.weighted-random.java" \
  --run-a "${RUN_A}" \
  --run-b "${RUN_B}" \
  --timeout 120s \
  --max-trials 0 \
  --algorithm weighted-random \
  --stop-size-percent 25 \
  --keep-temp \
  --json-report "${SCRIPT_DIR}/reduction-report.weighted-random.json"

