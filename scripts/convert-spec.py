#!/usr/bin/env python3
"""Convert Freestyle's OpenAPI 3.1 spec to 3.0.3 for progenitor compatibility."""

import json
import re
import sys
from collections import Counter


def fix_type_arrays(text: str) -> str:
    """Convert "type": ["X", "null"] to "type": "X", "nullable": true at the text level."""

    def replace(match):
        types = json.loads(match.group(1))
        non_null = [t for t in types if t != "null"]
        has_null = "null" in types
        if len(non_null) == 1:
            r = '"type": "' + non_null[0] + '"'
            if has_null:
                r += ', "nullable": true'
            return r
        elif len(non_null) == 0:
            return '"type": "object", "nullable": true' if has_null else '"type": "object"'
        else:
            parts = ", ".join('{"type": "' + t + '"}' for t in non_null)
            r = '"oneOf": [' + parts + "]"
            if has_null:
                r += ', "nullable": true'
            return r

    return re.sub(r'"type"\s*:\s*(\[[^\]]*\])', replace, text)


def fix_null_schemas(obj):
    """Remove {"type": "null"} from oneOf/anyOf, collapse singletons, fix standalone null types."""
    if isinstance(obj, dict):
        for key in ["anyOf", "oneOf"]:
            if key in obj and isinstance(obj[key], list):
                null_entries = [
                    s
                    for s in obj[key]
                    if isinstance(s, dict) and s.get("type") == "null"
                ]
                non_null = [
                    s
                    for s in obj[key]
                    if not (isinstance(s, dict) and s.get("type") == "null")
                ]
                if null_entries:
                    if len(non_null) == 1:
                        del obj[key]
                        obj.update(non_null[0])
                        obj["nullable"] = True
                    elif len(non_null) == 0:
                        del obj[key]
                        obj["nullable"] = True
                        obj.setdefault("type", "object")
                    else:
                        obj[key] = non_null
                        obj["nullable"] = True
                elif len(obj[key]) == 1 and key == "oneOf":
                    single = obj[key][0]
                    del obj[key]
                    obj.update(single)

        if obj.get("type") == "null":
            obj["type"] = "object"
            obj["nullable"] = True

        for v in list(obj.values()):
            fix_null_schemas(v)
    elif isinstance(obj, list):
        for item in obj:
            fix_null_schemas(item)


def remove_null_defaults(obj):
    """Remove all null default values — typify rejects them."""
    if isinstance(obj, dict):
        if "default" in obj and obj["default"] is None:
            del obj["default"]
        for v in obj.values():
            remove_null_defaults(v)
    elif isinstance(obj, list):
        for v in obj:
            remove_null_defaults(v)


def fix_defaults(obj):
    """Fix mistyped default values."""
    if isinstance(obj, dict):
        if "default" in obj:
            typ = obj.get("type")
            default = obj["default"]
            # String default on array type
            if typ == "array" and isinstance(default, str):
                obj["default"] = [default]
            # Object default on non-object (schema leaking into default)
            elif isinstance(default, dict) and "type" in default and typ != "object":
                del obj["default"]
        for v in list(obj.values()):
            fix_defaults(v)
    elif isinstance(obj, list):
        for v in obj:
            fix_defaults(v)


def remove_legacy_paths(spec):
    """Remove legacy /git/v1/identity routes superseded by /identity/v1."""
    legacy = [p for p in spec["paths"] if p.startswith("/git/v1/identity")]
    for p in legacy:
        del spec["paths"][p]
    return len(legacy)


def fix_duplicate_operation_ids(spec):
    """Deduplicate operationIds."""
    # Fix fetch deployment proxy (7 methods, same operationId)
    fetch_path = "/web/v1/deployments/{deployment_id}/fetch"
    if fetch_path in spec["paths"]:
        for method, details in spec["paths"][fetch_path].items():
            if isinstance(details, dict) and "operationId" in details:
                details["operationId"] = f"handle_fetch_deployment_{method}"


