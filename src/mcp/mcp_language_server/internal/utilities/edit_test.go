package utilities

import (
	"errors"
	"os"
	"reflect"
	"strings"
	"testing"
	"time"

	"github.com/isaacphi/mcp-language-server/internal/protocol"
)

// mockFileSystem provides mocked file system operations
type mockFileSystem struct {
	files     map[string][]byte
	fileStats map[string]os.FileInfo
	errors    map[string]error
}

// Setup mock file system functions
func setupMockFileSystem(_ *testing.T, mfs *mockFileSystem) func() {
	// Save original functions
	originalReadFile := osReadFile
	originalWriteFile := osWriteFile
	originalStat := osStat
	originalRemove := osRemove
	originalRemoveAll := osRemoveAll
	originalRename := osRename

	// Replace with mocks
	osReadFile = func(filename string) ([]byte, error) {
		if err, ok := mfs.errors[filename+"_read"]; ok {
			return nil, err
		}
		if content, ok := mfs.files[filename]; ok {
			return content, nil
		}
		return nil, os.ErrNotExist
	}

	osWriteFile = func(filename string, data []byte, perm os.FileMode) error {
		if err, ok := mfs.errors[filename+"_write"]; ok {
			return err
		}
		if mfs.files == nil {
			mfs.files = make(map[string][]byte)
		}
		mfs.files[filename] = data
		return nil
	}

	osStat = func(name string) (os.FileInfo, error) {
		if err, ok := mfs.errors[name+"_stat"]; ok {
			return nil, err
		}
		if info, ok := mfs.fileStats[name]; ok {
			return info, nil
		}
		return nil, os.ErrNotExist
	}

	osRemove = func(name string) error {
		if err, ok := mfs.errors[name+"_remove"]; ok {
			return err
		}
		if _, ok := mfs.files[name]; ok {
			delete(mfs.files, name)
			return nil
		}
		return os.ErrNotExist
	}

	osRemoveAll = func(path string) error {
		if err, ok := mfs.errors[path+"_removeall"]; ok {
			return err
		}
		// Remove any file that starts with this path
		for k := range mfs.files {
			if k == path || (len(k) > len(path) && k[:len(path)] == path) {
				delete(mfs.files, k)
			}
		}
		return nil
	}

	osRename = func(oldpath, newpath string) error {
		if err, ok := mfs.errors[oldpath+"_rename"]; ok {
			return err
		}
		if content, ok := mfs.files[oldpath]; ok {
			mfs.files[newpath] = content
			delete(mfs.files, oldpath)
			return nil
		}
		return os.ErrNotExist
	}

	// Return cleanup function
	return func() {
		osReadFile = originalReadFile
		osWriteFile = originalWriteFile
		osStat = originalStat
		osRemove = originalRemove
		osRemoveAll = originalRemoveAll
		osRename = originalRename
	}
}

// The os function variables are defined in edit.go

// Mock FileInfo implementation
type mockFileInfo struct {
	name    string
	size    int64
	mode    os.FileMode
	modTime int64
	isDir   bool
}

func (m mockFileInfo) Name() string       { return m.name }
func (m mockFileInfo) Size() int64        { return m.size }
func (m mockFileInfo) Mode() os.FileMode  { return m.mode }
func (m mockFileInfo) ModTime() time.Time { return time.Unix(m.modTime, 0) }
func (m mockFileInfo) IsDir() bool        { return m.isDir }
func (m mockFileInfo) Sys() any           { return nil }

