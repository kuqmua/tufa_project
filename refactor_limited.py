#!/usr/bin/env python3
import os
import re
import sys

CRATES = [...]  # same as before, but we'll import from refactor.py

# Copy functions from refactor.py (simplified)
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

def process_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    used_paths = extract_used_paths(content)
    if not used_paths:
        return None
    existing_imports = collect_existing_imports(content)
    imports_to_add = generate_import_statements(used_paths, existing_imports)
    if not imports_to_add:
        return None
    # Add imports after the last existing use statement
    lines = content.split('\n')
    last_use_idx = -1
    for i, line in enumerate(lines):
        if line.strip().startswith('use '):
            last_use_idx = i
    if last_use_idx >= 0:
        insert_idx = last_use_idx + 1
        while insert_idx < len(lines) and lines[insert_idx].strip() == '':
            insert_idx += 1
        for imp in imports_to_add:
            lines.insert(insert_idx, f'use {imp};')
            insert_idx += 1
    else:
        insert_idx = 0
        while insert_idx < len(lines) and (lines[insert_idx].startswith('#') or lines[insert_idx].startswith('//') or lines[insert_idx].strip() == ''):
            insert_idx += 1
        for imp in imports_to_add:
            lines.insert(insert_idx, f'use {imp};')
            insert_idx += 1
    new_content = '\n'.join(lines)
    # Replace usages (simple)
    for path in used_paths:
        imp = path_to_import(path)
        if imp in imports_to_add:
            # Determine replacement
            if path.startswith(imp):
                suffix = path[len(imp):]
                if suffix.startswith('::'):
                    import_last = imp.split('::')[-1]
                    replacement = import_last + suffix
                    new_content = new_content.replace(path, replacement)
                else:
                    new_content = new_content.replace(path, imp.split('::')[-1])
    if new_content != content:
        return new_content
    return None

def main():
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
    # Process only a few files for testing
    test_files = [
        'impl_display_as_debug/src/lib.rs',
        'token_patterns/src/lib.rs',
        'server_app_state/src/lib.rs',
        'enum_extension_lib/enum_extension/src/lib.rs',
        'enum_extension_lib/src/lib.rs'
    ]
    for f in test_files:
        if os.path.exists(f):
            new = process_file(f)
            if new:
                # Write backup
                backup = f + '.bak'
                with open(backup, 'w', encoding='utf-8') as b:
                    with open(f, 'r', encoding='utf-8') as orig:
                        b.write(orig.read())
                with open(f, 'w', encoding='utf-8') as out:
                    out.write(new)
                print(f'Modified {f}')
            else:
                print(f'No changes needed for {f}')
        else:
            print(f'File {f} not found')

if __name__ == '__main__':
    main()