#include<stdio.h>
#include<stdlib.h>
#include<sys/stat.h>
#include<sys/types.h>
#include<unistd.h>
#include<string.h>
#include<math.h>

#include "storage_mgr.h"

FILE *pageFile;

 void initStorageManager (void) {
	pageFile = NULL;
}

// Define create a Page file 
 RC createPageFile(char *fileName) {
    FILE *file = fopen(fileName, "w+");
    if (file == NULL) {
        return RC_FILE_NOT_FOUND; 
    }
    SM_PageHandle emptyPage = calloc(PAGE_SIZE, 1);
    if (emptyPage == NULL) {
        fclose(file);
        return RC_WRITE_FAILED;
    }
    size_t writeSize = fwrite(emptyPage, sizeof(char), PAGE_SIZE, file);
    free(emptyPage);
    fclose(file);
    if (writeSize < PAGE_SIZE) {
        return RC_WRITE_FAILED;
    }
    return RC_OK;
}

// Define Open a Page file
RC openPageFile(char *fileName, SM_FileHandle *fHandle) {
    FILE *fp;
    int File_RC;
    char *pageData;
    int i;

    pageData = (char *) calloc(PAGE_SIZE, sizeof(char));
    if (pageData == NULL) {
        return RC_MEMORY_ALLOCATION_FAIL;
    }
    fp = fopen(fileName, "rb+");
    if (fp == NULL) {
        free(pageData);
        return RC_FILE_NOT_FOUND;
    }
    int DataReadOK = 0;
    for (i = 0; i < 1; i++) {
        File_RC = fread(pageData, PAGE_SIZE, 1, fp);
        if (File_RC == 1) {
            DataReadOK = 1;
            break;
        }
    }
    if (!DataReadOK) {
        free(pageData);
        fclose(fp); 
        return RC_READ_FAILED;
    }
    if (pageData != NULL) {
        fHandle->fileName = fileName;
        fHandle->totalNumPages = atoi(pageData);
        fHandle->curPagePos = 0;
        fHandle->mgmtInfo = fp;
        free(pageData); 
    } else {
        fclose(fp);
        free(pageData); 
        return RC_INVALID_HEADER;
    }
    return RC_OK;
}

// Define Close a Page file 
RC closePageFile(SM_FileHandle *fHandle) {
    int FILE_RC;
    char *pageData;
    long offset;
    int Action, maxTrial = 1;

    pageData = (char *) calloc(PAGE_SIZE, sizeof(char));
    if (pageData == NULL) {
        return RC_MEMORY_ALLOCATION_FAIL;
    }
    offset = 0;
    FILE_RC = fseek(fHandle->mgmtInfo, offset, SEEK_SET);
    if (FILE_RC != 0) {
        free(pageData);
        return RC_SEEK_FAILED;
    }
    sprintf(pageData, "%d", fHandle->totalNumPages);
    for (Action = 0; Action < maxTrial; Action++) {
        FILE_RC = fwrite(pageData, PAGE_SIZE, 1, fHandle->mgmtInfo);
        if (FILE_RC != 1) {
        free(pageData);
        return RC_WRITE_FAILED;
        } 
        else {
        // Close the file
        FILE_RC = fclose(fHandle->mgmtInfo);
        if (FILE_RC != 0) {
            free(pageData);
            return RC_CLOSE_FAILED;
        }
         }
    free(pageData);
    return RC_OK;
    }
}

// Define destroy a Page file 
RC destroyPageFile(char *fileName) {
    int FILE_RC;
    int Trial = 0;
    const int maxTrial = 3; 
    while (Trial < maxTrial) {
        FILE_RC = remove(fileName);
        if (FILE_RC == 0) {
            return RC_OK;
        } else {
            Trial++;
        }
    }
    if (FILE_RC != 0) {
        return RC_DESTROY_FAILED;
    }
    return RC_OK;
}

