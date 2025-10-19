import json
import html
import sys
from datetime import datetime
from pathlib import Path


def parse_conversation(json_file_path):
    """Parse the conversation JSON file and extract messages."""
    with open(json_file_path, "r", encoding="utf-8") as f:
        content = f.read()

    # Split by lines and parse each JSON object
    messages = []
    for line in content.strip().split("\n"):
        if line.strip():
            try:
                msg_data = json.loads(line)
                messages.append(msg_data)
            except json.JSONDecodeError as e:
                print(f"Error parsing line: {e}")
                continue

    return messages


def format_timestamp(timestamp_str):
    """Format timestamp to readable format."""
    try:
        dt = datetime.fromisoformat(timestamp_str.replace("Z", "+00:00"))
        return dt.strftime("%Y-%m-%d %H:%M:%S UTC")
    except:
        return timestamp_str


def escape_html(text):
    """Escape HTML characters and preserve formatting."""
    if not text:
        return ""
    return html.escape(str(text)).replace("\n", "<br>")


def format_message_content(content):
    """Format message content, handling different content types."""
    if isinstance(content, str):
        return escape_html(content)
    elif isinstance(content, list):
        formatted_parts = []
        for item in content:
            if isinstance(item, dict):
                if item.get("type") == "text":
                    formatted_parts.append(escape_html(item.get("text", "")))
                elif item.get("type") == "tool_use":
                    tool_name = item.get("name", "Unknown Tool")
                    tool_input = json.dumps(item.get("input", {}), indent=2)
                    formatted_parts.append(
                        f"""
                    <div class="tool-use">
                        <strong>🔧 Tool Used: {tool_name}</strong>
                        <pre class="tool-input">{escape_html(tool_input)}</pre>
                    </div>
                    """
                    )
                elif item.get("type") == "tool_result":
                    result_content = item.get("content", "")
                    formatted_parts.append(
                        f"""
                    <div class="tool-result">
                        <strong>📋 Tool Result:</strong>
                        <pre class="tool-output">{escape_html(result_content)}</pre>
                    </div>
                    """
                    )
            else:
                formatted_parts.append(escape_html(str(item)))
        return "".join(formatted_parts)
    else:
        return escape_html(str(content))


def generate_html(messages, output_file="conversation.html"):
    """Generate HTML file from conversation messages."""

    # Create CSS as a separate string to avoid format conflicts
    css_styles = """
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        
        .header {
            background: linear-gradient(135deg, #2c3e50 0%, #3498db 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }
        
        .header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            font-weight: 300;
        }
        
        .header p {
            opacity: 0.9;
            font-size: 1.1em;
        }
        
        .conversation {
            padding: 20px;
            max-height: 80vh;
            overflow-y: auto;
        }
        
        .message {
            margin-bottom: 25px;
            border-radius: 12px;
            padding: 20px;
            position: relative;
            box-shadow: 0 4px 15px rgba(0,0,0,0.1);
            transition: transform 0.2s ease;
        }
        
        .message:hover {
            transform: translateY(-2px);
        }
        
        .message.user {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            margin-left: 10%;
        }
        
        .message.assistant {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            color: white;
            margin-right: 10%;
        }
        
        .message.external {
            background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
            color: white;
            margin-left: 5%;
            margin-right: 5%;
        }
        
        .message-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
            padding-bottom: 10px;
            border-bottom: 1px solid rgba(255,255,255,0.2);
        }
        
        .message-type {
            font-weight: bold;
            font-size: 1.1em;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        
        .timestamp {
            font-size: 0.9em;
            opacity: 0.8;
        }
        
        .message-content {
            font-size: 1.05em;
            line-height: 1.7;
        }
        
        .metadata {
            margin-top: 15px;
            padding-top: 15px;
            border-top: 1px solid rgba(255,255,255,0.2);
            font-size: 0.9em;
            opacity: 0.8;
        }
        
        .metadata-item {
            margin-bottom: 5px;
        }
        
        .tool-use, .tool-result {
            background: rgba(0,0,0,0.1);
            border-radius: 8px;
            padding: 15px;
            margin: 10px 0;
            border-left: 4px solid rgba(255,255,255,0.5);
        }
        
        .tool-input, .tool-output {
            background: rgba(0,0,0,0.2);
            padding: 10px;
            border-radius: 5px;
            font-family: 'Courier New', monospace;
            font-size: 0.9em;
            margin-top: 10px;
            white-space: pre-wrap;
            word-wrap: break-word;
        }
        
        .stats {
            background: #f8f9fa;
            padding: 20px;
            text-align: center;
            font-size: 0.9em;
            color: #666;
        }
        
        @media (max-width: 768px) {
            body {
                padding: 10px;
            }
            
            .message {
                margin-left: 0 !important;
                margin-right: 0 !important;
            }
            
            .header h1 {
                font-size: 2em;
            }
        }
        
        /* Custom scrollbar */
        .conversation::-webkit-scrollbar {
            width: 8px;
        }
        
        .conversation::-webkit-scrollbar-track {
            background: #f1f1f1;
            border-radius: 4px;
        }
        
        .conversation::-webkit-scrollbar-thumb {
            background: #888;
            border-radius: 4px;
        }
        
        .conversation::-webkit-scrollbar-thumb:hover {
            background: #555;
        }
    """

    # HTML template with placeholders for dynamic content
    html_template = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Conversation History</title>
    <style>{css_styles}</style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>💬 Conversation History</h1>
            <p>AI Assistant Conversation Log</p>
        </div>
        
        <div class="conversation">
            {messages}
        </div>
        
        <div class="stats">
            <strong>Total Messages:</strong> {total_messages} | 
            <strong>Generated:</strong> {generation_time}
        </div>
    </div>
