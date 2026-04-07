default:
    @just --list

# Fetch the OpenAPI spec from Freestyle's API and generate the Rust client
generate:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Fetching OpenAPI spec..."
    curl -s https://api.freestyle.sh/openapi.json -o /tmp/freestyle-openapi-raw.json

    echo "Converting 3.1 → 3.0 and fixing spec issues..."
    python3 scripts/convert-spec.py /tmp/freestyle-openapi-raw.json openapi.json

    echo "Generating Rust client..."
    cargo progenitor -i openapi.json -o /tmp/freestyle-progenitor -n freestyle-sandboxes -v 0.1.0

    cp /tmp/freestyle-progenitor/src/lib.rs src/openapi.rs
    rm -rf /tmp/freestyle-progenitor

    echo "Done. Run 'cargo clippy' to verify."

clippy:
    cargo clippy

fmt:
    cargo fmt
