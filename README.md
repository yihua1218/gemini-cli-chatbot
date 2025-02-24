# Gemini CLI Chatbot

A command-line interface chatbot powered by Google AI's Gemini 2.0 Pro Experimental 02-05 model.

## Setup

### Get Gemini API Key

1. Go to [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Create a new API key or use an existing one
3. Copy your API key

### Set API Key

You have multiple options to set your Gemini API key:

1. Set environment variable:

```bash
export GEMINI_API_KEY=your_api_key_here
```

2. Create a `.env` file in the project root:

```.env
GEMINI_API_KEY=your_api_key_here
```

3. Set it directly in your shell configuration file (e.g., `.bashrc`, `.zshrc`):

```bash
export GEMINI_API_KEY=your_api_key_here
```

4. Pass it as a command-line argument:

```bash
GEMINI_API_KEY=your_api_key_here gemini-cli-chatbot
```

## Usage

```bash
gemini-cli-chatbot --help
```

## Development

To debug in VSCode with the environment variable `GEMINI_API_KEY`:

1. Open the project in VSCode.
2. Go to the Run and Debug view (Ctrl+Shift+D).
3. Click on "create a launch.json file" if you don't have one.
4. Add the following configuration:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Launch Program",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/target/debug/gemini-cli-chatbot",
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "GEMINI_API_KEY": "your_api_key_here"
            },
            "sourceLanguages": ["rust"]
        }
    ]
}
```

5. Replace `"your_api_key_here"` with your actual API key.
6. Start debugging by clicking the green play button.

...
