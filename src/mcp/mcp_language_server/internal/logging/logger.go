package logging

import (
	"fmt"
	"io"
	"log"
	"os"
	"strings"
	"sync"
)

// LogLevel represents the severity of a log message
type LogLevel int

const (
	// Debug level for verbose development logs
	LevelDebug LogLevel = iota
	// Info level for general operational information
	LevelInfo
	// Warn level for warning conditions
	LevelWarn
	// Error level for error conditions
	LevelError
	// Fatal level for critical errors
	LevelFatal
)

// String returns the string representation of a log level
func (l LogLevel) String() string {
	switch l {
	case LevelDebug:
		return "DEBUG"
	case LevelInfo:
		return "INFO"
	case LevelWarn:
		return "WARN"
	case LevelError:
		return "ERROR"
	case LevelFatal:
		return "FATAL"
	default:
		return fmt.Sprintf("LEVEL(%d)", l)
	}
}

// Component represents a specific part of the application for which logs can be filtered
type Component string

const (
	// Core component for the main application
	Core Component = "core"
	// LSP component for high-level Language Server Protocol operations
	LSP Component = "lsp"
	// LSPWire component for raw LSP wire protocol messages
	LSPWire Component = "wire"
	// LSPProcess component for logs from the LSP server process itself
	LSPProcess Component = "lsp-process"
	// Watcher component for file system watching
	Watcher Component = "watcher"
	// Tools component for LSP tools
	Tools Component = "tools"
)

// DefaultMinLevel is the default minimum log level
var DefaultMinLevel = LevelInfo

// ComponentLevels tracks the minimum log level for each component
var ComponentLevels = map[Component]LogLevel{}

// Writer is the destination for logs
var Writer io.Writer = os.Stderr

// TestOutput can be set during tests to capture log output
var TestOutput io.Writer

// logMu protects concurrent modifications to logging config
var logMu sync.Mutex

