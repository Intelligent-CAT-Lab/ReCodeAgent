#!/usr/bin/env bash

# Batch runner for tool projects (crust, alphatrans, oxidizer, skel)
# Keeps up to MAX_CONCURRENT projects running; starts the next as soon as one finishes
# Usage: ./run_batch_projects.sh <base_dir> [tool_name] [agent_name]
#   base_dir:  path to repo root (contains data/, scripts/); required
#   tool_name: crust, alphatrans, oxidizer, or skel (default: crust)
#   agent_name: recodeagent, baseagent-concat, or baseagent-condensed (default: recodeagent)

set -e

# Parse arguments
if [ $# -lt 1 ]; then
    echo "Usage: $0 <base_dir> [tool_name] [agent_name]"
    echo "  base_dir:  path to repo root (contains data/, scripts/)"
    echo "  tool_name: crust, alphatrans, oxidizer, or skel (default: crust)"
    echo "  agent_name: recodeagent, baseagent-concat, etc. (default: recodeagent)"
    exit 1
fi

BASE_DIR="$(cd "$1" && pwd)"
TOOL_NAME="${2:-crust}"
AGENT_NAME="${3:-recodeagent}"

# Validate tool name
case "$TOOL_NAME" in
    crust|alphatrans|oxidizer|skel)
        ;;
    *)
        echo "Error: Invalid tool name '$TOOL_NAME'"
        echo "Valid tools: crust, alphatrans, oxidizer, skel"
        exit 1
        ;;
esac

# Validate agent name
case "$AGENT_NAME" in
    recodeagent|baseagent-concat|baseagent-condensed|noanalyzer|noplanning|novalidator)
        ;;
    *)
        echo "Error: Invalid agent name '$AGENT_NAME'"
        echo "Valid agents: recodeagent, baseagent-concat, baseagent-condensed"
        exit 1
        ;;
esac

# Map tool names to source and target languages
declare -A SOURCE_LANG
declare -A TARGET_LANG

SOURCE_LANG[crust]="c"
TARGET_LANG[crust]="rust"

SOURCE_LANG[alphatrans]="java"
TARGET_LANG[alphatrans]="python"

SOURCE_LANG[oxidizer]="go"
TARGET_LANG[oxidizer]="rust"

SOURCE_LANG[skel]="python"
TARGET_LANG[skel]="javascript"

# Set language variables
SOURCE_LANGUAGE="${SOURCE_LANG[$TOOL_NAME]}"
TARGET_LANGUAGE="${TARGET_LANG[$TOOL_NAME]}"

# Configuration (paths relative to user-provided base_dir)
PROJECTS_DIR="${BASE_DIR}/data/tool_projects/${TOOL_NAME}"
declare -A MAX_CONCURRENT_BY_TOOL=(
    [alphatrans]=4
    [skel]=4
    [oxidizer]=3
    [crust]=5
)
MAX_CONCURRENT="${MAX_CONCURRENT_BY_TOOL[$TOOL_NAME]}"
CHECK_INTERVAL=30  # seconds between completion checks

PRINTED_AGENT_NAME="${AGENT_NAME}"
if [[ "$AGENT_NAME" == "noplanning" || "$AGENT_NAME" == "noanalyzer" || "$AGENT_NAME" == "novalidator" ]]; then
    PRINTED_AGENT_NAME="recodeagent"
fi

# Set success/fail strings based on agent type
SUCCESS_STRING="$PRINTED_AGENT_NAME completed successfully"
FAIL_STRING="$PRINTED_AGENT_NAME failed"

MAX_WAIT_TIME=20000  # Maximum wait time per project in seconds (2 hours)

# Skip list - projects that are already completed (add project names here)
# Format: "tool_name:project_name" or just "project_name" (applies to all tools)
# Note: Skip lists are tool-specific, not agent-specific, so if you run the same
# project with a different agent, it will not be skipped automatically
SKIP_PROJECTS=(
    # Crust projects (c -> rust)
    # Add other tool projects here as needed
    # Format: "tool_name:project_name"
)

# Function to check if a project should be skipped
should_skip() {
    local project=$1
    for skip in "${SKIP_PROJECTS[@]}"; do
        # Check for tool-specific skip (format: "tool_name:project_name")
        if [[ "$skip" == "${TOOL_NAME}:${project}" ]]; then
            return 0  # Should skip
        fi
        # Check for global skip (format: "project_name" - applies to all tools)
        if [[ "$skip" == "$project" ]] && [[ "$skip" != *:* ]]; then
            return 0  # Should skip
        fi
    done
    return 1  # Should not skip
}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