//  Define Read a block 
RC readBlock(int pageNum, SM_FileHandle *fHandle, SM_PageHandle memPage) {
    long offset;
    int FILE_RC;
    int Action, maxTrial = 1; 
    if (pageNum < 0 || pageNum >= fHandle->totalNumPages) {
        return RC_READ_NON_EXISTING_PAGE;
    }
    offset = (pageNum + 1) * PAGE_SIZE;
    FILE_RC = fseek(fHandle->mgmtInfo, offset, SEEK_SET);
    if (FILE_RC != 0) {
        return RC_SEEK_FAILED;
    }
    for (Action = 0; Action < maxTrial; Action++) {
        FILE_RC = fread(memPage, PAGE_SIZE, 1, fHandle->mgmtInfo);
        if (FILE_RC == 1) {
            break;
        }
    }
    if (FILE_RC != 1) {
        return RC_READ_FAILED;
    }
    fHandle->curPagePos = pageNum;
    return RC_OK;
}

// Define Read first block 
RC readFirstBlock(SM_FileHandle *fHandle, SM_PageHandle memPage) {
    int read_code;
    const int maxTrial = 3; 
    int Action = 0;
    for (Action = 0; Action < maxTrial; Action++) {
        read_code = readBlock(0, fHandle, memPage);
        if (read_code == RC_OK) {
            break;
        } else {
            printf("Action %d to read the first block failed. Retrying...\n", Action + 1);
        }
    }
    return read_code;
}

// Define Read Last block
RC readLastBlock(SM_FileHandle *fHandle, SM_PageHandle memPage) {
    int read_code;
    int pageNum;
    const int maxTrial = 3; 
    int Action = 0;
    pageNum = fHandle->totalNumPages - 1;
    if (pageNum < 0) {
        return RC_READ_NON_EXISTING_PAGE;
    }
    for (Action = 0; Action < maxTrial; Action++) {
        read_code = readBlock(pageNum, fHandle, memPage);
        if (read_code == RC_OK) {
            break;
        } 
    }
    return read_code;
}

// Define read previous block
RC readPreviousBlock(SM_FileHandle *fHandle, SM_PageHandle memPage) {
    int read_code;
    int pageNum;
    const int maxTrial = 2;
    int Action = 0;
    pageNum = fHandle->curPagePos - 1;
    if (pageNum < 0) {
        return RC_READ_NON_EXISTING_PAGE; 
    }
    for (Action = 0; Action < maxTrial; Action++) {
        read_code = readBlock(pageNum, fHandle, memPage);
        if (read_code == RC_OK) {
            break;
        }
    }
    return read_code;
}

// Define read current block
RC readCurrentBlock(SM_FileHandle *fHandle, SM_PageHandle memPage) {
    if (fHandle == NULL || fHandle->mgmtInfo == NULL) {
        return RC_FILE_HANDLE_NOT_INIT;
    }
    int pageNum = fHandle->curPagePos;
    if (pageNum < 0 || pageNum >= fHandle->totalNumPages) {
        return RC_READ_NON_EXISTING_PAGE;
    }
    int read_code = RC_READ_NON_EXISTING_PAGE;
    const int maxReadTrial = 3; 
    for (int Action = 0; Action < maxReadTrial; Action++) {
        read_code = readBlock(pageNum, fHandle, memPage);
        if (read_code == RC_OK) {
            break; 
        }
    }
    return read_code;
}

// Define read next block
RC readNextBlock(SM_FileHandle *fHandle, SM_PageHandle memPage) {
    int read_code;
    int pageNum;
    const int maxTrial = 2; 
    int Action = 0;
    pageNum = fHandle->curPagePos + 1;
    for (Action = 0; Action < maxTrial; Action++) {
        read_code = readBlock(pageNum, fHandle, memPage);
        if (read_code == RC_OK) {
            break;
        }
    }
    if (read_code == RC_READ_NON_EXISTING_PAGE) {
    }
    return read_code;
}

