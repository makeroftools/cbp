#!/usr/bin/env python3
"""
Convert Binance Spot Algo API Rust client (like spot_algo_api.rs)
into async-graphql queries for Rust.

Generates a clean `spot_algo_queries.rs` file with proper async-graphql Object impl
for a root Query object containing SpotAlgoQueries.
"""

import typer
from pathlib import Path
import re

app = typer.Typer(
    name="rs2gql",
    help="Convert Binance Spot Algo Rust API → async-graphql queries",
    add_completion=False,
)

TEMPLATE = """    async fn {gql_name}(
        &self, ctx: &Context<'_>{args}
    ) -> FieldResult<{return_type}> {{
        let client = ctx.data::<Arc<BinanceClient>>().unwrap();
        let res = binance::spot_algo_api::{func_name}(
            &client.config,{call_args}
        ).await?;
        Ok(res)
    }}
"""

QUERY_IMPL_TEMPLATE = """#[derive(Default)]
pub struct Query;

#[async_graphql::Object]
impl Query {{
{fields}
}}

pub type Schema = async_graphql::Schema<{{Query, EmptyMutation, EmptySubscription}}>;

pub async fn create_schema(client: Arc<BinanceClient>) -> Schema {{
    Schema::build({{Query, EmptyMutation, EmptySubscription}}, Some(client)).finish()
}}
"""

def to_gql_name(rust_name: str) -> str:
    name = rust_name.removeprefix("sapi_v1_algo_spot_")
    name = re.sub(r"_(get|post|delete)$", "", name)
    return re.sub(r"_([a-z])", lambda m: m.group(1).upper(), name)

def rust_to_gql_type(t: str) -> str:
    mapping = {
        "i64": "i64",
        "i32": "i32",
        "f64": "f64",
        "f32": "f32",
        "&str": "String",
        "String": "String",
    }
    return mapping.get(t.strip("& "), t)

@app.command()
def convert(
    input_file: Path = typer.Argument(..., help="Path to spot_algo_api.rs file", exists=True, dir_okay=False),
    output_dir: Path = typer.Argument(..., help="Directory to write generated queries (e.g. src/graphql)"),
):
    """Generate async-graphql queries from Binance Spot Algo Rust API."""
    
    if not output_dir.exists():
        output_dir.mkdir(parents=True)
        typer.echo(f"Created directory: {output_dir}")

    content = input_file.read_text()
    lines = content.splitlines()

    fields = []
    func_pattern = re.compile(r'^pub async fn (\w+)\(.*\) -> .*Error<(\w+Error)>')

    for line in lines:
        match = func_pattern.search(line)
        if not match:
            continue

        func_name = match.group(1)

        # Find full function signature
        sig_start = next(i for i, l in enumerate(lines) if l.strip().startswith(f"pub async fn {func_name}("))
        signature = lines[sig_start]

        # Extract parameters
        params_part = signature.split("(", 1)[1].rsplit(")", 1)[0]
        params = [p.strip() for p in params_part.split(",") if p.strip() and "configuration:" not in p]

        required = []
        optional = []

        for param in params:
            if ":" not in param:
                continue
            name_type = param.split(":", 1)
            name = name_type[0].strip().lstrip("&")
            type_part = name_type[1].strip()

            if type_part.startswith("Option<"):
                clean_type = type_part[7:-1].strip("& ")
                optional.append((name, clean_type))
            else:
                clean_type = type_part.strip("& ")
                required.append((name, clean_type))

        gql_name = to_gql_name(func_name)
        return_type = func_name.replace("sapi_v1_algo_spot_", "SapiV1AlgoSpot").rsplit("_", 1)[0] + "200Response"

        args = []
        args += [f", {name}: {rust_to_gql_type(t)}" for name, t in required]
        args += [f", {name}: Option<{rust_to_gql_type(t)}>" for name, t in optional]
        args_str = "".join(args) if args else ""

        call_args = []
        for name, _ in required:
            call_args.append(f"{name}")
        for name, t in optional:
            default = 'None' if 'str' in t.lower() else 'None'
            call_args.append(f"{name}.as_ref().unwrap_or(&{default})")

        call_args_str = ",\n            ".join(call_args) if call_args else ""

        fields.append(TEMPLATE.format(
            gql_name=gql_name,
            args=args_str,
            return_type=return_type,
            func_name=func_name,
            call_args=call_args_str,
        ))

    output_file = output_dir / "spot_algo_queries.rs"
    final_code = QUERY_IMPL_TEMPLATE.format(fields="\n\n    ".join(fields))

    output_file.write_text(final_code)
    typer.echo(typer.style(f"Generated {len(fields)} GraphQL fields in root Query → ", fg=typer.colors.GREEN) +
               typer.style(str(output_file), fg=typer.colors.BRIGHT_CYAN, bold=True))

if __name__ == "__main__":
    app()