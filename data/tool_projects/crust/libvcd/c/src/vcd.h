#ifndef LIBVCD_VCD_H
#define LIBVCD_VCD_H

#include <stdint.h>
#include <stdio.h>

#define VCD_SIGNAL_COUNT 32
#define VCD_VALUE_CHANGE_COUNT 4096
#define VCD_SIGNAL_SIZE 64
#define VCD_NAME_SIZE 32
#define VCD_TIME_UNIT_SIZE 8
#define VCD_VERSION_SIZE 64
#define VCD_DATE_SIZE 64

typedef uint32_t timestamp_t;

typedef struct {
    timestamp_t timestamp;
    char value[VCD_SIGNAL_SIZE];
} value_change_t;

typedef struct {
    char name[VCD_NAME_SIZE];
    size_t size;
    value_change_t value_changes[VCD_VALUE_CHANGE_COUNT];
    size_t changes_count;
} signal_t;

typedef struct {
    char unit[VCD_TIME_UNIT_SIZE];
    size_t scale;
} timescale_t;

typedef struct {
    size_t signals_count;
    signal_t signals[VCD_SIGNAL_COUNT];
    char date[VCD_DATE_SIZE];
    char version[VCD_VERSION_SIZE];
    timescale_t timescale;
} vcd_t;

vcd_t *vcd_read_from_path(char *path);

signal_t *vcd_get_signal_by_name(vcd_t *vcd, const char *signal_name);

char *vcd_signal_get_value_at_timestamp(signal_t *signal, timestamp_t timestamp);

#endif // LIBVCD_VCD_H