func TestRangesOverlap(t *testing.T) {
	tests := []struct {
		name     string
		range1   protocol.Range
		range2   protocol.Range
		expected bool
	}{
		{
			name: "No overlap - completely different lines",
			range1: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 2, Character: 10},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 3, Character: 0},
				End:   protocol.Position{Line: 4, Character: 10},
			},
			expected: false,
		},
		{
			name: "No overlap - same start line but no character overlap",
			range1: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 1, Character: 5},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 6},
				End:   protocol.Position{Line: 1, Character: 10},
			},
			expected: false,
		},
		{
			name: "No overlap - same end line but no character overlap",
			range1: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 2, Character: 5},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 2, Character: 6},
				End:   protocol.Position{Line: 3, Character: 10},
			},
			expected: false,
		},
		{
			name: "Overlap - start of range1 inside range2",
			range1: protocol.Range{
				Start: protocol.Position{Line: 2, Character: 5},
				End:   protocol.Position{Line: 3, Character: 10},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 2, Character: 10},
			},
			expected: true,
		},
		{
			name: "Overlap - end of range1 inside range2",
			range1: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 2, Character: 5},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 2, Character: 0},
				End:   protocol.Position{Line: 3, Character: 10},
			},
			expected: true,
		},
		{
			name: "Overlap - range2 completely inside range1",
			range1: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 5, Character: 10},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 2, Character: 5},
				End:   protocol.Position{Line: 3, Character: 5},
			},
			expected: true,
		},
		{
			name: "Overlap - range1 completely inside range2",
			range1: protocol.Range{
				Start: protocol.Position{Line: 2, Character: 5},
				End:   protocol.Position{Line: 3, Character: 5},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 5, Character: 10},
			},
			expected: true,
		},
		{
			name: "Overlap - exact same range",
			range1: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 2, Character: 10},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 2, Character: 10},
			},
			expected: true,
		},
		{
			name: "Edge case - ranges touch at line boundary",
			range1: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 2, Character: 0},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 2, Character: 0},
				End:   protocol.Position{Line: 3, Character: 0},
			},
			expected: true, // Touching at a boundary counts as overlap
		},
		{
			name: "Edge case - ranges touch at character boundary",
			range1: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 0},
				End:   protocol.Position{Line: 1, Character: 5},
			},
			range2: protocol.Range{
				Start: protocol.Position{Line: 1, Character: 5},
				End:   protocol.Position{Line: 1, Character: 10},
			},
			expected: true, // Touching at a boundary counts as overlap
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := RangesOverlap(tt.range1, tt.range2)
			if result != tt.expected {
				t.Errorf("RangesOverlap() = %v, want %v", result, tt.expected)
			}
		})
	}
}

func TestApplyTextEdit(t *testing.T) {
	tests := []struct {
		name       string
		lines      []string
		edit       protocol.TextEdit
		lineEnding string
		expected   []string
		expectErr  bool
	}{
		{
			name:  "Delete text - single line",
			lines: []string{"This is a test line"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 5},
					End:   protocol.Position{Line: 0, Character: 9},
				},
				NewText: "",
			},
			lineEnding: "\n",
			expected:   []string{"This  test line"},
			expectErr:  false,
		},
		{
			name:  "Replace text - single line",
			lines: []string{"This is a test line"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 5},
					End:   protocol.Position{Line: 0, Character: 9},
				},
				NewText: "was",
			},
			lineEnding: "\n",
			expected:   []string{"This was test line"},
			expectErr:  false,
		},
		{
			name:  "Insert text - single line",
			lines: []string{"This is a test line"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 5},
					End:   protocol.Position{Line: 0, Character: 5},
				},
				NewText: "really ",
			},
			lineEnding: "\n",
			expected:   []string{"This really is a test line"},
			expectErr:  false,
		},
		{
			name:  "Delete text - multi-line",
			lines: []string{"Line 1", "Line 2", "Line 3"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 2},
					End:   protocol.Position{Line: 2, Character: 2},
				},
				NewText: "",
			},
			lineEnding: "\n",
			expected:   []string{"Line 3"},
			expectErr:  false,
		},
		{
			name:  "Replace text - multi-line",
			lines: []string{"Line 1", "Line 2", "Line 3"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 2},
					End:   protocol.Position{Line: 2, Character: 2},
				},
				NewText: "updated content",
			},
			lineEnding: "\n",
			expected:   []string{"Liupdated contentne 3"},
			expectErr:  false,
		},
		{
			name:  "Replace text with multi-line content",
			lines: []string{"Line 1", "Line 2", "Line 3"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 2},
					End:   protocol.Position{Line: 2, Character: 2},
				},
				NewText: "new\ntext\ncontent",
			},
			lineEnding: "\n",
			expected:   []string{"Linew", "text", "contentne 3"},
			expectErr:  false,
		},
		{
			name:  "Invalid start line",
			lines: []string{"Line 1", "Line 2", "Line 3"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 5, Character: 0},
					End:   protocol.Position{Line: 6, Character: 0},
				},
				NewText: "newtext",
			},
			lineEnding: "\n",
			expected:   nil,
			expectErr:  true,
		},
		{
			name:  "End line beyond file - should default to last line",
			lines: []string{"Line 1", "Line 2", "Line 3"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 1, Character: 2},
					End:   protocol.Position{Line: 5, Character: 0},
				},
				NewText: "newtext",
			},
			lineEnding: "\n",
			expected:   []string{"Line 1", "LinewtextLine 3"},
			expectErr:  false,
		},
		{
			name:  "Start character beyond line length",
			lines: []string{"Line 1", "Line 2", "Line 3"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 20},
					End:   protocol.Position{Line: 1, Character: 2},
				},
				NewText: "newtext",
			},
			lineEnding: "\n",
			expected:   []string{"Line 1newtextne 2", "Line 3"},
			expectErr:  false,
		},
		{
			name:  "End character beyond line length",
			lines: []string{"Line 1", "Line 2", "Line 3"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 2},
					End:   protocol.Position{Line: 1, Character: 20},
				},
				NewText: "newtext",
			},
			lineEnding: "\n",
			expected:   []string{"Linewtext", "Line 3"},
			expectErr:  false,
		},
		{
			name:  "Empty file - first insertion",
			lines: []string{""},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 0},
					End:   protocol.Position{Line: 0, Character: 0},
				},
				NewText: "New content",
			},
			lineEnding: "\n",
			expected:   []string{"New content"},
			expectErr:  false,
		},
		{
			name:  "Replace entire file with empty content",
			lines: []string{"Line 1", "Line 2", "Line 3"},
			edit: protocol.TextEdit{
				Range: protocol.Range{
					Start: protocol.Position{Line: 0, Character: 0},
					End:   protocol.Position{Line: 2, Character: 6},
				},
				NewText: "",
			},
			lineEnding: "\n",
			expected:   []string{},
			expectErr:  false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := ApplyTextEdit(tt.lines, tt.edit, tt.lineEnding)
			if tt.expectErr {
				if err == nil {
					t.Errorf("Expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v", err)
				} else if !reflect.DeepEqual(result, tt.expected) {
					t.Errorf("applyTextEdit() result = %v, want %v", result, tt.expected)
				}
			}
		})
	}
}

