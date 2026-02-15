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
    """Return set of full crate paths found in content (outside quote blocks)."""
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
    """Return set of imported paths (as strings) from use statements."""
    imports = set()
    # naive regex for use statements (single line)
    use_re = r'^\s*use\s+([^;]+);'
    for line in content.split('\n'):
        m = re.match(use_re, line.strip())
        if m:
            imports.add(m.group(1))
    return imports

def path_to_import(path):
    """Convert a usage path to an import statement.
    e.g., app_state::SourcePlaceType::from_env_or_default -> app_state::SourcePlaceType
    We'll strip trailing ::method or ::variant."""
    # Split by ::
    parts = path.split('::')
    # Keep removing trailing parts that start with lowercase (method) or are not uppercase?
    # Heuristic: keep until we encounter a part that is ALL_CAPS (constant) or CamelCase (type)
    # For simplicity, just keep first two parts if there are more than two.
    if len(parts) >= 2:
        # Check if second part is uppercase (type) or not
        # We'll just assume the type is the first two parts.
        import_path = '::'.join(parts[:2])
        # But there are cases like git_info::PROJECT_GIT_INFO (constant) where second part is all caps.
        # That's fine.
        return import_path
    else:
        return path

def generate_import_statements(used_paths, existing_imports):
    """Return list of import statements to add."""
    imports = set()
    for path in used_paths:
        imp = path_to_import(path)
        # Check if already imported
        if imp in existing_imports:
            continue
        # Also check if there is a broader import that covers it (e.g., use app_state::*)
        # Not implemented.
        imports.add(imp)
    return sorted(imports)

def replace_usages(content, used_paths, imports_added):
    """Replace each usage path with its shortened form based on imports."""
    # Build mapping from full path to replacement
    mapping = {}
    for path in used_paths:
        imp = path_to_import(path)
        if imp in imports_added:
            # Determine replacement: if the import is exactly the path, replace with last segment
            # else keep the suffix after import.
            if path.startswith(imp):
                suffix = path[len(imp):]
                if suffix.startswith('::'):
                    # The import is a prefix, we need to keep the suffix but replace the prefix with last segment of import?
                    # Actually we want to replace the whole path with import_last_segment + suffix
                    import_last = imp.split('::')[-1]
                    replacement = import_last + suffix
                    mapping[path] = replacement
                else:
                    # path == imp
                    mapping[path] = imp.split('::')[-1]
            else:
                # shouldn't happen
                pass
    # Apply replacements (careful to avoid overlapping)
    # Sort by length descending to avoid partial replacements
    sorted_paths = sorted(mapping.keys(), key=lambda x: len(x), reverse=True)
    for old in sorted_paths:
        new = mapping[old]
        content = content.replace(old, new)
    return content

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
    new_lines = []
    last_use_idx = -1
    for i, line in enumerate(lines):
        if line.strip().startswith('use '):
            last_use_idx = i
    # Insert after the last use line
    if last_use_idx >= 0:
        # Ensure there is an empty line after uses? We'll just insert after.
        insert_idx = last_use_idx + 1
        # But if there is a blank line after, we can keep it.
        while insert_idx < len(lines) and lines[insert_idx].strip() == '':
            insert_idx += 1
        # Insert imports
        for imp in imports_to_add:
            lines.insert(insert_idx, f'use {imp};')
            insert_idx += 1
    else:
        # No existing use statements, insert after any leading comments/module attributes?
        # For simplicity, insert after first line that is not a comment or attribute.
        insert_idx = 0
        while insert_idx < len(lines) and (lines[insert_idx].startswith('#') or lines[insert_idx].startswith('//') or lines[insert_idx].strip() == ''):
            insert_idx += 1
        for imp in imports_to_add:
            lines.insert(insert_idx, f'use {imp};')
            insert_idx += 1
    new_content = '\n'.join(lines)
    # Now replace usages
    new_content = replace_usages(new_content, used_paths, set(imports_to_add))
    if new_content != content:
        return new_content
    return None

def main():
    root = '.'
    changed_files = []
    for dirpath, dirnames, filenames in os.walk(root):
        if '.git' in dirpath:
            continue
        for f in filenames:
            if f.endswith('.rs'):
                full = os.path.join(dirpath, f)
                new_content = process_file(full)
                if new_content is not None:
                    # Backup original
                    backup = full + '.bak'
                    with open(backup, 'w', encoding='utf-8') as b:
                        with open(full, 'r', encoding='utf-8') as orig:
                            b.write(orig.read())
                    # Write new content
                    with open(full, 'w', encoding='utf-8') as out:
                        out.write(new_content)
                    changed_files.append(full)
                    print(f'Modified {full}')
    print(f'Total files changed: {len(changed_files)}')
    if changed_files:
        print('Backup files created with .bak extension')

if __name__ == '__main__':
    main()