// Define write block
RC writeBlock(int pageNum, SM_FileHandle *fHandle, SM_PageHandle memPage) {
    FILE *fp = fHandle->mgmtInfo;
    long int offset;
    if (fp == NULL) {
        return RC_FILE_NOT_FOUND; 
    }
    if (pageNum < 0) {
        return RC_WRITE_FAILED; 
    }
    offset = (pageNum + 1) * PAGE_SIZE;
    fseek(fp, 0, SEEK_END);
    long fileSize = ftell(fp);
    if (offset > fileSize) {
        if (pageNum == fHandle->totalNumPages) {
            fseek(fp, fileSize, SEEK_SET);
            for (long i = fileSize; i < offset; i++) {
                fputc('\0', fp);
            }
            fHandle->totalNumPages++; 
        } else {
            return RC_WRITE_FAILED; 
        }
    }
        if (lseek(fileno(fp), offset, SEEK_SET) == -1) {
            return RC_SEEK_FAILED;
            }
            ssize_t bytes_written = write(fileno(fp), memPage, PAGE_SIZE);
            if (bytes_written != PAGE_SIZE) {
                return RC_WRITE_FAILED;
            }
            fHandle->curPagePos = pageNum;

            return RC_OK;
}

// Define write current block 
RC writeCurrentBlock(SM_FileHandle *fHandle, SM_PageHandle memPage) {
    int read_code;
    int pageNum;
    const int maxWriteTrial = 3; 
    int writeAction;
    int SuccessStatus = 0;
    pageNum = fHandle->curPagePos; 
    if (pageNum < 0 || pageNum >= fHandle->totalNumPages) {
        return RC_WRITE_FAILED; 
    }
    for (writeAction = 0; writeAction < maxWriteTrial; writeAction++) {
        read_code = writeBlock(pageNum, fHandle, memPage);
        if (read_code == RC_OK) {
            SuccessStatus = 1; 
            break; 
        } 
    }
    if (!SuccessStatus) {
        return RC_WRITE_FAILED; 
    }
    return RC_OK;
}

// Define append and empty block
RC appendEmptyBlock(SM_FileHandle *fHandle) {
    FILE *fp = fHandle->mgmtInfo;
    char *pageData;
    int FILE_RC;
    const int maxWriteTrial = 2;
    int Action = 0;
    int SuccessStatus = 0;
    pageData = (char *)calloc(PAGE_SIZE, sizeof(char));
    if (pageData == NULL) {
        return RC_MEMORY_ALLOCATION_FAIL;
    }
    if (fp == NULL) {
        free(pageData);
        return RC_FILE_HANDLE_NOT_INIT;
    }
    FILE_RC = fseek(fp, 0, SEEK_END);
    if (FILE_RC != 0) {
        free(pageData);
        return RC_SEEK_FAILED;
    }
    for (Action = 0; Action < maxWriteTrial; Action++) {
        FILE_RC = fwrite(pageData, PAGE_SIZE, 1, fp);
        if (FILE_RC == 1) {
            SuccessStatus = 1;
            break;
        }
    }
    free(pageData);
    if (!SuccessStatus) {
        return RC_WRITE_FAILED;
    }
    fHandle->totalNumPages += 1;
    fHandle->curPagePos = fHandle->totalNumPages - 1;
    return RC_OK;
}

// Define ensureCapacity
RC ensureCapacity(int allPagesCount, SM_FileHandle *fHandle) {
    if (fHandle == NULL || fHandle->mgmtInfo == NULL) {
        return RC_FILE_HANDLE_NOT_INIT;
    }
    int activePageCount = fHandle->totalNumPages;
    if (allPagesCount <= activePageCount) {
        return RC_OK;
    }
    int IncrementPageCount = allPagesCount - activePageCount;
    int FILE_RC, i;
    for (i = 0; i < IncrementPageCount; i++) {
        FILE_RC = appendEmptyBlock(fHandle);
        if (FILE_RC != RC_OK) {
            return FILE_RC;
        }
    }
    return RC_OK;
}