func TestApplyTextEdits(t *testing.T) {
	tests := []struct {
		name       string
		uri        protocol.DocumentUri
		content    string
		edits      []protocol.TextEdit
		expected   string
		expectErr  bool
		setupMocks func(*mockFileSystem)
	}{
		{
			name:    "Single edit - replace text",
			uri:     "file:///test/file.txt",
			content: "This is a test line",
			edits: []protocol.TextEdit{
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 0, Character: 5},
						End:   protocol.Position{Line: 0, Character: 9},
					},
					NewText: "was",
				},
			},
			expected:  "This was test line",
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/file.txt": []byte("This is a test line"),
				}
			},
		},
		{
			name:    "Multiple edits - non-overlapping",
			uri:     "file:///test/file.txt",
			content: "This is a test line",
			edits: []protocol.TextEdit{
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 0, Character: 5},
						End:   protocol.Position{Line: 0, Character: 7},
					},
					NewText: "was",
				},
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 0, Character: 10},
						End:   protocol.Position{Line: 0, Character: 14},
					},
					NewText: "sample",
				},
			},
			expected:  "This was a sample line",
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/file.txt": []byte("This is a test line"),
				}
			},
		},
		{
			name:    "CRLF line endings",
			uri:     "file:///test/file.txt",
			content: "Line 1\r\nLine 2\r\nLine 3",
			edits: []protocol.TextEdit{
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 1, Character: 0},
						End:   protocol.Position{Line: 1, Character: 6},
					},
					NewText: "Modified",
				},
			},
			expected:  "Line 1\r\nModified\r\nLine 3",
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/file.txt": []byte("Line 1\r\nLine 2\r\nLine 3"),
				}
			},
		},
		{
			name:    "Overlapping edits",
			uri:     "file:///test/file.txt",
			content: "This is a test line",
			edits: []protocol.TextEdit{
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 0, Character: 5},
						End:   protocol.Position{Line: 0, Character: 9},
					},
					NewText: "was",
				},
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 0, Character: 8},
						End:   protocol.Position{Line: 0, Character: 14},
					},
					NewText: "sample",
				},
			},
			expected:  "",
			expectErr: true,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/file.txt": []byte("This is a test line"),
				}
			},
		},
		{
			name:    "File with final newline",
			uri:     "file:///test/file.txt",
			content: "Line 1\nLine 2\n",
			edits: []protocol.TextEdit{
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 1, Character: 0},
						End:   protocol.Position{Line: 1, Character: 6},
					},
					NewText: "Modified",
				},
			},
			expected:  "Line 1\nModified\n",
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/file.txt": []byte("Line 1\nLine 2\n"),
				}
			},
		},
		{
			name:    "Error reading file",
			uri:     "file:///test/file.txt",
			content: "",
			edits: []protocol.TextEdit{
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 0, Character: 0},
						End:   protocol.Position{Line: 0, Character: 0},
					},
					NewText: "New content",
				},
			},
			expected:  "",
			expectErr: true,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.errors = map[string]error{
					"/test/file.txt_read": errors.New("read error"),
				}
			},
		},
		{
			name:    "Error writing file",
			uri:     "file:///test/file.txt",
			content: "This is a test line",
			edits: []protocol.TextEdit{
				{
					Range: protocol.Range{
						Start: protocol.Position{Line: 0, Character: 5},
						End:   protocol.Position{Line: 0, Character: 9},
					},
					NewText: "was",
				},
			},
			expected:  "",
			expectErr: true,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/file.txt": []byte("This is a test line"),
				}
				mfs.errors = map[string]error{
					"/test/file.txt_write": errors.New("write error"),
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mfs := &mockFileSystem{}
			tt.setupMocks(mfs)
			cleanup := setupMockFileSystem(t, mfs)
			defer cleanup()

			err := ApplyTextEdits(tt.uri, tt.edits)
			if tt.expectErr {
				if err == nil {
					t.Errorf("Expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v", err)
				} else {
					path := strings.TrimPrefix(string(tt.uri), "file://")
					if content, ok := mfs.files[path]; ok {
						if string(content) != tt.expected {
							t.Errorf("applyTextEdits() result = %q, want %q", string(content), tt.expected)
						}
					} else {
						t.Errorf("File not found in mock file system")
					}
				}
			}
		})
	}
}

