package logging

import (
	"bytes"
	"maps"
	"strings"
	"testing"
)

func TestLogger(t *testing.T) {
	// Save original writer to restore after test
	originalWriter := Writer
	originalLevels := make(map[Component]LogLevel)
	maps.Copy(originalLevels, ComponentLevels)

	// Set up a buffer to capture logs
	var buf bytes.Buffer
	SetWriter(&buf)

	// Reset buffer and log levels after test
	defer func() {
		SetWriter(originalWriter)
		maps.Copy(ComponentLevels, originalLevels)
	}()

	// Test different log levels
	tests := []struct {
		name           string
		component      Component
		componentLevel LogLevel
		logFunc        func(Logger)
		level          LogLevel
		shouldLog      bool
	}{
		{
			name:           "Debug message with Debug level",
			component:      Core,
			componentLevel: LevelDebug,
			logFunc:        func(l Logger) { l.Debug("test debug message") },
			level:          LevelDebug,
			shouldLog:      true,
		},
		{
			name:           "Debug message with Info level",
			component:      Core,
			componentLevel: LevelInfo,
			logFunc:        func(l Logger) { l.Debug("test debug message") },
			level:          LevelDebug,
			shouldLog:      false,
		},
		{
			name:           "Info message with Info level",
			component:      LSP,
			componentLevel: LevelInfo,
			logFunc:        func(l Logger) { l.Info("test info message") },
			level:          LevelInfo,
			shouldLog:      true,
		},
		{
			name:           "Warn message with Error level",
			component:      Watcher,
			componentLevel: LevelError,
			logFunc:        func(l Logger) { l.Warn("test warn message") },
			level:          LevelWarn,
			shouldLog:      false,
		},
		{
			name:           "Error message with Error level",
			component:      Tools,
			componentLevel: LevelError,
			logFunc:        func(l Logger) { l.Error("test error message") },
			level:          LevelError,
			shouldLog:      true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Reset buffer
			buf.Reset()

			// Set component log level
			SetLevel(tt.component, tt.componentLevel)

			// Create logger and log message
			logger := NewLogger(tt.component)
			tt.logFunc(logger)

			// Check if message was logged
			loggedMessage := buf.String()
			if tt.shouldLog && loggedMessage == "" {
				t.Errorf("Expected log message but got none")
			} else if !tt.shouldLog && loggedMessage != "" {
				t.Errorf("Expected no log message but got: %s", loggedMessage)
			}

			// When log should appear, check if it contains expected parts
			if tt.shouldLog {
				if !strings.Contains(loggedMessage, tt.level.String()) {
					t.Errorf("Log message missing level '%s': %s", tt.level, loggedMessage)
				}
				if !strings.Contains(loggedMessage, string(tt.component)) {
					t.Errorf("Log message missing component '%s': %s", tt.component, loggedMessage)
				}
			}
		})
	}
}
