#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include "buffer_mgr.h"
#include "storage_mgr.h"
#include "dberror.h"

// Define Bufferpool
typedef struct Bufferpool
{
     int numRead;
     int numWrite;
     int totalPages;
     int updatedStrategy;
     int free_space;
     int *updatedOrder;
     bool *bitdirty;
     int *fix_count;
     int *accessTime;
     int *pagenum;
     char *pagedata;
     SM_FileHandle fhl;
}Bufferpool;

bool pageFound = FALSE;

//  Helper Functions
static RC writeDirtyPages(BM_BufferPool *const bm);
static RC freeBufferPoolMemory(BM_BufferPool *const bm);
static void ShiftUpdatedOrder(int start, int end, Bufferpool *bp, int newPageNum);
static void UpdateBufferPoolStats(Bufferpool *bp, int memoryAddress, int pageNum);

// Define initBufferPool
RC initBufferPool(BM_BufferPool *const bm, const char *const pageFileName, const int numPages, ReplacementStrategy strategy, void *stratData) {
    SM_FileHandle fh;
    Bufferpool *bp;
    int rcode, i;

    rcode = openPageFile((char *)pageFileName, &fh);
    if (rcode != RC_OK) {
        return rcode; 
    }
    bp = (Bufferpool *)calloc(1, sizeof(Bufferpool));
    if (!bp) {
        return RC_MEMORY_ALLOCATION_FAIL; 
    }
    bp->totalPages = numPages;
    bp->pagedata = (char *)calloc(numPages * PAGE_SIZE, sizeof(char));
    bp->numRead = 0;
    bp->numWrite = 0;
    bp->updatedOrder = (int *)calloc(numPages, sizeof(int));
    bp->bitdirty = (bool *)calloc(numPages, sizeof(bool));
    bp->free_space = numPages;
    bp->fhl = fh;
    bp->pagenum = (int *)calloc(numPages, sizeof(int));

    int arraySize = numPages * sizeof(int);
    bool boolArraySize = numPages * sizeof(bool);
    bp->fix_count = (int *)calloc(numPages, sizeof(int));
    bp->updatedStrategy = strategy;

   

    for (i = 0; i < numPages; i++) {
        bp->bitdirty[i] = FALSE;
    }

    for (i = 0; i < numPages; i++) {
      bp->fix_count[i] = 0;
    }

    for (i = 0; i < numPages; i++) {
        bp->pagenum[i] = NO_PAGE;
    }

    for (i = 0; i < numPages; i++) {
        bp->updatedOrder[i] = NO_PAGE;
    }

    if (bm != NULL) {
        if (pageFileName != NULL) {
            bm->pageFile = strdup(pageFileName); 
        } else {
            bm->pageFile = NULL;
        }
        bm->numPages = numPages;
        bm->strategy = strategy;
        bm->mgmtData = bp;
        if (bp != NULL) {
        } 
        } 
        return RC_OK;
    }

