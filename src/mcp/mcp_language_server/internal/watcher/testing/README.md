# Workspace Watcher Testing

This package contains tests for the `WorkspaceWatcher` component. The tests use a real filesystem and a mock LSP client to verify that the watcher correctly detects and reports file events.

## Test Suite Overview

The test suite consists of the following tests:

### 1. Basic Functionality Tests
- Tests file creation, modification, and deletion events
- Confirms that appropriate notifications are sent to the LSP client
- Verifies that each operation triggers the correct event type (Created, Changed, Deleted)

### 2. Exclusion Pattern Tests
- Tests that files matching exclusion patterns are not reported
- Specifically tests:
  - Files with excluded extensions (.tmp)
  - Files ending with tilde (~)
  - Files in excluded directories (.git)
  - Files matching gitignore patterns

### 3. Debouncing Tests
- Tests that rapid changes to the same file result in a single notification
- Verifies the debouncing mechanism works correctly

## Mock LSP Client

The `MockLSPClient` implements the `watcher.LSPClient` interface and provides functionality for:
- Recording file events
- Testing if files are open
- Opening files
- Notifying about file changes
- Waiting for events with a timeout

## Running the Tests

To run the tests:

```bash
go test -v ./internal/watcher/testing
```

For more detailed output, enable debug logging:

```bash
go test -v -tags debug ./internal/watcher/testing
```

## Known Issues and Limitations

1. Gitignore Integration:
   - The watcher uses the go-gitignore package to parse and match gitignore patterns.
   - The tests verify that files matching gitignore patterns are excluded from notifications.
   - Additional tests in gitignore_test.go verify more complex patterns and matching scenarios.

2. File Deletion in Excluded Directories:
   - Since excluded directories are not watched, file deletion events in these directories are not detected.

3. Large Binary Files:
   - The tests don't verify the handling of large binary files due to test resource limitations.