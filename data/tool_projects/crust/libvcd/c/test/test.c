#include <stdio.h>
#include <stdlib.h>

#include "vcd.h"

void print_vcd(vcd_t *vcd) {
    printf("{\n\tdate=\"%s\",\n\tversion=\"%s\",\n\ttimescale= {\n\t\tunit=\"%s\",\n\t\tscale=\"%zu\"\n\t},\n\tsignal= "
           "{\n",
           vcd->date, vcd->version, vcd->timescale.unit, vcd->timescale.scale);
    for (int i = 0; i < vcd->signals_count; ++i) {
        printf("\t\t%s= {\n\t\t\tsize=%zu,\n\t\t\tchanges= {\n", vcd->signals[i].name, vcd->signals[i].size);
        for (int j = 0; j < vcd->signals[i].changes_count; ++j) {
            printf("\t\t\t\t{\n\t\t\t\t\ttimestamp=%u,\n\t\t\t\t\tvalue=%s\n\t\t\t\t},\n",
                   vcd->signals[i].value_changes[j].timestamp, vcd->signals[i].value_changes[j].value);
        }
        printf("\t\t\t},\n\t\t},\n");
    }
    printf("\t}\n}\n");
}

int main(int argc, char *argv[]) {
    if (argc % 2 == 1) {
        fprintf(stderr, "Usage: test <vcd-file> [signal-name timestamp] ...\n");
        exit(EXIT_FAILURE);
    }

    vcd_t *vcd = vcd_read_from_path(argv[1]);
    if (vcd == NULL) {
        fprintf(stderr, "Could not read the VCD\n");
        return 1;
    }

    // print_vcd(vcd);
    printf("\n");

    for (int i = 2; i < argc; i += 2) {
        char *signal_name = argv[i];
        signal_t *signal = vcd_get_signal_by_name(vcd, signal_name);
        timestamp_t timestamp = strtol(argv[i + 1], NULL, 0);
        printf("%s at %u equals %s\n", argv[i], timestamp, vcd_signal_get_value_at_timestamp(signal, timestamp));
    }

    free(vcd);
}