// Define shutdown the buffer pool
RC shutdownBufferPool(BM_BufferPool *const bm) {
    Bufferpool *bpl = bm->mgmtData;
    
    for (int i = 0; i < bpl->totalPages; i++) {
        if (bpl->fix_count[i] != 0) {
            return RC_BUFFERPOOL_IN_USE;
        }
    }
    RC rc = writeDirtyPages(bm);
    if (rc != RC_OK) {
        return rc; 
    }
 
    if (closePageFile(&bpl->fhl) != RC_OK) {
        return RC_CLOSE_FAILED;
    }
    
    freeBufferPoolMemory(bm);
    return RC_OK;
}

    // Helper function
    static RC writeDirtyPages(BM_BufferPool *const bm) {
        Bufferpool *bpl = bm->mgmtData;
        for (int j = 0; j < bpl->totalPages; j++) {
            if (bpl->bitdirty[j]) {
                int record_pointer = j * PAGE_SIZE;
                // Ensure capacity before writing
                RC rc = ensureCapacity(bpl->pagenum[j] + 1, &bpl->fhl);
                if (rc != RC_OK) {
                    return rc;
                }
                // Write block
                rc = writeBlock(bpl->pagenum[j], &bpl->fhl, (bpl->pagedata + record_pointer));
                if (rc != RC_OK) {
                    return RC_WRITE_FAILED;
                }
                bpl->numWrite++;
            }
        }
        return RC_OK;
    }

    // Helper function
    static RC freeBufferPoolMemory(BM_BufferPool *const bm)
                
            {
        if (bm != NULL) {
            Bufferpool *bpl = bm->mgmtData;
            if (bpl != NULL) {
                if (bpl->updatedOrder != NULL) {
                    free(bpl->updatedOrder);
                    bpl->updatedOrder = NULL; 
                }
                if (bpl->pagenum != NULL) {
                    free(bpl->pagenum);
                    bpl->pagenum = NULL; 
                }
                if (bpl->bitdirty != NULL) {
                    free(bpl->bitdirty);
                    bpl->bitdirty = NULL; 
                }
                if (bpl->fix_count != NULL) {
                    free(bpl->fix_count);
                    bpl->fix_count = NULL;
                }
                if (bpl->pagedata != NULL) {
                    free(bpl->pagedata);
                    bpl->pagedata = NULL; 
                }
                free(bpl);
                bm->mgmtData = NULL;
            } else {
            }
            return RC_OK;
        } else {
            return RC_ERROR; 
        }

        return RC_OK;
}
    // Define flush the bufferpool
    RC forceFlushPool(BM_BufferPool *const bm) {
        Bufferpool *bpl;
        int rcode = RC_OK;
        bpl = bm->mgmtData;

        for (int i = 0; bpl != NULL && i < bpl->totalPages; i++) {
                if (bpl->fix_count[i] == 0 && bpl->bitdirty[i] == TRUE) {
                FILE *file = fopen(bpl->fhl.fileName, "r+");
                fseek(file, 0, SEEK_END);
                long fileSize = ftell(file);
                long requiredSize = (bpl->pagenum[i] + 1) * PAGE_SIZE;
                if (fileSize < requiredSize) {
                    fseek(file, requiredSize - 1, SEEK_SET);
                    fputc('\0', file);
                }
                fclose(file);
                file = fopen(bpl->fhl.fileName, "r+");
                if (file == NULL) {
                    rcode = RC_WRITE_FAILED;
                    break;
                }
                int record_pointer = i * PAGE_SIZE;
                fseek(file, bpl->pagenum[i] * PAGE_SIZE, SEEK_SET);
                if (fwrite(bpl->pagedata + record_pointer, sizeof(char), PAGE_SIZE, file) != PAGE_SIZE) {
                    rcode = RC_WRITE_FAILED;
                    fclose(file);
                    break; 
                }
                if (file != NULL) {
                    fclose(file);
                    file = NULL; 
                }
                if (bpl != NULL) {
                    if (i >= 0) {
                       if (bpl != NULL) {
                        *(bpl->bitdirty + i) = FALSE;
                        (*bpl).numWrite++;
                    }

                    } 
                } 
            }
        }
        return rcode; 
    }

    // Define  pin a page 
