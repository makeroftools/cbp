#!/usr/bin/env python3
"""
openapi-gen.py - Generate code from OpenAPI spec using openapi-generator Docker images

Usage:
    python openapi-gen.py cli <spec_url> [generator] [output_dir]
    python openapi-gen.py online <spec_url> [generator]

Modes:
    cli     - Fast generation using openapi-generator-cli (recommended)
    online  - Uses the full online image (returns a download link)

Arguments:
    spec_url     - URL or local file path to OpenAPI spec (yaml/json)
    generator    - Target generator name (default: python for cli, ruby for online)
    output_dir   - Output directory (cli mode only, default: out)

Examples:
    python openapi-gen.py cli https://example.com/openapi.json python ./client
    python openapi-gen.py online https://example.com/openapi.yaml java

Requirements: Docker, requests (pip install requests)
"""

import subprocess
import sys
import os
import time
import requests
import zipfile

SCHEMA_URL = "https://raw.githubusercontent.com/binance/binance-api-swagger/refs/heads/master/spot_api.yaml"

def run_cli(spec_url=SCHEMA_URL, generator='graphql-schema', output_dir='out'):
    cmd = [
        'docker', 'run', '--rm', '-v', f"{os.getcwd()}:/local",
        'openapitools/openapi-generator-cli', 'generate',
        '-i', spec_url,
        '-g', generator,
        '-o', f'/local/{output_dir}'
    ]
    subprocess.run(cmd, check=True)
    print(f"Generated in ./{output_dir}")

def run_online(spec_url, generator='ruby'):
    container = subprocess.check_output([
        'docker', 'run', '-d', '-p', '8888:8080',
        'openapitools/openapi-generator-online'
    ]).decode().strip()
    time.sleep(8)
    resp = requests.post(
        'http://localhost:8888/api/gen/clients/' + generator,
        json={'openAPIUrl': spec_url},
        headers={'Content-Type': 'application/json'}
    )
    resp.raise_for_status()
    data = resp.json()
    link = data['link']
    code = data['code']
    zip_path = f"{code}.zip"
    with open(zip_path, 'wb') as f:
        f.write(requests.get(link).content)
    with zipfile.ZipFile(zip_path) as z:
        z.extractall('.')
    os.remove(zip_path)
    subprocess.run(['docker', 'rm', '-f', container], shell=True)
    print("Generated and extracted")

if __name__ == '__main__':
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(1)
    mode, spec_url = sys.argv[1], sys.argv[2]
    generator = sys.argv[3] if len(sys.argv) > 3 else None
    output_dir = sys.argv[4] if len(sys.argv) > 4 else 'out'
    if mode == 'cli':
        run_cli(spec_url, generator or 'python', output_dir)
    elif mode == 'online':
        run_online(spec_url, generator or 'ruby')
    else:
        print("Mode must be 'cli' or 'online'")
        sys.exit(1)