// Initialize from environment variables
func init() {
	// Set default levels for each component
	ComponentLevels[Core] = DefaultMinLevel
	ComponentLevels[LSP] = DefaultMinLevel
	ComponentLevels[Watcher] = DefaultMinLevel
	ComponentLevels[Tools] = DefaultMinLevel
	ComponentLevels[LSPProcess] = DefaultMinLevel
	ComponentLevels[LSPWire] = DefaultMinLevel

	// Parse log level from environment variable
	if level := os.Getenv("LOG_LEVEL"); level != "" {
		switch strings.ToUpper(level) {
		case "DEBUG":
			DefaultMinLevel = LevelDebug
		case "INFO":
			DefaultMinLevel = LevelInfo
		case "WARN":
			DefaultMinLevel = LevelWarn
		case "ERROR":
			DefaultMinLevel = LevelError
		case "FATAL":
			DefaultMinLevel = LevelFatal
		}

		// Set all components to this level by default
		for comp := range ComponentLevels {
			ComponentLevels[comp] = DefaultMinLevel
		}
	}

	// Allow overriding levels for specific components
	if compLevels := os.Getenv("LOG_COMPONENT_LEVELS"); compLevels != "" {
		for _, part := range strings.SplitN(compLevels, ",", -1) {
			compAndLevel := strings.Split(part, ":")
			if len(compAndLevel) != 2 {
				continue
			}

			comp := Component(strings.TrimSpace(compAndLevel[0]))
			levelStr := strings.ToUpper(strings.TrimSpace(compAndLevel[1]))

			var level LogLevel
			switch levelStr {
			case "DEBUG":
				level = LevelDebug
			case "INFO":
				level = LevelInfo
			case "WARN":
				level = LevelWarn
			case "ERROR":
				level = LevelError
			case "FATAL":
				level = LevelFatal
			default:
				continue
			}

			ComponentLevels[comp] = level
		}
	}

	// Use custom log file if specified
	if logFile := os.Getenv("LOG_FILE"); logFile != "" {
		file, err := os.OpenFile(logFile, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
		if err == nil {
			Writer = io.MultiWriter(os.Stderr, file)
		}
	}

	// Configure the standard logger
	log.SetOutput(Writer)
	log.SetFlags(log.Ldate | log.Ltime | log.Lmicroseconds)
}

// Logger is the interface for component-specific logging
type Logger interface {
	Debug(format string, v ...any)
	Info(format string, v ...any)
	Warn(format string, v ...any)
	Error(format string, v ...any)
	Fatal(format string, v ...any)
	IsLevelEnabled(level LogLevel) bool
}

// ComponentLogger is a logger for a specific component
type ComponentLogger struct {
	component Component
}

// NewLogger creates a new logger for the specified component
func NewLogger(component Component) Logger {
	return &ComponentLogger{
		component: component,
	}
}

// IsLevelEnabled returns true if the given log level is enabled for this component
func (l *ComponentLogger) IsLevelEnabled(level LogLevel) bool {
	logMu.Lock()
	defer logMu.Unlock()

	minLevel, ok := ComponentLevels[l.component]
	if !ok {
		minLevel = DefaultMinLevel
	}
	return level >= minLevel
}

// log logs a message at the specified level if it meets the threshold
func (l *ComponentLogger) log(level LogLevel, format string, v ...any) {
	if !l.IsLevelEnabled(level) {
		return
	}

	message := fmt.Sprintf(format, v...)
	logMessage := fmt.Sprintf("[%s][%s] %s", level, l.component, message)

	if err := log.Output(3, logMessage); err != nil {
		fmt.Fprintf(os.Stderr, "Failed to output log: %v\n", err)
	}

	// Write to test output if set
	if TestOutput != nil {
		if _, err := fmt.Fprintln(TestOutput, logMessage); err != nil {
			fmt.Fprintf(os.Stderr, "Failed to output log to test output: %v\n", err)
		}
	}
}

// Debug logs a debug message
func (l *ComponentLogger) Debug(format string, v ...any) {
	l.log(LevelDebug, format, v...)
}

// Info logs an info message
func (l *ComponentLogger) Info(format string, v ...any) {
	l.log(LevelInfo, format, v...)
}

// Warn logs a warning message
func (l *ComponentLogger) Warn(format string, v ...any) {
	l.log(LevelWarn, format, v...)
}

// Error logs an error message
func (l *ComponentLogger) Error(format string, v ...any) {
	l.log(LevelError, format, v...)
}

// Fatal logs a fatal message and exits
func (l *ComponentLogger) Fatal(format string, v ...any) {
	l.log(LevelFatal, format, v...)
	os.Exit(1)
}

// SetLevel sets the minimum log level for a component
func SetLevel(component Component, level LogLevel) {
	logMu.Lock()
	defer logMu.Unlock()
	ComponentLevels[component] = level
}

// SetGlobalLevel sets the log level for all components
func SetGlobalLevel(level LogLevel) {
	logMu.Lock()
	defer logMu.Unlock()

	DefaultMinLevel = level
	for comp := range ComponentLevels {
		ComponentLevels[comp] = level
	}
}

// SetWriter sets the writer for log output
func SetWriter(w io.Writer) {
	logMu.Lock()
	defer logMu.Unlock()

	Writer = w
	log.SetOutput(Writer)
}

// SetupFileLogging configures logging to a file in addition to stderr
func SetupFileLogging(filePath string) error {
	file, err := os.OpenFile(filePath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return fmt.Errorf("failed to open log file: %w", err)
	}

	logMu.Lock()
	defer logMu.Unlock()

	Writer = io.MultiWriter(os.Stderr, file)
	log.SetOutput(Writer)
	return nil
}

// SetupTestLogging configures logging for tests
func SetupTestLogging(captureOutput io.Writer) {
	logMu.Lock()
	defer logMu.Unlock()

	// Set test output for capturing logs
	TestOutput = captureOutput
}

// ResetTestLogging resets logging after tests
func ResetTestLogging() {
	logMu.Lock()
	defer logMu.Unlock()

	TestOutput = nil
}
