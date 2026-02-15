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
    "gen_pg_types_test_content",
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

def find_rust_files(root):
    for dirpath, dirnames, filenames in os.walk(root):
        # skip .git directories
        if '.git' in dirpath:
            continue
        for f in filenames:
            if f.endswith('.rs'):
                yield os.path.join(dirpath, f)

def detect_quote_blocks(content):
    """Return a list of (start, end) positions of quote!{} blocks.
    Simple heuristic: find 'quote!{' and matching '}' ignoring nested braces.
    Not perfect but works for most cases."""
    lines = content.split('\n')
    in_quote = False
    brace_depth = 0
    blocks = []
    start = 0
    for i, line in enumerate(lines):
        # Check for quote!{ on this line
        if 'quote!{' in line and not in_quote:
            in_quote = True
            start = i
            # compute brace depth from the opening brace after quote!{
            idx = line.find('quote!{')
            sub = line[idx + len('quote!{'):]
            brace_depth = sub.count('{') - sub.count('}')
            if brace_depth == 0:
                # closing brace on same line
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

def analyze_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    blocks = detect_quote_blocks(content)
    lines = content.split('\n')
    crate_usage = {}
    for i, line in enumerate(lines):
        if is_inside_quote(i, blocks):
            continue
        for crate in CRATES:
            # pattern crate::something
            pattern = rf'\b{crate}::'
            if re.search(pattern, line):
                crate_usage.setdefault(crate, []).append(i+1)
    if crate_usage:
        print(f"\n{path}:")
        for crate, lines in crate_usage.items():
            print(f"  {crate} at lines {lines}")

def main():
    root = '.'
    for f in find_rust_files(root):
        analyze_file(f)

if __name__ == '__main__':
    main()