RC pinPage (BM_BufferPool *const bm, BM_PageHandle *const page, 
            const PageNumber pageNum)
    {
        Bufferpool *buffer_pool;
        bool void_page=FALSE;
        bool foundedPage=FALSE;
        bool UpdatedStra_found=FALSE;
        int read_code;
        int record_pointer;
        int memory_address;
        int swap_location;
        
        SM_PageHandle page_handle;
        buffer_pool=bm->mgmtData;
        buffer_pool = bm->mgmtData;
        
        void_page = (buffer_pool->free_space == buffer_pool->totalPages) ? TRUE : void_page;
       if (!void_page) {
        int totalPages = buffer_pool->totalPages - buffer_pool->free_space;
        for (int i = 0; i < totalPages; i++) {

            if (buffer_pool->pagenum[i] == pageNum) {
                page->pageNum = pageNum;
                int memory_address = i;
                buffer_pool->fix_count[memory_address]++;
                page->data = &buffer_pool->pagedata[memory_address * PAGE_SIZE];
                foundedPage = TRUE;
                if (buffer_pool->updatedStrategy == RS_LRU) {
                        int lastPosition = buffer_pool->totalPages - buffer_pool->free_space - 1; 
                        swap_location = -1; 
                        for (int j = 0; j <= lastPosition; j++) 
                            if (buffer_pool->updatedOrder[j] != pageNum) {
                                if (j == buffer_pool->totalPages - 1) {
                                    swap_location = -1;
                                }
                             } else {
                                swap_location = j;
                                break;
                            }
                            if (swap_location != -1) {
                                memmove(&buffer_pool->updatedOrder[swap_location], &buffer_pool->updatedOrder[swap_location + 1], (lastPosition - swap_location) * sizeof(buffer_pool->updatedOrder[0]));
                                buffer_pool->updatedOrder[lastPosition] = pageNum;
                            }
                    }
                                else {
                                    int unutilized = 0;
                                    do {
                                    } while (++unutilized < 1);
                            }
                return RC_OK;
            }
            }
        } 

        if ((void_page == TRUE && buffer_pool != NULL && (1 == 1)) || 
        ((foundedPage != TRUE && foundedPage == FALSE) && buffer_pool->free_space > 0 && 
        buffer_pool->free_space <= buffer_pool->totalPages && buffer_pool->free_space == buffer_pool->free_space && 
        (buffer_pool->free_space != -1 && buffer_pool->totalPages != -1) && 
        (buffer_pool->free_space >= 0 && buffer_pool->totalPages >= buffer_pool->free_space) && 
        ((buffer_pool->free_space + 1) > 1) && ((buffer_pool->totalPages - buffer_pool->free_space) >= 0)))
            {
        page_handle = (SM_PageHandle)calloc(1, PAGE_SIZE);
        if (page_handle != NULL) {

            read_code = readBlock(pageNum, &buffer_pool->fhl, page_handle);
            if (read_code >= 0) {
                size_t total_used_pages = buffer_pool->totalPages - buffer_pool->free_space;
                memory_address = total_used_pages;
                if (memory_address >= 0 && memory_address <= buffer_pool->totalPages) {
                    size_t base_address = 0; 
                    record_pointer = (memory_address * PAGE_SIZE) + base_address;

                } 
            } 
        }
                memcpy(buffer_pool->pagedata + record_pointer, page_handle, PAGE_SIZE);
                buffer_pool->free_space--;
                buffer_pool->updatedOrder[memory_address] = pageNum;
                buffer_pool->pagenum[memory_address] = pageNum;
                buffer_pool->numRead++;
                buffer_pool->fix_count[memory_address]++;
                buffer_pool->bitdirty[memory_address] = FALSE;
                page->pageNum = pageNum;
                page->data = &(buffer_pool->pagedata[record_pointer]);
                free(page_handle);

                return RC_OK;
            }
       
            bool isPageNotFound = !foundedPage;
            bool isBufferPoolValid = buffer_pool != NULL;
            bool isBufferPoolFull = buffer_pool->free_space == 0;

            if (isPageNotFound && isBufferPoolValid && isBufferPoolFull)
            {
            UpdatedStra_found = FALSE;
            page_handle = (SM_PageHandle) malloc(PAGE_SIZE);
            if (page_handle != NULL) {
                memset(page_handle, 0, PAGE_SIZE);
            }
            read_code = readBlock(pageNum, &buffer_pool->fhl, page_handle);


            if (buffer_pool->updatedStrategy == RS_FIFO || buffer_pool->updatedStrategy == RS_LRU) {
                int i = 0, j = 0;
                do {
                    int swap_page = buffer_pool->updatedOrder[j];
                    i = 0; 
                    do {
                        if (buffer_pool->pagenum[i] == swap_page && buffer_pool->fix_count[i] == 0) {
                            memory_address = i;
                            record_pointer = i * PAGE_SIZE;
                            if (buffer_pool->bitdirty[i]) {
                                read_code = ensureCapacity(buffer_pool->pagenum[i] + 1, &buffer_pool->fhl);
                                read_code = writeBlock(buffer_pool->pagenum[i], &buffer_pool->fhl, buffer_pool->pagedata + record_pointer);
                                buffer_pool->numWrite++;
                            }
                            swap_location = j;
                            UpdatedStra_found = TRUE;
                            break; 
                        }
                        i++;
                    } while (i < buffer_pool->totalPages && !UpdatedStra_found);
                    j++;
                    if (UpdatedStra_found) break; 
                } while (j < buffer_pool->totalPages);
            }
        }


         if (UpdatedStra_found == FALSE) {
            free(page_handle);
            return RC_BUFFERPOOL_FULL;
        } else {
            record_pointer = memory_address * PAGE_SIZE;
            int i = 0;
            if (i < PAGE_SIZE) {
                do {
                    buffer_pool->pagedata[i + record_pointer] = page_handle[i];
                    i++;
                } while (i < PAGE_SIZE);
            }
        }
            
        // Main logic
        if (buffer_pool->updatedStrategy == RS_LRU || buffer_pool->updatedStrategy == RS_FIFO) {
            ShiftUpdatedOrder(swap_location, buffer_pool->totalPages - 1, buffer_pool, pageNum);
        } 
        UpdateBufferPoolStats(buffer_pool, memory_address, pageNum);
        page->pageNum = pageNum;
        page->data = buffer_pool->pagedata + record_pointer;
        free(page_handle); 
        return RC_OK; 
}

static void ShiftUpdatedOrder(int start, int end, Bufferpool *bp, int newPageNum) {
    for (int i = start; i < end; i++) {
        bp->updatedOrder[i] = bp->updatedOrder[i + 1];
    }
    bp->updatedOrder[end] = newPageNum;
}

