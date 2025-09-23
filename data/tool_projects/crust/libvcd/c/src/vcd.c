#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "vcd.h"

#define isexpression(c) (strchr("-0123456789zZxXbU", (c)))

#define BUFFER_LENGTH 512

typedef enum { BEFORE_MODULE_DEFINITIONS, INSIDE_TOP_MODULE, INSIDE_INNER_MODULES } state_t;

static vcd_t *new_vcd();
static bool parse_instruction(FILE *file, vcd_t *vcd, state_t *state);
static bool parse_timestamp(FILE *file, timestamp_t *timestamp);
static bool parse_assignment(FILE *file, vcd_t *vcd, timestamp_t timestamp);
static int get_signal_index(const char *string);

vcd_t *vcd_read_from_path(char *path) {
    FILE *file = fopen(path, "r");
    if (file == NULL)
        return NULL;

    vcd_t *vcd = new_vcd();
    timestamp_t current_timestamp = 0;
    state_t state = BEFORE_MODULE_DEFINITIONS;

    int character = 0;
    while ((character = fgetc(file)) != EOF) {
        if (character == '$') {
            bool successful = parse_instruction(file, vcd, &state);
            if (successful)
                continue;
        }

        else if (character == '#') {
            bool successful = parse_timestamp(file, &current_timestamp);
            if (successful)
                continue;
        }

        else if (isexpression(character)) {
            ungetc(character, file);
            bool successful = parse_assignment(file, vcd, current_timestamp);
            if (successful)
                continue;
        }

        else if (isspace(character))
            continue;

        fclose(file);
        free(vcd);
        return NULL;
    }

    fclose(file);
    return vcd;
}

signal_t *vcd_get_signal_by_name(vcd_t *vcd, const char *signal_name) {
    for (int i = 0; i < vcd->signals_count; ++i) {
        if (strcmp(vcd->signals[i].name, signal_name) == 0)
            return &vcd->signals[i];
    }

    return NULL;
}

char *vcd_signal_get_value_at_timestamp(signal_t *signal, timestamp_t timestamp) {
    char *previous_value = NULL;
    for (int i = 0; i < signal->changes_count; ++i) {
        value_change_t *value_change = &signal->value_changes[i];
        if (timestamp < value_change->timestamp)
            break;
        previous_value = value_change->value;
    }
    return previous_value;
}

vcd_t *new_vcd() { return (vcd_t *)calloc(1, sizeof(vcd_t)); }

bool parse_instruction(FILE *file, vcd_t *vcd, state_t *state) {
    char instruction[BUFFER_LENGTH];
    if (fscanf(file, "%s", instruction) != 1)
        return false;

    if (strcmp(instruction, "end") == 0 || strcmp(instruction, "dumpvars") == 0 || strcmp(instruction, "dumpall") == 0)
        return true;

    if (strcmp(instruction, "scope") == 0) {
        switch (*state) {
        case BEFORE_MODULE_DEFINITIONS:
            *state = INSIDE_TOP_MODULE;
            break;
        case INSIDE_TOP_MODULE:
            *state = INSIDE_INNER_MODULES;
            break;
        default:
            break;
        }
        fscanf(file, "\n%*[^$]");
        return true;
    }

    if (strcmp(instruction, "scope") == 0 || strcmp(instruction, "upscope") == 0 ||
        strcmp(instruction, "enddefinitions") == 0 || strcmp(instruction, "comment") == 0) {
        fscanf(file, "\n%*[^$]");
        return true;
    }

    if (strcmp(instruction, "var") == 0) {
        if (*state == INSIDE_INNER_MODULES) {
            fscanf(file, " %*[^\n]\n");
            return true;
        }

        signal_t *signal = &vcd->signals[vcd->signals_count];
        vcd->signals_count += 1;

        char signal_id[VCD_NAME_SIZE];
        fscanf(file, " %*s %zu %[^ ] %[^ $]%*[^$]", &signal->size, signal_id, signal->name);
        int index = get_signal_index(signal_id);

        /* This signal is an alias. */
        if (vcd->signals[index].size != 0)
            return true;

        return true;
    }

    if (strcmp(instruction, "date") == 0) {
        fscanf(file, "\n%[^$\n]", vcd->date);
        return true;
    }

    if (strcmp(instruction, "version") == 0) {
        fscanf(file, "\n%[^$\n]", vcd->version);
        return true;
    }

    if (strcmp(instruction, "timescale") == 0) {
        fscanf(file, "\n\t%zu%[^$\n]", &vcd->timescale.scale, vcd->timescale.unit);
        return true;
    }

    return false;
}

bool parse_timestamp(FILE *file, timestamp_t *timestamp) {
    bool successful = fscanf(file, "%u", timestamp) == 1;
    return successful;
}

bool parse_assignment(FILE *file, vcd_t *vcd, timestamp_t timestamp) {
    char buffer[BUFFER_LENGTH];
    char value[VCD_SIGNAL_SIZE];
    char signal_id[VCD_NAME_SIZE];
    fscanf(file, "%[^\n]", buffer);

    bool is_vector = strchr("01xXzZ", buffer[0]) == NULL;
    char *assignment_format_string = is_vector ? "%[^ ] %[^\n]" : "%1s%[^\n]";
    if (sscanf(buffer, assignment_format_string, value, signal_id) != 2)
        return false;

    /* For now, we ignore longer signal ids. */
    if (strlen(signal_id) > 1)
        return true;

    size_t index = get_signal_index(signal_id);
    if (index == -1 || index >= vcd->signals_count)
        return true;

    size_t changes_count = vcd->signals[index].changes_count;
    vcd->signals[index].value_changes[changes_count].timestamp = timestamp;
    strncpy(vcd->signals[index].value_changes[changes_count].value, value, VCD_SIGNAL_SIZE);
    vcd->signals[index].changes_count += 1;

    return true;
}

int get_signal_index(const char *string) {
    int id = *string - '!';
    if (id >= VCD_SIGNAL_COUNT)
        return -1;

    return id;
}
