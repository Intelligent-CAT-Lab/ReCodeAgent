#include "../../include/logging/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include<string.h>
#include<errno.h>


// TODO: take care of std output such as stderr, stdout
// TODO: handle log file in .vaultsync
void logger(enum LOGGING_TAG tag, const char* message) {
   time_t now;
   time(&now);
   char* time_now = ctime(&now);
   time_now[strlen(time_now) - 1] = 0;
   switch(tag) {
      case INFO_TAG:
         printf("%s " COLOR_BOLD "[%s]" COLOR_OFF ": %s\n", time_now, "INFO", message);
         break;
      case ERROR_TAG:
         fprintf(stderr, "%s " KRED COLOR_BOLD "[%s]" COLOR_OFF KNORMAL ": %s (%s)\n", time_now, "ERROR", message, strerror(errno));
         break;
      case WARNING_TAG:
         printf("%s " COLOR_BOLD "[%s]" COLOR_OFF ": %s\n", time_now, "WARNING", message);
         break;
      default:
   }
}