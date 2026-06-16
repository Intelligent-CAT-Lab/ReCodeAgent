package testing

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/isaacphi/mcp-language-server/internal/protocol"
	"github.com/isaacphi/mcp-language-server/internal/watcher"
)

// TestGitignorePatterns specifically tests the gitignore pattern integration
func TestGitignorePatterns(t *testing.T) {
	if os.Getenv("GITHUB_ACTIONS") == "true" {
		t.Skip("Skipping filesystem watcher tests in GitHub Actions environment")
	}
	// Set up a test workspace in a temporary directory
	testDir, err := os.MkdirTemp("", "watcher-gitignore-patterns-*")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer func() {
		if err := os.RemoveAll(testDir); err != nil {
			t.Logf("Failed to remove test directory: %v", err)
		}
	}()

	// Create a .gitignore file with specific patterns
	gitignorePath := filepath.Join(testDir, ".gitignore")
	gitignoreContent := `# This is a test gitignore file
# Ignore files with .ignored extension
*.ignored

# Ignore specific directory
ignored_dir/

# Ignore a specific file
exact_file.txt

# Ignore files with a pattern
**/temp_*.log
`
	err = os.WriteFile(gitignorePath, []byte(gitignoreContent), 0644)
	if err != nil {
		t.Fatalf("Failed to write .gitignore: %v", err)
	}

	// Create a mock LSP client
	mockClient := NewMockLSPClient()

	// Create a watcher with default config
	testWatcher := watcher.NewWorkspaceWatcher(mockClient)

	// Register watchers for all files
	watchers := []protocol.FileSystemWatcher{
		{
			GlobPattern: protocol.GlobPattern{Value: "**/*"},
			Kind: func() *protocol.WatchKind {
				kind := protocol.WatchKind(protocol.WatchCreate | protocol.WatchChange | protocol.WatchDelete)
				return &kind
			}(),
		},
	}

	// Create a context with timeout
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// Start watching the workspace
	go testWatcher.WatchWorkspace(ctx, testDir)

	// Give the watcher time to initialize
	time.Sleep(500 * time.Millisecond)

	// Add watcher registrations
	testWatcher.AddRegistrations(ctx, "test-id", watchers)
	time.Sleep(500 * time.Millisecond)

	// Test file with ignored extension
	t.Run("IgnoredExtension", func(t *testing.T) {
		mockClient.ResetEvents()

		filePath := filepath.Join(testDir, "test.ignored")
		err := os.WriteFile(filePath, []byte("This file should be ignored by gitignore"), 0644)
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		time.Sleep(1 * time.Second)

		events := mockClient.GetEvents()
		if len(events) > 0 {
			t.Errorf("Received %d events for file %s which should be ignored by gitignore", len(events), filePath)
			for i, evt := range events {
				t.Logf("  Event %d: URI=%s, Type=%d", i, evt.URI, evt.Type)
			}
		}
	})

	// Test ignored directory
	t.Run("IgnoredDirectory", func(t *testing.T) {
		mockClient.ResetEvents()

		dirPath := filepath.Join(testDir, "ignored_dir")
		err := os.MkdirAll(dirPath, 0755)
		if err != nil {
			t.Fatalf("Failed to create directory: %v", err)
		}

		filePath := filepath.Join(dirPath, "file.txt")
		err = os.WriteFile(filePath, []byte("This file should be ignored due to gitignore dir pattern"), 0644)
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		time.Sleep(1 * time.Second)

		events := mockClient.GetEvents()
		if len(events) > 0 {
			t.Errorf("Received %d events for file in ignored directory %s", len(events), dirPath)
			for i, evt := range events {
				t.Logf("  Event %d: URI=%s, Type=%d", i, evt.URI, evt.Type)
			}
		}
	})

	// Test exact file match
	t.Run("ExactFileMatch", func(t *testing.T) {
		mockClient.ResetEvents()

		filePath := filepath.Join(testDir, "exact_file.txt")
		err := os.WriteFile(filePath, []byte("This file should be ignored by exact match in gitignore"), 0644)
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		time.Sleep(1 * time.Second)

		events := mockClient.GetEvents()
		if len(events) > 0 {
			t.Errorf("Received %d events for file %s which should be ignored by gitignore", len(events), filePath)
			for i, evt := range events {
				t.Logf("  Event %d: URI=%s, Type=%d", i, evt.URI, evt.Type)
			}
		}
	})

	// Test pattern match
	t.Run("PatternMatch", func(t *testing.T) {
		mockClient.ResetEvents()

		filePath := filepath.Join(testDir, "subdir")
		err := os.MkdirAll(filePath, 0755)
		if err != nil {
			t.Fatalf("Failed to create directory: %v", err)
		}

		filePath = filepath.Join(filePath, "temp_123.log")
		err = os.WriteFile(filePath, []byte("This file should be ignored by pattern match in gitignore"), 0644)
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		time.Sleep(1 * time.Second)

		events := mockClient.GetEvents()
		if len(events) > 0 {
			t.Errorf("Received %d events for file %s which should be ignored by gitignore", len(events), filePath)
			for i, evt := range events {
				t.Logf("  Event %d: URI=%s, Type=%d", i, evt.URI, evt.Type)
			}
		}
	})

	// Test non-ignored file
	t.Run("NonIgnoredFile", func(t *testing.T) {
		mockClient.ResetEvents()

		filePath := filepath.Join(testDir, "regular_file.txt")
		err := os.WriteFile(filePath, []byte("This file should NOT be ignored"), 0644)
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		waitCtx, waitCancel := context.WithTimeout(ctx, 2*time.Second)
		defer waitCancel()

		if !mockClient.WaitForEvent(waitCtx) {
			t.Fatal("Timed out waiting for file creation event")
		}

		uri := "file://" + filePath
		count := mockClient.CountEvents(uri, protocol.FileChangeType(protocol.Created))
		if count == 0 {
			t.Errorf("No create event received for non-ignored file %s", filePath)
		}
	})
}