# Function to check if a project is complete
# Returns: 0 = success, 1 = still running, 2 = container not running, 3 = failed (FAIL_STRING found)
check_project_complete() {
    local project=$1
    local container_name="${AGENT_NAME}.${TOOL_NAME}.${project}.${SOURCE_LANGUAGE}.${TARGET_LANGUAGE}"
    
    # Check if container exists and is running
    if ! docker ps --format '{{.Names}}' | grep -q "^${container_name}$"; then
        log_warning "Container $container_name is not running"
        return 2  # Container not running
    fi
    
    # Get the log content once
    local log_content
    log_content=$(docker exec "$container_name" cat logs/orchestrator.log 2>/dev/null || echo "")
    
    # Check for success string in logs (case-insensitive)
    if echo "$log_content" | grep -iq "$SUCCESS_STRING"; then
        return 0  # Success
    fi
    
    # Check for fail string in logs (case-insensitive)
    if echo "$log_content" | grep -iq "$FAIL_STRING"; then
        return 3  # Failed
    fi
    
    return 1  # Still running
}

# Function to start a project
start_project() {
    local project=$1
    log_info "Starting project: $project (agent: $AGENT_NAME)"

    cd "$BASE_DIR"
    if bash scripts/start_docker.sh "$AGENT_NAME" "$TOOL_NAME" "$project" "$SOURCE_LANGUAGE" "$TARGET_LANGUAGE"; then
        log_success "Project $project started successfully"
        return 0
    else
        log_error "Failed to start project $project"
        return 1
    fi
}

# Function to stop a project's Docker container
stop_project() {
    local project=$1
    local container_name="${AGENT_NAME}.${TOOL_NAME}.${project}.${SOURCE_LANGUAGE}.${TARGET_LANGUAGE}"
    
    log_info "Stopping container for project: $project"
    
    if docker ps --format '{{.Names}}' | grep -q "^${container_name}$"; then
        if docker stop "$container_name" >/dev/null 2>&1; then
            log_success "Container $container_name stopped successfully"
            return 0
        else
            log_error "Failed to stop container $container_name"
            return 1
        fi
    else
        log_warning "Container $container_name is not running (may have already stopped)"
        return 0
    fi
}

