#!/bin/bash
# Generated for {{ project_name }}

cd "{{ project_slug }}"
pixi install
git init
git add .
git commit -m "Initial commit: {{ project_name }} from copier template"
echo "{{ project_name }} setup complete. Run 'pixi run app' to test."