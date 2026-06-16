package watcher

import (
	"os"
	"path/filepath"

	gitignore "github.com/sabhiram/go-gitignore"
)

// GitignoreMatcher provides a simple wrapper around the go-gitignore package
type GitignoreMatcher struct {
	gitignore *gitignore.GitIgnore
	basePath  string
}

// NewGitignoreMatcher creates a new gitignore matcher for a workspace
func NewGitignoreMatcher(workspacePath string) (*GitignoreMatcher, error) {
	gitignorePath := filepath.Join(workspacePath, ".gitignore")

	// Check if .gitignore exists
	_, err := os.Stat(gitignorePath)
	if os.IsNotExist(err) {
		// No .gitignore file, return a matcher with no patterns
		emptyIgnore := gitignore.CompileIgnoreLines([]string{}...)
		return &GitignoreMatcher{
			gitignore: emptyIgnore,
			basePath:  workspacePath,
		}, nil
	} else if err != nil {
		return nil, err
	}

	// Parse .gitignore file using the go-gitignore library
	ignore, err := gitignore.CompileIgnoreFile(gitignorePath)
	if err != nil {
		return nil, err
	}

	return &GitignoreMatcher{
		gitignore: ignore,
		basePath:  workspacePath,
	}, nil
}

// ShouldIgnore checks if a file or directory should be ignored based on gitignore patterns
func (g *GitignoreMatcher) ShouldIgnore(path string, isDir bool) bool {
	// Make path relative to workspace root
	relPath, err := filepath.Rel(g.basePath, path)
	if err != nil {
		return false
	}

	// Use the go-gitignore Match function to check if the path should be ignored
	return g.gitignore.MatchesPath(relPath)
}
