

#!/usr/bin/env python3
"""
xAI API CLI Tool
Secure, robust local client using typer and pydantic for validation.
Updated models list as of November 2025.
"""

import os
# import sys
from typing import Optional, List

import typer
from pydantic import BaseModel, ValidationError, field_validator
import httpx
import json

USAGE = """
export XAI_API_KEY=your_key_here
python xai_cli.py chat "Hello Grok!" --model grok-4-1-fast-reasoning
python xai_cli.py chat "Explain quantum computing" --temperature 0.9 --history chat.json
python xai_cli.py models
"""

app = typer.Typer(
    name="xai",
    help="xAI Grok API CLI - chat with Grok models locally",
    add_completion=False,
)

# --- Configuration & Models ---
class Message(BaseModel):
    role: str
    content: str

    @field_validator("role")
    @classmethod
    def role_must_be_valid(cls, v: str) -> str:
        if v not in {"user", "assistant", "system"}:
            raise ValueError("role must be user, assistant, or system")
        return v

class ChatRequest(BaseModel):
    model: str
    messages: List[Message]
    temperature: Optional[float] = 0.7
    max_tokens: Optional[int] = None
    stream: bool = False

    @field_validator("temperature")
    @classmethod
    def temperature_range(cls, v: float) -> float:
        if not (0.0 <= v <= 2.0):
            raise ValueError("temperature must be between 0.0 and 2.0")
        return v

    @field_validator("max_tokens")
    @classmethod
    def max_tokens_positive(cls, v: Optional[int]) -> Optional[int]:
        if v is not None and v <= 0:
            raise ValueError("max_tokens must be positive")
        return v

# --- Constants ---
BASE_URL = "https://api.x.ai/v1"
VALID_MODELS = {
    "grok-4-1-fast-reasoning",
    "grok-4-1-fast-non-reasoning",
    "grok-code-fast-1",
    "grok-4-fast-reasoning",
    "grok-4-fast-non-reasoning",
    "grok-4-0709",
    "grok-3-mini",
    "grok-3",
    "grok-2-vision-1212",
    "grok-2-image-1212"
}

# --- Helpers ---
def get_api_key() -> str:
    key = os.getenv("XAI_API_KEY")
    if not key:
        typer.echo("Error: XAI_API_KEY environment variable not set.", err=True)
        raise typer.Exit(code=1)
    return key

def get_client() -> httpx.Client:
    return httpx.Client(
        base_url=BASE_URL,
        headers={"Authorization": f"Bearer {get_api_key()}"},
        timeout=60.0,
    )

def load_history(file_path: str) -> List[Message]:
    if not os.path.exists(file_path):
        return []
    try:
        with open(file_path, "r") as f:
            data = json.load(f)
        return [Message(**msg) for msg in data]
    except (json.JSONDecodeError, KeyError, ValidationError) as e:
        typer.echo(f"Warning: Invalid history file {file_path}, starting fresh: {e}", err=True)
        return []

def save_history(file_path: str, messages: List[Message]) -> None:
    try:
        with open(file_path, "w") as f:
            json.dump([msg.model_dump() for msg in messages], f, indent=2)
    except Exception as e:
        typer.echo(f"Warning: Failed to save history: {e}", err=True)

# --- Commands ---
@app.command()
def chat(
    prompt: str = typer.Argument(..., help="Your message to Grok"),
    model: str = typer.Option("grok-4-1-fast-reasoning", "--model", "-m", help="Model name"),
    temperature: float = typer.Option(0.7, "--temperature", "-t", help="Sampling temperature"),
    max_tokens: Optional[int] = typer.Option(None, "--max-tokens", help="Maximum output tokens"),
    system: Optional[str] = typer.Option(None, "--system", "-s", help="System prompt"),
    history: Optional[str] = typer.Option(None, "--history", "-h", help="JSON file for conversation history"),
    stream: bool = typer.Option(False, "--stream", help="Stream response"),
):
    """Send a chat message to xAI Grok API."""
    if model not in VALID_MODELS:
        typer.echo(f"Invalid model. Choose from: {', '.join(sorted(VALID_MODELS))}", err=True)
        raise typer.Exit(code=1)

    messages: List[Message] = []
    if history:
        messages.extend(load_history(history))
    if system and not any(msg.role == "system" for msg in messages):
        messages.append(Message(role="system", content=system))
    messages.append(Message(role="user", content=prompt))

    try:
        request = ChatRequest(
            model=model,
            messages=messages,
            temperature=temperature,
            max_tokens=max_tokens,
            stream=stream,
        )
    except ValidationError as e:
        typer.echo(f"Validation error: {e}", err=True)
        raise typer.Exit(code=1)

    with get_client() as client:
        try:
            resp = client.post("/chat/completions", json=request.model_dump(exclude_none=True))
            resp.raise_for_status()
            data = resp.json()
            if stream:
                # Basic streaming support
                full_content = ""
                for chunk in data.get("choices", [{}])[0].get("delta", {}).get("content", ""):
                    content = chunk.get("content", "")
                    if content:
                        typer.echo(content, nl=False)
                        full_content += content
                typer.echo()
                content = full_content
            else:
                content = data["choices"][0]["message"]["content"]
            typer.echo(content.strip())
            
            # Append to history
            if history:
                assistant_msg = Message(role="assistant", content=content)
                messages.append(assistant_msg)
                save_history(history, messages)
        except httpx.HTTPStatusError as e:
            typer.echo(f"API error {e.response.status_code}: {e.response.text}", err=True)
            raise typer.Exit(code=1)
        except KeyError as e:
            typer.echo(f"Unexpected API response format: {e}", err=True)
            raise typer.Exit(code=1)
        except Exception as e:
            typer.echo(f"Unexpected error: {e}", err=True)
            raise typer.Exit(code=1)

@app.command()
def models():
    """List available xAI models via API."""
    with get_client() as client:
        try:
            resp = client.get("/models")
            resp.raise_for_status()
            data = resp.json()
            for m in sorted(data.get("data", []), key=lambda x: x["id"]):
                typer.echo(f"{m['id']}: context {m.get('context_length', 'N/A')}")
        except httpx.HTTPStatusError as e:
            typer.echo(f"API error {e.response.status_code}: {e.response.text}", err=True)
            raise typer.Exit(code=1)
        except Exception as e:
            typer.echo(f"Failed to fetch models: {e}", err=True)
            raise typer.Exit(code=1)

if __name__ == "__main__":
    app()