print_overall_status() {
    local total=$1
    local running_count=$2
    local completed_count=$3
    local failed_count=$4
    local elapsed=$5
    local running_projects=("${@:6}")

    local elapsed_min=$((elapsed / 60))
    local elapsed_sec=$((elapsed % 60))

    echo ""
    log_info "=== Status (elapsed: ${elapsed_min}m ${elapsed_sec}s) ==="
    log_info "Progress: ${completed_count}/${total} completed, ${failed_count} failed, ${running_count} running. Next check in ${CHECK_INTERVAL}s..."
    if [ ${#running_projects[@]} -gt 0 ]; then
        log_info "Running: ${running_projects[*]}"
    fi
}

# Main execution
main() {
    log_info "Starting batch project runner"
    log_info "Base directory: $BASE_DIR"
    log_info "Agent: $AGENT_NAME"
    log_info "Tool: $TOOL_NAME"
    log_info "Source language: $SOURCE_LANGUAGE -> Target language: $TARGET_LANGUAGE"
    log_info "Projects directory: $PROJECTS_DIR"
    log_info "Max concurrent: $MAX_CONCURRENT"
    
    if [ ! -d "$PROJECTS_DIR" ]; then
        log_error "Projects directory does not exist: $PROJECTS_DIR"
        exit 1
    fi
    
    # Get all project directories
    mapfile -t all_dirs < <(ls -1 "$PROJECTS_DIR" | sort)
    local total_dirs=${#all_dirs[@]}
    
    # Filter out skipped projects
    local all_projects=()
    local skipped_projects=()
    for project in "${all_dirs[@]}"; do
        if should_skip "$project"; then
            skipped_projects+=("$project")
        else
            all_projects+=("$project")
        fi
    done
    
    local total_projects=${#all_projects[@]}
    local total_skipped=${#skipped_projects[@]}
    
    log_info "Found $total_dirs total projects"
    log_info "Skipping $total_skipped already completed projects: ${skipped_projects[*]}"
    log_info "Will process $total_projects projects"
    
    # Track overall results
    local all_completed=()
    local all_failed=()

    # Rolling scheduler state
    declare -A project_start_time=()
    local running_projects=()
    local next_index=0
    local run_start_time
    run_start_time=$(date +%s)

    echo ""
    echo "=============================================="
    log_info "RUN: Processing ${total_projects} projects with max concurrency ${MAX_CONCURRENT}"
    echo "=============================================="

    while [ $((${#all_completed[@]} + ${#all_failed[@]})) -lt "$total_projects" ]; do
        local progress_made=0

        # Start new projects while we have capacity.
        while [ ${#running_projects[@]} -lt "$MAX_CONCURRENT" ] && [ "$next_index" -lt "$total_projects" ]; do
            local project="${all_projects[$next_index]}"
            next_index=$((next_index + 1))

            if start_project "$project"; then
                running_projects+=("$project")
                project_start_time["$project"]=$(date +%s)
                progress_made=1
            else
                all_failed+=("$project")
                progress_made=1
            fi

            # Small delay between starts to avoid overwhelming the system
            sleep 5
        done

        # Check running projects for completion/failure/timeout.
        local still_running=()
        local now
        now=$(date +%s)

        for project in "${running_projects[@]}"; do
            # With `set -e`, calling a function that returns non-zero would
            # terminate the script unless it's part of a conditional.
            local status
            if check_project_complete "$project"; then
                status=0
            else
                status=$?
            fi
            local started_at=${project_start_time["$project"]:-$run_start_time}
            local elapsed=$((now - started_at))

            if [ $status -eq 0 ]; then
                log_success "Project $project completed successfully!"
                stop_project "$project" || true
                all_completed+=("$project")
                progress_made=1
            elif [ $status -eq 3 ]; then
                log_error "Project $project failed (FAIL_STRING detected in logs)"
                stop_project "$project" || true
                all_failed+=("$project")
                progress_made=1
            elif [ $status -eq 2 ]; then
                log_error "Project $project container stopped unexpectedly"
                stop_project "$project" || true
                all_failed+=("$project")
                progress_made=1
            elif [ $elapsed -gt "$MAX_WAIT_TIME" ]; then
                log_error "Project $project timed out after $MAX_WAIT_TIME seconds"
                stop_project "$project" || true
                all_failed+=("$project")
                progress_made=1
            else
                still_running+=("$project")
            fi
        done

        running_projects=("${still_running[@]}")

        # If we made progress (freed slots), immediately loop to refill capacity.
        if [ "$progress_made" -eq 1 ]; then
            continue
        fi

        # Otherwise print status and sleep until next poll.
        local total_elapsed=$((now - run_start_time))
        print_overall_status "$total_projects" "${#running_projects[@]}" "${#all_completed[@]}" "${#all_failed[@]}" "$total_elapsed" "${running_projects[@]}"
        sleep "$CHECK_INTERVAL"
    done
    
    # Final summary
    echo ""
    echo "=============================================="
    log_info "ALL BATCHES COMPLETE"
    echo "=============================================="
    log_success "Total completed: ${#all_completed[@]}"
    log_error "Total failed: ${#all_failed[@]}"
    
    if [ ${#all_failed[@]} -gt 0 ]; then
        echo ""
        log_warning "Failed projects:"
        for project in "${all_failed[@]}"; do
            echo "  - $project"
        done
    fi
    
    # Write results to file
    local results_file="${BASE_DIR}/batch_results_${AGENT_NAME}_${TOOL_NAME}_$(date '+%Y%m%d_%H%M%S').txt"
    {
        echo "Batch Run Results - ${AGENT_NAME} - ${TOOL_NAME} (${SOURCE_LANGUAGE} -> ${TARGET_LANGUAGE}) - $(date)"
        echo "================================"
        echo "Agent: ${AGENT_NAME}"
        echo ""
        echo "Skipped (${#skipped_projects[@]}) - previously completed:"
        for project in "${skipped_projects[@]}"; do
            echo "  $project"
        done
        echo ""
        echo "Completed (${#all_completed[@]}):"
        for project in "${all_completed[@]}"; do
            echo "  $project"
        done
        echo ""
        echo "Failed (${#all_failed[@]}):"
        for project in "${all_failed[@]}"; do
            echo "  $project"
        done
    } > "$results_file"
    
    log_info "Results written to: $results_file"
}

# Run main
main "$@"
