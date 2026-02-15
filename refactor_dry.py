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

def extract_used_paths(content):
    lines = content.split('\n')
    blocks = detect_quote_blocks(content)
    used = set()
    for i, line in enumerate(lines):
        if is_inside_quote(i, blocks):
            continue
        for crate in CRATES:
            pattern = rf'\b{crate}::(\w+(::\w+)*)'
            for match in re.finditer(pattern, line):
                full = match.group(0)
                used.add(full)
    return used

def collect_existing_imports(content):
    imports = set()
    use_re = r'^\s*use\s+([^;]+);'
    for line in content.split('\n'):
        m = re.match(use_re, line.strip())
        if m:
            imports.add(m.group(1))
    return imports

def path_to_import(path):
    parts = path.split('::')
    if len(parts) >= 2:
        import_path = '::'.join(parts[:2])
        return import_path
    else:
        return path

def generate_import_statements(used_paths, existing_imports):
    imports = set()
    for path in used_paths:
        imp = path_to_import(path)
        if imp in existing_imports:
            continue
        imports.add(imp)
    return sorted(imports)

def process_file(path, dry_run=True):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    used_paths = extract_used_paths(content)
    if not used_paths:
        return None
    existing_imports = collect_existing_imports(content)
    imports_to_add = generate_import_statements(used_paths, existing_imports)
    if not imports_to_add:
        return None
    if dry_run:
        print(f'Would modify {path}: add {imports_to_add}')
        return True
    # Actually modify (not implemented)
    return None

def main():
    root = '.'
    changed = 0
    for dirpath, dirnames, filenames in os.walk(root):
        if '.git' in dirpath:
            continue
        for f in filenames:
            if f.endswith('.rs'):
                full = os.path.join(dirpath, f)
                if process_file(full, dry_run=True):
                    changed += 1
    print(f'Total files that would be changed: {changed}')

if __name__ == '__main__':
    main()