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


def find_subagent_trajectories(json_file_path):
    """Find subagent trajectory files for a main session trajectory."""
    trajectory_path = Path(json_file_path)
    session_id = trajectory_path.stem
    subagents_dir = trajectory_path.parent / session_id / "subagents"

    if not subagents_dir.is_dir():
        return []

    subagent_files = sorted(subagents_dir.glob("*.jsonl"))
    trajectories = []
    for subagent_file in subagent_files:
        trajectories.append(
            {
                "title": f"Subagent: {subagent_file.stem}",
                "path": str(subagent_file),
            }
        )
    return trajectories


def render_messages_html(messages):
    """Render conversation messages into HTML fragments."""
    message_html = ""
    for msg in messages:
        message_type = msg.get("type", "unknown")
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
        if msg.get("agentId"):
            metadata_items.append(f"<div class='metadata-item'>🧩 Agent: {msg['agentId']}</div>")
        if msg.get("slug"):
            metadata_items.append(f"<div class='metadata-item'>🏷️ Slug: {msg['slug']}</div>")
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

    return message_html


def generate_html(trajectories, output_file="conversation.html"):
    """Generate HTML file from one or more conversation trajectories."""

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

        .toc {
            background: #f8f9fa;
            padding: 20px 30px;
            border-bottom: 1px solid #e9ecef;
        }

        .toc h2 {
            font-size: 1.2em;
            margin-bottom: 12px;
            color: #2c3e50;
        }

        .toc ul {
            list-style: none;
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
        }

        .toc a {
            display: inline-block;
            padding: 8px 14px;
            background: white;
            border: 1px solid #dee2e6;
            border-radius: 20px;
            color: #2c3e50;
            text-decoration: none;
            font-size: 0.95em;
        }

        .toc a:hover {
            background: #3498db;
            color: white;
            border-color: #3498db;
        }

        .trajectory-section {
            border-bottom: 1px solid #e9ecef;
        }

        .trajectory-section:last-child {
            border-bottom: none;
        }

        .section-header {
            background: #f1f3f5;
            padding: 20px 30px;
            border-bottom: 1px solid #dee2e6;
        }

        .section-header.main {
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            color: white;
        }

        .section-header.subagent {
            background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
            color: white;
        }

        .section-header h2 {
            font-size: 1.5em;
            font-weight: 500;
            margin-bottom: 6px;
        }

        .section-header p {
            opacity: 0.9;
            font-size: 0.95em;
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

    total_messages = sum(len(trajectory["messages"]) for trajectory in trajectories)
    subagent_count = sum(1 for trajectory in trajectories if trajectory.get("is_subagent"))

    toc_items = []
    sections_html = ""
    for index, trajectory in enumerate(trajectories):
        section_id = f"trajectory-{index}"
        title = trajectory["title"]
        message_count = len(trajectory["messages"])
        source_path = trajectory.get("path", "")
        is_subagent = trajectory.get("is_subagent", False)

        toc_items.append(
            f'<li><a href="#{section_id}">{html.escape(title)} ({message_count})</a></li>'
        )

        section_class = "subagent" if is_subagent else "main"
        subtitle = html.escape(source_path) if source_path else ""
        sections_html += f"""
        <section class="trajectory-section" id="{section_id}">
            <div class="section-header {section_class}">
                <h2>{html.escape(title)}</h2>
                <p>{message_count} messages{f" · {subtitle}" if subtitle else ""}</p>
            </div>
            <div class="conversation">
                {render_messages_html(trajectory["messages"])}
            </div>
        </section>
        """

    toc_html = ""
    if len(trajectories) > 1:
        toc_html = f"""
        <div class="toc">
            <h2>Trajectories</h2>
            <ul>
                {"".join(toc_items)}
            </ul>
        </div>
        """

    page_title = trajectories[0]["title"] if trajectories else "Conversation History"
    header_subtitle = "AI Assistant Conversation Log"
    if subagent_count:
        header_subtitle += f" · {subagent_count} subagent trajectory"
        if subagent_count != 1:
            header_subtitle += "ies"

    # HTML template with placeholders for dynamic content
    html_template = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{page_title}</title>
    <style>{css_styles}</style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>💬 Conversation History</h1>
            <p>{header_subtitle}</p>
        </div>

        {toc_html}

        {sections_html}
        
        <div class="stats">
            <strong>Total Messages:</strong> {total_messages} |
            <strong>Trajectories:</strong> {trajectory_count} |
            <strong>Generated:</strong> {generation_time}
        </div>
    </div>
</body>
</html>"""

    # Fill in the template
    final_html = html_template.format(
        css_styles=css_styles,
        page_title=html.escape(page_title),
        header_subtitle=header_subtitle,
        toc_html=toc_html,
        sections_html=sections_html,
        total_messages=total_messages,
        trajectory_count=len(trajectories),
        generation_time=datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
    )

    # Write to file
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(final_html)

    return output_file


def load_trajectory(json_file_path, title=None, is_subagent=False):
    """Load a trajectory file and return its metadata."""
    trajectory_path = Path(json_file_path)
    messages = parse_conversation(json_file_path)
    return {
        "title": title or f"Main Session: {trajectory_path.stem}",
        "path": str(trajectory_path),
        "messages": messages,
        "is_subagent": is_subagent,
    }


def main():
    """Main function to run the converter."""

    if len(sys.argv) < 2:
        print("Usage: python visualize_trajectory.py <trajectory.jsonl> [output.html]")
        return

    json_file_path = sys.argv[1]
    trajectory_path = Path(json_file_path)

    if not trajectory_path.exists():
        print(f"Error: File '{json_file_path}' not found.")
        return

    try:
        trajectories = []

        print(f"📖 Reading main conversation from: {json_file_path}")
        main_trajectory = load_trajectory(json_file_path)
        trajectories.append(main_trajectory)
        print(f"✅ Parsed {len(main_trajectory['messages'])} main messages")

        subagent_specs = find_subagent_trajectories(json_file_path)
        if subagent_specs:
            print(f"🔍 Found {len(subagent_specs)} subagent trajectories")
            for subagent_spec in subagent_specs:
                print(f"   📖 Reading subagent: {subagent_spec['path']}")
                subagent_trajectory = load_trajectory(
                    subagent_spec["path"],
                    title=subagent_spec["title"],
                    is_subagent=True,
                )
                trajectories.append(subagent_trajectory)
                print(
                    f"   ✅ Parsed {len(subagent_trajectory['messages'])} messages "
                    f"for {subagent_spec['title']}"
                )
        else:
            print("ℹ️ No subagent trajectories found for this session")

        if len(sys.argv) > 2:
            output_file = sys.argv[2]
        else:
            output_file = str(trajectory_path.with_suffix(".html"))

        output_path = generate_html(trajectories, output_file=output_file)
        print(f"🎉 HTML file generated: {output_path}")
        print(f"🌐 Open '{output_path}' in your web browser to view the conversation")

    except Exception as e:
        print(f"❌ Error: {e}")


if __name__ == "__main__":
    main()
