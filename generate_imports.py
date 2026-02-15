#!/usr/bin/env python3
import os
import re
import sys

CRATES = [
    "app_state",
    "common_routes",
    "config_lib",
    "constants",
    "to_err_string",
    "enum_extension",
    "enum_extension_lib",
    "error_occurence_lib",
    "from_sqlx_pg_error",
    "from_str",
    "git_info",
    "git_sync_command",
    "pg_crud",
    "pg_crud_common",
    "pg_crud_common_and_macros_common",
    "pg_crud_macros_common",
    "pg_json_object_type",
    "gen_pg_json_object_type",
    "gen_pg_json_object_type_source",
    "gen_pg_json_object_type_test",
    "gen_pg_json_object_type_test_content",
    "pg_json_object_type_common",
    "pg_json_types",
    "gen_pg_json_types",
    "gen_pg_json_types_common",
    "gen_pg_json_types_source",
    "gen_pg_json_types_test",
    "gen_pg_json_types_test_content",
    "pg_table",
    "gen_pg_table",
    "gen_pg_table_source",
    "gen_pg_table_test",
    "gen_pg_table_test_content",
    "pg_types",
    "gen_pg_types",
    "gen_pg_types_source",
    "gen_pg_types_test",
    "gen_pg_json_types_test_content",
    "pg_types_common",
    "where_filters",
    "gen_where_filters",
    "gen_quotes",
    "macros_helpers",
    "gen_struct_or_enum_derive_ts_builder",
    "server",
    "server_app_state",
    "server_config",
    "server_table_example",
    "server_types",
    "telegram_bot",
    "gen_getter_traits_for_struct_fields",
    "try_from_env",
    "server_port",
    "server_port_try_from_u16",
    "server_port_common",
    "http_logic",
    "route_validators",
    "tests",
    "impl_display_as_debug",
    "macro_clippy_check_common",
    "error_occurence_test",
    "file_system",
    "token_patterns",
    "naming",
    "naming_common",
    "naming_macros",
    "panic_location",
    "providers"
]

def detect_quote_blocks(content):
    lines = content.split('\n')
    in_quote = False
    brace_depth = 0
    blocks = []
    start = 0
    for i, line in enumerate(lines):
        if 'quote!{' in line and not in_quote:
            in_quote = True
            start = i
            idx = line.find('quote!{')
            sub = line[idx + len('quote!{'):]
            brace_depth = sub.count('{') - sub.count('}')
            if brace_depth == 0:
                in_quote = False
                blocks.append((start, i))
            continue
        if in_quote:
            brace_depth += line.count('{') - line.count('}')
            if brace_depth == 0:
                in_quote = False
                blocks.append((start, i))
    return blocks

def is_inside_quote(line_idx, blocks):
    for start, end in blocks:
        if start <= line_idx <= end:
            return True
    return False

def extract_used_paths(line):
    """Return list of full crate paths found in line."""
    paths = []
    # pattern: crate::something::something_else
    for crate in CRATES:
        pattern = rf'\b{crate}::(\w+(::\w+)*)'
        for match in re.finditer(pattern, line):
            full = match.group(0)  # e.g., app_state::SourcePlaceType
            paths.append(full)
    return paths

def analyze_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    blocks = detect_quote_blocks(content)
    lines = content.split('\n')
    used_paths = set()
    for i, line in enumerate(lines):
        if is_inside_quote(i, blocks):
            continue
        paths = extract_used_paths(line)
        for p in paths:
            used_paths.add(p)
    return used_paths

def collect_existing_uses(content):
    """Return set of imported paths (as strings) from use statements."""
    uses = set()
    # naive regex for use statements
    use_re = r'^\s*use\s+([^;]+);'
    for line in content.split('\n'):
        m = re.match(use_re, line.strip())
        if m:
            use_stmt = m.group(1)
            # simplify: just add the whole use statement as is
            uses.add(use_stmt)
    return uses

def main():
    root = '.'
    for dirpath, dirnames, filenames in os.walk(root):
        if '.git' in dirpath:
            continue
        for f in filenames:
            if f.endswith('.rs'):
                full = os.path.join(dirpath, f)
                used = analyze_file(full)
                if used:
                    print(f"\n{full}:")
                    for p in sorted(used):
                        print(f"  {p}")

if __name__ == '__main__':
    main()