def remove_proxy_endpoint(spec):
    """Remove the 7-method deployment proxy — progenitor can't model it."""
    spec["paths"].pop("/web/v1/deployments/{deployment_id}/fetch", None)


def fix_wildcard_paths(spec):
    """Convert {*param} wildcard syntax to standard {param}."""
    paths = spec["paths"]
    for path in list(paths.keys()):
        if "{*" in path:
            new_path = path.replace("{*", "{")
            # Avoid collision with existing path
            if new_path in paths:
                new_path = new_path.replace(
                    "{branch}", "create/{branch}"
                )
            paths[new_path] = paths.pop(path)


def fix_path_parameters(spec):
    """Ensure path parameters are required."""
    for path, methods in spec["paths"].items():
        for method, op in methods.items():
            if not isinstance(op, dict):
                continue
            for param in op.get("parameters", []):
                if param.get("in") == "path":
                    param["required"] = True
                    if "schema" in param:
                        param["schema"].pop("nullable", None)


def fix_response_collisions(spec):
    """Remove error responses that collide with success response schemas."""
    trigger_path = "/git/v1/repo/{repo}/trigger/{trigger}"
    if trigger_path in spec["paths"]:
        delete_op = spec["paths"][trigger_path].get("delete", {})
        delete_op.get("responses", {}).pop("404", None)


def fix_empty_response_schemas(spec):
    """Replace empty {} schemas with {type: object}."""
    for path, methods in spec["paths"].items():
        for method, op in methods.items():
            if not isinstance(op, dict):
                continue
            for code, resp in op.get("responses", {}).items():
                for ct, media in resp.get("content", {}).items():
                    if media.get("schema") == {}:
                        media["schema"] = {"type": "object"}


def validate(spec):
    """Check for remaining issues."""
    text = json.dumps(spec)
    issues = []

    if re.findall(r'"type"\s*:\s*\[', text):
        issues.append("type arrays remain")
    if re.findall(r'"type"\s*:\s*"null"', text):
        issues.append('"type": "null" remains')

    op_ids = []
    for path, methods in spec["paths"].items():
        for method, details in methods.items():
            if isinstance(details, dict) and "operationId" in details:
                op_ids.append(details["operationId"])
    dupes = {k: v for k, v in Counter(op_ids).items() if v > 1}
    if dupes:
        issues.append(f"duplicate operationIds: {dupes}")

    return issues


def convert(input_path: str, output_path: str):
    with open(input_path) as f:
        spec = json.load(f)

    # Handle double-JSON-encoding
    if isinstance(spec, str):
        spec = json.loads(spec)

    print(f"  Input: OpenAPI {spec.get('openapi')}, {len(spec.get('paths', {}))} paths, "
          f"{len(spec.get('components', {}).get('schemas', {}))} schemas")

    # Version
    spec["openapi"] = "3.0.3"
    spec.pop("webhooks", None)

    # Structural fixes
    n = remove_legacy_paths(spec)
    if n:
        print(f"  Removed {n} legacy /git/v1/identity paths")
    fix_duplicate_operation_ids(spec)
    remove_proxy_endpoint(spec)
    fix_wildcard_paths(spec)
    fix_response_collisions(spec)
    fix_empty_response_schemas(spec)

    # Type system conversion (text-level for completeness, then structural)
    text = json.dumps(spec)
    text = fix_type_arrays(text)
    spec = json.loads(text)
    fix_null_schemas(spec)
    remove_null_defaults(spec)
    fix_defaults(spec)

    # Path parameter fixes must run after type conversion to avoid re-introduction of nullable
    fix_path_parameters(spec)

    # Validate
    issues = validate(spec)
    if issues:
        print(f"  WARNING: {issues}")
    else:
        print("  Validation passed")

    with open(output_path, "w") as f:
        json.dump(spec, f, indent=2)

    print(f"  Output: {len(spec['paths'])} paths, "
          f"{len(spec['components']['schemas'])} schemas → {output_path}")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <input.json> <output.json>")
        sys.exit(1)
    convert(sys.argv[1], sys.argv[2])