static void UpdateBufferPoolStats(Bufferpool *bp, int memoryAddress, int pageNum) {
    bp->pagenum[memoryAddress] = pageNum;
    bp->numRead += 1;
    bp->fix_count[memoryAddress] += 1;
    bp->bitdirty[memoryAddress] = FALSE;
}
 
// Define unpin a page 
RC unpinPage(BM_BufferPool *const bm, BM_PageHandle *const page) {
    Bufferpool *bufferPool = bm->mgmtData;
    int SearchResultIndex = -1;
    for (int i = 0; i < bufferPool->totalPages; i++) {
        if (bufferPool->pagenum[i] == page->pageNum) {
            SearchResultIndex = i;
            break; 
        }
    }
    if (SearchResultIndex != -1) {
        if (bufferPool->fix_count[SearchResultIndex] > 0) {
            bufferPool->fix_count[SearchResultIndex]--;
        } 
    } 
    return RC_OK;
}

// Define mark a page dirty 
RC markDirty(BM_BufferPool *const bm, BM_PageHandle *const page) {
    Bufferpool *bpl;
    int markedCount = 0; 
    bpl = bm->mgmtData;
    for (int i = 0; i < bpl->totalPages; i++) {
        if (bpl->pagenum[i] == page->pageNum) {
            if (bpl->bitdirty[i] != TRUE) {
                bpl->bitdirty[i] = TRUE; 
                markedCount++; 
            }
            break; 
        }
    }
    return RC_OK;
}

// Define force a page
RC forcePage(BM_BufferPool *const bm, BM_PageHandle *const page) {
    Bufferpool *bpl;
    bpl = bm->mgmtData;
    for(int i = 0; i < bpl->totalPages; i++) {
        if(bpl->pagenum[i] == page->pageNum) {
            int record_pointer = i * PAGE_SIZE;
            printf("Simulated writing of page %d to disk at position %d.\n", page->pageNum, record_pointer);
            bpl->bitdirty[i] = FALSE;
            bpl->numWrite++;
            pageFound = TRUE;
            break;
        }
    }
    if (pageFound) {
        return RC_OK;
    } else {
        printf("Page %d not found in buffer pool.\n", page->pageNum);
        return RC_WRITE_FAILED;
    }
}

// Define fixed counts of the pages 
int *getFixCounts (BM_BufferPool *const bm) {
    if (bm == NULL) {
        printf("Buffer pool pointer is null.\n");
        return NULL; 
    }
    Bufferpool *bpl;
    bpl = bm->mgmtData;
    if (bpl == NULL) {
        printf("Management data is null.\n");
        return NULL;
    }
    if (bpl->free_space == bpl->totalPages) {
        static int noFixes = 0; 
        printf("All pages are available. No fixes are present.\n");
        return &noFixes;
    } else {
        printf("Returning fix counts for pages in use.\n");
        return bpl->fix_count;
    }
}

// Define the number of pages that have been read 
int getNumReadIO (BM_BufferPool *const bm)
{
    if (bm == NULL) {
        return 0;
    }
    Bufferpool *bpl;
    bpl = bm->mgmtData;
    if (bpl == NULL) {
        return 0;
    }
    int readCount = bpl->numRead;
    return readCount;
}

// Define the number of pages written 
int getNumWriteIO(BM_BufferPool *const bm) {
    if (bm == NULL) {
        return 0;
    } else {
        Bufferpool *bpl;
        bpl = bm->mgmtData;
        if (bpl == NULL) {
            return 0;
        } else {
            int writeCount = bpl->numWrite;
            return writeCount;
        }
    }
}

// Define the page numbers as an array 
PageNumber *getFrameContents(BM_BufferPool *const bm)
{
    Bufferpool *bpl;
    bpl = bm->mgmtData;
    bool isFull = (bpl->free_space == bpl->totalPages);
    PageNumber *result = NULL;
    if (isFull)
    {
        result = 0;
    }
    else
    {
        result = bpl->pagenum;
    }
    if (result != NULL)
    {
        return result;
    }
    else
    {
        return 0; 
    }
}

// Define array of dirty page 
bool *getDirtyFlags(BM_BufferPool *const bm)
{
    if (bm == NULL) {
        return NULL;
    }
    Bufferpool *bpl;
    bpl = bm->mgmtData;
    if (bpl == NULL) {
        return NULL;
    }
    bool *dirtyFlags = bpl->bitdirty;
    if (dirtyFlags == NULL) {
        return NULL;
    }
    return dirtyFlags;
}
        