</body>
</html>"""

    # Generate message HTML
    message_html = ""
    for msg in messages:
        message_type = msg.get("type", "unknown")
        user_type = msg.get("userType", "unknown")
        timestamp = format_timestamp(msg.get("timestamp", ""))
        uuid = msg.get("uuid", "")[:8]  # Short UUID for display

        # Determine message class based on type
        if message_type == "user":
            msg_class = "user"
            type_display = "👤 User"
        elif message_type == "assistant":
            msg_class = "assistant"
            type_display = "🤖 Assistant"
        else:
            msg_class = "external"
            type_display = f"🔧 {message_type.title()}"

        # Get message content
        if "message" in msg and isinstance(msg["message"], dict):
            content = msg["message"].get("content", "")
            role = msg["message"].get("role", "")
            if role:
                type_display += f" ({role})"
        else:
            content = msg.get("content", "")

        formatted_content = format_message_content(content)

        # Build metadata
        metadata_items = []
        if uuid:
            metadata_items.append(f"<div class='metadata-item'>🔗 ID: {uuid}</div>")
        if msg.get("sessionId"):
            session_id = msg["sessionId"][:8]
            metadata_items.append(f"<div class='metadata-item'>📝 Session: {session_id}</div>")
        if msg.get("version"):
            metadata_items.append(f"<div class='metadata-item'>⚙️ Version: {msg['version']}</div>")

        metadata_html = ""
        if metadata_items:
            metadata_html = f"<div class='metadata'>{''.join(metadata_items)}</div>"

        message_html += f"""
        <div class="message {msg_class}">
            <div class="message-header">
                <div class="message-type">{type_display}</div>
                <div class="timestamp">{timestamp}</div>
            </div>
            <div class="message-content">{formatted_content}</div>
            {metadata_html}
        </div>
        """

    # Fill in the template
    final_html = html_template.format(
        css_styles=css_styles,
        messages=message_html,
        total_messages=len(messages),
        generation_time=datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
    )

    # Write to file
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(final_html)

    return output_file


def main():
    """Main function to run the converter."""

    json_file_path = sys.argv[1]

    if not Path(json_file_path).exists():
        print(f"Error: File '{json_file_path}' not found.")
        return

    try:
        print(f"📖 Reading conversation from: {json_file_path}")
        messages = parse_conversation(json_file_path)
        print(f"✅ Parsed {len(messages)} messages")

        output_file = generate_html(messages)
        print(f"🎉 HTML file generated: {output_file}")
        print(f"🌐 Open '{output_file}' in your web browser to view the conversation")

    except Exception as e:
        print(f"❌ Error: {e}")


if __name__ == "__main__":
    main()
