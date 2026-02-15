#!/usr/bin/env python3
import os
import re
import sys

CRATES = [...]  # same as before, but we'll import from refactor.py

# For simplicity, just copy the functions we need
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
    print(f'Used paths: {used_paths}')
    existing_imports = collect_existing_imports(content)
    print(f'Existing imports: {existing_imports}')
    imports_to_add = generate_import_statements(used_paths, existing_imports)
    print(f'Imports to add: {imports_to_add}')
    return imports_to_add

if __name__ == '__main__':
    CRATES = [
        "app_state",
        "git_info",
        # ... we can limit for test
    ]
    result = process_file('error_occurence_lib/src/code_occurence.rs')
    print(result)