func TestApplyDocumentChange(t *testing.T) {
	tests := []struct {
		name       string
		change     protocol.DocumentChange
		expectErr  bool
		setupMocks func(*mockFileSystem)
		checkState func(*testing.T, *mockFileSystem)
	}{
		{
			name: "Create file",
			change: protocol.DocumentChange{
				CreateFile: &protocol.CreateFile{
					URI: "file:///test/newfile.txt",
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if _, ok := mfs.files["/test/newfile.txt"]; !ok {
					t.Errorf("File was not created")
				}
			},
		},
		{
			name: "Create file - overwrite",
			change: protocol.DocumentChange{
				CreateFile: &protocol.CreateFile{
					URI: "file:///test/existing.txt",
					Options: &protocol.CreateFileOptions{
						Overwrite: true,
					},
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/existing.txt": []byte("existing content"),
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if content, ok := mfs.files["/test/existing.txt"]; !ok {
					t.Errorf("File was not created")
				} else if string(content) != "" {
					t.Errorf("File was not overwritten, content: %s", string(content))
				}
			},
		},
		{
			name: "Create file - ignore if exists",
			change: protocol.DocumentChange{
				CreateFile: &protocol.CreateFile{
					URI: "file:///test/existing.txt",
					Options: &protocol.CreateFileOptions{
						IgnoreIfExists: true,
					},
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/existing.txt": []byte("existing content"),
				}
				mfs.fileStats = map[string]os.FileInfo{
					"/test/existing.txt": mockFileInfo{name: "existing.txt"},
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if content, ok := mfs.files["/test/existing.txt"]; !ok {
					t.Errorf("File was removed")
				} else if string(content) != "existing content" {
					t.Errorf("File was modified, content: %s", string(content))
				}
			},
		},
		{
			name: "Delete file",
			change: protocol.DocumentChange{
				DeleteFile: &protocol.DeleteFile{
					URI: "file:///test/existing.txt",
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/existing.txt": []byte("existing content"),
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if _, ok := mfs.files["/test/existing.txt"]; ok {
					t.Errorf("File was not deleted")
				}
			},
		},
		{
			name: "Delete file recursively",
			change: protocol.DocumentChange{
				DeleteFile: &protocol.DeleteFile{
					URI: "file:///test/dir",
					Options: &protocol.DeleteFileOptions{
						Recursive: true,
					},
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/dir/file1.txt":        []byte("content 1"),
					"/test/dir/file2.txt":        []byte("content 2"),
					"/test/dir/subdir/file3.txt": []byte("content 3"),
					"/test/other.txt":            []byte("other content"),
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if _, ok := mfs.files["/test/dir/file1.txt"]; ok {
					t.Errorf("File in directory was not deleted")
				}
				if _, ok := mfs.files["/test/dir/file2.txt"]; ok {
					t.Errorf("File in directory was not deleted")
				}
				if _, ok := mfs.files["/test/dir/subdir/file3.txt"]; ok {
					t.Errorf("File in subdirectory was not deleted")
				}
				if _, ok := mfs.files["/test/other.txt"]; !ok {
					t.Errorf("File outside target directory was deleted")
				}
			},
		},
		{
			name: "Rename file",
			change: protocol.DocumentChange{
				RenameFile: &protocol.RenameFile{
					OldURI: "file:///test/oldname.txt",
					NewURI: "file:///test/newname.txt",
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/oldname.txt": []byte("file content"),
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if _, ok := mfs.files["/test/oldname.txt"]; ok {
					t.Errorf("Old file still exists")
				}
				if content, ok := mfs.files["/test/newname.txt"]; !ok {
					t.Errorf("New file was not created")
				} else if string(content) != "file content" {
					t.Errorf("New file has incorrect content: %s", string(content))
				}
			},
		},
		{
			name: "Rename file - no overwrite",
			change: protocol.DocumentChange{
				RenameFile: &protocol.RenameFile{
					OldURI: "file:///test/oldname.txt",
					NewURI: "file:///test/existing.txt",
					Options: &protocol.RenameFileOptions{
						Overwrite: false,
					},
				},
			},
			expectErr: true,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/oldname.txt":  []byte("old content"),
					"/test/existing.txt": []byte("existing content"),
				}
				mfs.fileStats = map[string]os.FileInfo{
					"/test/existing.txt": mockFileInfo{name: "existing.txt"},
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if _, ok := mfs.files["/test/oldname.txt"]; !ok {
					t.Errorf("Old file was removed despite rename failure")
				}
				if content, ok := mfs.files["/test/existing.txt"]; !ok || string(content) != "existing content" {
					t.Errorf("Existing file was modified despite no overwrite")
				}
			},
		},
		{
			name: "Text document edit",
			change: protocol.DocumentChange{
				TextDocumentEdit: &protocol.TextDocumentEdit{
					TextDocument: protocol.OptionalVersionedTextDocumentIdentifier{
						TextDocumentIdentifier: protocol.TextDocumentIdentifier{
							URI: "file:///test/document.txt",
						},
					},
					Edits: []protocol.Or_TextDocumentEdit_edits_Elem{
						{
							Value: protocol.TextEdit{
								Range: protocol.Range{
									Start: protocol.Position{Line: 0, Character: 5},
									End:   protocol.Position{Line: 0, Character: 9},
								},
								NewText: "was",
							},
						},
					},
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/document.txt": []byte("This is a test line"),
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if content, ok := mfs.files["/test/document.txt"]; !ok {
					t.Errorf("File not found")
				} else if string(content) != "This was test line" {
					t.Errorf("Text edit not applied correctly, content: %s", string(content))
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mfs := &mockFileSystem{}
			tt.setupMocks(mfs)
			cleanup := setupMockFileSystem(t, mfs)
			defer cleanup()

			err := ApplyDocumentChange(tt.change)
			if tt.expectErr {
				if err == nil {
					t.Errorf("Expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v", err)
				}
				tt.checkState(t, mfs)
			}
		})
	}
}

func TestApplyWorkspaceEdit(t *testing.T) {
	tests := []struct {
		name       string
		edit       protocol.WorkspaceEdit
		expectErr  bool
		setupMocks func(*mockFileSystem)
		checkState func(*testing.T, *mockFileSystem)
	}{
		{
			name: "Text edits via Changes field",
			edit: protocol.WorkspaceEdit{
				Changes: map[protocol.DocumentUri][]protocol.TextEdit{
					"file:///test/file1.txt": {
						{
							Range: protocol.Range{
								Start: protocol.Position{Line: 0, Character: 5},
								End:   protocol.Position{Line: 0, Character: 9},
							},
							NewText: "was",
						},
					},
					"file:///test/file2.txt": {
						{
							Range: protocol.Range{
								Start: protocol.Position{Line: 1, Character: 0},
								End:   protocol.Position{Line: 1, Character: 6},
							},
							NewText: "Modified",
						},
					},
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/file1.txt": []byte("This is a test line"),
					"/test/file2.txt": []byte("Line 1\nLine 2\nLine 3"),
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if content, ok := mfs.files["/test/file1.txt"]; !ok {
					t.Errorf("File1 not found")
				} else if string(content) != "This was test line" {
					t.Errorf("Edit to file1 not applied correctly, content: %s", string(content))
				}

				if content, ok := mfs.files["/test/file2.txt"]; !ok {
					t.Errorf("File2 not found")
				} else if string(content) != "Line 1\nModified\nLine 3" {
					t.Errorf("Edit to file2 not applied correctly, content: %s", string(content))
				}
			},
		},
		{
			name: "Document changes",
			edit: protocol.WorkspaceEdit{
				DocumentChanges: []protocol.DocumentChange{
					{
						CreateFile: &protocol.CreateFile{
							URI: "file:///test/newfile.txt",
						},
					},
					{
						RenameFile: &protocol.RenameFile{
							OldURI: "file:///test/oldname.txt",
							NewURI: "file:///test/newname.txt",
						},
					},
					{
						TextDocumentEdit: &protocol.TextDocumentEdit{
							TextDocument: protocol.OptionalVersionedTextDocumentIdentifier{
								TextDocumentIdentifier: protocol.TextDocumentIdentifier{
									URI: "file:///test/document.txt",
								},
							},
							Edits: []protocol.Or_TextDocumentEdit_edits_Elem{
								{
									Value: protocol.TextEdit{
										Range: protocol.Range{
											Start: protocol.Position{Line: 0, Character: 5},
											End:   protocol.Position{Line: 0, Character: 9},
										},
										NewText: "was",
									},
								},
							},
						},
					},
				},
			},
			expectErr: false,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/oldname.txt":  []byte("file content"),
					"/test/document.txt": []byte("This is a test line"),
				}
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				if _, ok := mfs.files["/test/newfile.txt"]; !ok {
					t.Errorf("New file was not created")
				}

				if _, ok := mfs.files["/test/oldname.txt"]; ok {
					t.Errorf("Old file still exists")
				}

				if _, ok := mfs.files["/test/newname.txt"]; !ok {
					t.Errorf("Renamed file not found")
				}

				if content, ok := mfs.files["/test/document.txt"]; !ok {
					t.Errorf("Document not found")
				} else if string(content) != "This was test line" {
					t.Errorf("Text edit not applied correctly, content: %s", string(content))
				}
			},
		},
		{
			name: "Error in Changes field",
			edit: protocol.WorkspaceEdit{
				Changes: map[protocol.DocumentUri][]protocol.TextEdit{
					"file:///test/file1.txt": {
						{
							Range: protocol.Range{
								Start: protocol.Position{Line: 0, Character: 5},
								End:   protocol.Position{Line: 0, Character: 9},
							},
							NewText: "was",
						},
					},
					"file:///test/missing.txt": {
						{
							Range: protocol.Range{
								Start: protocol.Position{Line: 0, Character: 0},
								End:   protocol.Position{Line: 0, Character: 1},
							},
							NewText: "Modified",
						},
					},
				},
			},
			expectErr: true,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{
					"/test/file1.txt": []byte("This is a test line"),
				}
				// Missing file causes an error
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				// The first edit might or might not be applied depending on implementation
				// so we don't check it
			},
		},
		{
			name: "Error in DocumentChanges field",
			edit: protocol.WorkspaceEdit{
				DocumentChanges: []protocol.DocumentChange{
					{
						CreateFile: &protocol.CreateFile{
							URI: "file:///test/newfile.txt",
						},
					},
					{
						RenameFile: &protocol.RenameFile{
							OldURI: "file:///test/missing.txt", // Missing file causes error
							NewURI: "file:///test/newname.txt",
						},
					},
				},
			},
			expectErr: true,
			setupMocks: func(mfs *mockFileSystem) {
				mfs.files = map[string][]byte{}
				// Missing file causes an error
			},
			checkState: func(t *testing.T, mfs *mockFileSystem) {
				// The first operation might or might not be applied depending on implementation
				// so we don't check it
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mfs := &mockFileSystem{}
			tt.setupMocks(mfs)
			cleanup := setupMockFileSystem(t, mfs)
			defer cleanup()

			err := ApplyWorkspaceEdit(tt.edit)
			if tt.expectErr {
				if err == nil {
					t.Errorf("Expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v", err)
				}
				tt.checkState(t, mfs)
			}
		})
	}
}
