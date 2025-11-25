#!/usr/bin/env python3
"""
Rust → async-graphql struct generator from plain Rust-like syntax.

Example input (as a single string):

ExchangeInfo {
    timezone: String,
    serverTime: i64,
    symbols: Vec<Symbol>,
}

Symbol {
    symbol: String,
    baseAsset: String,
    quoteAsset: String,
    pricePrecision: i32,
    contractSize: i64,
    filters: Vec<SymbolFilter>,
}

SymbolFilter {
    filterType: String,
    minPrice: Option<String>,
    maxQty: Option<String>,
}
"""

import re
import sys
from textwrap import dedent

def parse_rust_structs(input_text: str):
    structs = {}
    current = None
    pattern = re.compile(r'^(\w+)\s*\{([^}]+)\}', re.MULTILINE)

    for match in pattern.finditer(input_text):
        name = match.group(1).strip()
        body = match.group(2)
        fields = {}
        for line in body.split(','):
            line = line.strip()
            if not line:
                continue
            if ':' not in line:
                continue
            field_part, type_part = line.split(':', 1)
            field_name = field_part.strip()
            type_str = type_part.strip().rstrip(',')
            fields[field_name] = type_str
        structs[name] = fields
        current = name
    return structs

def to_graphql_type(rust_type: str) -> str:
    mapping = {
        "String": "String",
        "i32": "i32",
        "i64": "i64",
        "f64": "f64",
        "bool": "bool",
    }
    if rust_type.startswith("Vec<"):
        inner = rust_type[4:-1]
        return f"Vec<{to_graphql_type(inner)}>"
    if rust_type.startswith("Option<"):
        inner = rust_type[7:-1]
        return f"Option<{to_graphql_type(inner)}>"
    return mapping.get(rust_type, rust_type)

def generate_async_graphql(structs: dict) -> str:
    lines = [
        "// Auto-generated async-graphql types",
        "use async_graphql::*;\n",
    ]

    for name, fields in structs.items():
        lines.append(f"#[derive(SimpleObject, Clone, Debug)]")
        lines.append(f"pub struct {name} {{")
        for field_name, rust_type in fields.items():
            gql_type = to_graphql_type(rust_type)
            rust_field = field_name
            if field_name in ["type", "async", "mut"]:  # reserved keywords
                rust_field = f"r#{field_name}"
            lines.append(f"    #[graphql(name = \"{field_name}\")]")
            lines.append(f"    pub {rust_field}: {gql_type},")
        lines.append("}\n")

    return "\n".join(lines)

def main(input_str: str):
    structs = parse_rust_structs(dedent(input_str))
    print(generate_async_graphql(structs))

# Example usage
if __name__ == "__main__":
    src = """
    ExchangeInfo {
        timezone: String,
        serverTime: i64,
        symbols: Vec<Symbol>,
    }
    
    Symbol {
        symbol: String,
        baseAsset: String,
        quoteAsset: String,
        pricePrecision: i32,
        contractSize: i64,
        filters: Vec<SymbolFilter>,
    }
    
    SymbolFilter {
        filterType: String,
        minPrice: Option<String>,
        maxQty: Option<String>,
    }
    """

    # if len(sys.argv) > 1:
    #     with open(sys.argv[1]) as f:
    #         src = f.read()
    # else:
    #     src = sys.stdin.read()

    main(src)