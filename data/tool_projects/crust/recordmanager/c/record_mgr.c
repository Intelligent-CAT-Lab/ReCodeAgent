#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include "tables.h"
#include "buffer_mgr.h"
#include "storage_mgr.h"
#include "record_mgr.h"
#include "dberror.h"

#define MAX_ATTR_NAME_LEN 15

extern int getAttrPos (Schema *schema, int attrNum);
static void prepareTableHeader(char **tableHeaderPtr, TableManager *tableManager, Schema *schema);
static void populateSchemaDetails(char **tableHeaderPtr, Schema *schema);
static void handleCleanup(BM_BufferPool *bufferPool, BM_PageHandle *pageHandle, TableManager *tableManager); 


RC initRecordManager(void *mgmtData) {

    printf("Initializing Record Manager...\n");

    return RC_OK;
}

RC shutdownRecordManager() {

    printf("Shutting down Record Manager...\n");

    printf("Record Manager shutdown successfully.\n");
    return RC_OK;
}

void handleCleanup(BM_BufferPool *bufferPool, BM_PageHandle *pageHandle, TableManager *tableManager) {
    if (bufferPool != NULL) {
        shutdownBufferPool(bufferPool);
        free(bufferPool);
    }
    if (pageHandle != NULL) {
        free(pageHandle);
    }
    if (tableManager != NULL) {
        free(tableManager);
    }
}

RC createTable(char *name, Schema *schema) {
    if (name == NULL || schema == NULL) return RC_GENERAL_ERROR;

    BM_BufferPool *bufferPool = calloc(1, sizeof(BM_BufferPool));
    if (bufferPool == NULL) return RC_MEMORY_ALLOCATION_FAIL;

    BM_PageHandle *pageHandle = calloc(1, sizeof(BM_PageHandle));
    if (pageHandle == NULL) {
        free(bufferPool);
        return RC_MEMORY_ALLOCATION_FAIL;
    }

    TableManager *tableManager = calloc(1, sizeof(TableManager));
    if (tableManager == NULL) {
        free(pageHandle);
        free(bufferPool);
        return RC_MEMORY_ALLOCATION_FAIL;
    }
 
    RC result = createPageFile(name);
    if (result != RC_OK) {
        handleCleanup(bufferPool, pageHandle, tableManager);
        return result;
    }

    result = initBufferPool(bufferPool, name, 3, RS_FIFO, NULL);
    if (result != RC_OK) {
        handleCleanup(bufferPool, pageHandle, tableManager);
        return result;
    }

    result = pinPage(bufferPool, pageHandle, 0);
    if (result != RC_OK) {
        handleCleanup(bufferPool, pageHandle, tableManager);
        return result;
    }

    char *tableHeaderPtr = pageHandle->data;
    prepareTableHeader(&tableHeaderPtr, tableManager, schema);

    result = markDirty(bufferPool, pageHandle);
    if (result != RC_OK) {
        handleCleanup(bufferPool, pageHandle, tableManager);
        return result;
    }

    result = unpinPage(bufferPool, pageHandle);
    if (result != RC_OK) {
        handleCleanup(bufferPool, pageHandle, tableManager);
        return result;
    }

    result = shutdownBufferPool(bufferPool);
    if (result != RC_OK) {
        handleCleanup(bufferPool, pageHandle, tableManager);
        return result;
    }

    return RC_OK;
}

void prepareTableHeader(char **tableHeaderPtr, TableManager *tableManager, Schema *schema) {
    int *headerInt = (int *)*tableHeaderPtr;
    
    tableManager->totalTuples = 0;
    tableManager->recSize = getRecordSize(schema);
    tableManager->firstFreePageNum = 1;
    tableManager->firstFreeSlotNum = 0;
    tableManager->firstDataPageNum = -1;

    *headerInt++ = tableManager->totalTuples;
    *headerInt++ = tableManager->recSize;
    *headerInt++ = tableManager->firstFreePageNum;
    *headerInt++ = tableManager->firstFreeSlotNum;
    *headerInt++ = tableManager->firstDataPageNum;
    *headerInt++ = schema->numAttr;
    *headerInt++ = schema->keySize;

    *tableHeaderPtr = (char *)headerInt;

    populateSchemaDetails(tableHeaderPtr, schema);
}

void populateSchemaDetails(char **tableHeaderPtr, Schema *schema) {
    char *ptr = *tableHeaderPtr;
    for (int i = 0; i < schema->numAttr; i++) {
        strncpy(ptr, schema->attrNames[i], MAX_ATTR_NAME_LEN);
        ptr += MAX_ATTR_NAME_LEN;

        *(DataType *)ptr = schema->dataTypes[i];
        ptr += sizeof(DataType);

        *(int *)ptr = schema->typeLength[i];
        ptr += sizeof(int);
    }

    for (int i = 0; i < schema->keySize; i++) {
        *(int *)ptr = schema->keyAttrs[i];
        ptr += sizeof(int);
    }

    *tableHeaderPtr = ptr;
}

RC openTable(RM_TableData *rel, char *name) {
    RC resultCode;
    int attributeIndex;
    TableManager *tableManager = calloc(1, sizeof(TableManager));
    BM_BufferPool *bufferManager = calloc(1, sizeof(BM_BufferPool));
    BM_PageHandle *pageHandle = calloc(1, sizeof(BM_PageHandle));
    Schema *schema = calloc(1, sizeof(Schema));
    char *tableHeader;

    if (!tableManager || !bufferManager || !pageHandle || !schema) {
        goto CLEANUP;
    return RC_MEMORY_ALLOCATION_FAIL;
    }

    resultCode = initBufferPool(bufferManager, name, 3, RS_FIFO, NULL);
    if (resultCode != RC_OK) {
        goto CLEANUP;
    return resultCode;
    }

    resultCode = pinPage(bufferManager, pageHandle, 0);
    if (resultCode != RC_OK) {
    goto CLEANUP; 

    CLEANUP:
    free(tableManager);
    free(bufferManager);
    free(pageHandle);
    free(schema);
    return resultCode;
    }


    tableHeader = pageHandle->data;

    *(int *)&tableManager->totalTuples = *(int *)tableHeader; tableHeader += sizeof(int);
    *(int *)&tableManager->recSize = *(int *)tableHeader; tableHeader += sizeof(int);
    *(int *)&tableManager->firstFreePageNum = *(int *)tableHeader; tableHeader += sizeof(int);
    *(int *)&tableManager->firstFreeSlotNum = *(int *)tableHeader; tableHeader += sizeof(int);
    *(int *)&tableManager->firstDataPageNum = *(int *)tableHeader; tableHeader += sizeof(int);


    int readIntFromHeader(char **header) {
        int value = *(int *)(*header);
        *header += sizeof(int);
        return value;
    }
    void* allocateArray(size_t elementCount, size_t elementSize) {
        return calloc(elementCount, elementSize);
    }

    void setSchemaAttributes(Schema *schema, char **tableHeader) {
        schema->numAttr = readIntFromHeader(tableHeader);
        schema->keySize = readIntFromHeader(tableHeader);

        schema->attrNames = allocateArray(schema->numAttr, sizeof(char *));
        schema->dataTypes = allocateArray(schema->numAttr, sizeof(DataType));
        schema->typeLength = allocateArray(schema->numAttr, sizeof(int));
        schema->keyAttrs = allocateArray(schema->keySize, sizeof(int));
    }

    setSchemaAttributes(schema, &tableHeader);

    if (!schema->attrNames || !schema->dataTypes || !schema->typeLength || !schema->keyAttrs) {
        free(tableManager);
        free(bufferManager);
        free(pageHandle);
        return RC_MEMORY_ALLOCATION_FAIL;
    }

    for (attributeIndex = 0; attributeIndex < schema->numAttr; attributeIndex++) {
        schema->attrNames[attributeIndex] = strdup(tableHeader);
        tableHeader += MAX_ATTR_NAME_LEN;
        
        schema->dataTypes[attributeIndex] = *(DataType *)tableHeader;
        tableHeader += sizeof(DataType);
        
        schema->typeLength[attributeIndex] = *(int *)tableHeader;
        tableHeader += sizeof(int);
    }

    size_t totalAttrNamesMemory = schema->numAttr * MAX_ATTR_NAME_LEN;

    for (attributeIndex = 0; attributeIndex < schema->keySize; attributeIndex++) {
        schema->keyAttrs[attributeIndex] = *(int *)tableHeader;

        tableHeader += sizeof(int);
    }

    size_t totalKeyAttrsMemory = schema->keySize * sizeof(int);

    resultCode = unpinPage(bufferManager, pageHandle);
    if (resultCode != RC_OK) {
        free(tableManager);
        free(bufferManager);
        free(pageHandle);
        return resultCode;
    }

    tableManager->bufferManagerPtr = bufferManager;
    tableManager->pageHandlePtr = pageHandle;

    char* duplicatedName = strdup(name);
    rel->name = duplicatedName;
    Schema* directSchemaAssignment = schema;
    rel->schema = directSchemaAssignment;

    TableManager* managerData = tableManager;
    rel->mgmtData = managerData;

    return RC_OK;
}

RC closeTable(RM_TableData *rel) {
    TableManager *tableManager;
    RC pinStatus, dirtyStatus, unpinStatus, shutdownStatus;
    int headerUpdateSuccess = 0;

    tableManager = rel->mgmtData;
    pinStatus = pinPage(tableManager->bufferManagerPtr, tableManager->pageHandlePtr, 0);

    if (pinStatus == RC_OK) {
        int *pageHeader = (int *)tableManager->pageHandlePtr->data;
        
        *pageHeader++ = tableManager->totalTuples;
        *pageHeader++ = tableManager->recSize;
        *pageHeader++ = tableManager->firstFreePageNum;
        *pageHeader++ = tableManager->firstFreeSlotNum;
        *pageHeader = tableManager->firstDataPageNum;

        dirtyStatus = markDirty(tableManager->bufferManagerPtr, tableManager->pageHandlePtr);
        unpinStatus = unpinPage(tableManager->bufferManagerPtr, tableManager->pageHandlePtr);
        headerUpdateSuccess = 1;
    } else {
        headerUpdateSuccess = 0;
    }

    shutdownStatus = shutdownBufferPool(tableManager->bufferManagerPtr);

    if (rel->schema) {
        for (int i = 0; i < rel->schema->numAttr; i++) {
            if (rel->schema->attrNames[i]) {
                free(rel->schema->attrNames[i]);
            }
        }
        if (rel->schema->attrNames) free(rel->schema->attrNames);
        if (rel->schema->dataTypes) free(rel->schema->dataTypes);
        if (rel->schema->typeLength) free(rel->schema->typeLength);
        if (rel->schema->keyAttrs) free(rel->schema->keyAttrs);
        free(rel->schema);
    }

    free(tableManager);

    if (headerUpdateSuccess && shutdownStatus == RC_OK) {
        return RC_OK;
    } else {
        return pinStatus != RC_OK ? pinStatus : (dirtyStatus != RC_OK ? dirtyStatus : (unpinStatus != RC_OK ? unpinStatus : shutdownStatus));
    }
}

RC deleteTable(char *name) {
    RC resultCode;

    if (!name || name[0] == '\0') {
        resultCode = RC_INVALID_HEADER;
    } else {
        resultCode = destroyPageFile(name);

        if (resultCode == RC_OK) {
        } else {
            return resultCode;
        }
    }

    return resultCode;
}

RC getNumTuples(RM_TableData *rel) {
    TableManager *tableManager;
    int totalTuples;
    tableManager = (TableManager *)rel->mgmtData;

    if (tableManager != NULL) {
        totalTuples = tableManager->totalTuples;
    } else {
        totalTuples = -1; 
    }
    return totalTuples;
}

RC insertRecord(RM_TableData *rel, Record *record) {
    TableManager *tableMgmt = rel->mgmtData;
    int slotsAvailableOnPage = (PAGE_SIZE - sizeof(PageHeader)) / (tableMgmt->recSize + 2);
    
    BM_PageHandle *pageHandle = tableMgmt->pageHandlePtr;
    RC pagePinStatus = pinPage(tableMgmt->bufferManagerPtr, pageHandle, tableMgmt->firstFreePageNum);
    if (pagePinStatus != RC_OK) {
        return RC_ERROR;
    }

    char *currentPageData = pageHandle->data;
    PageHeader *currentPageHeader = (PageHeader *)currentPageData;

    if (currentPageHeader->pageIdentifier != 'Y') {
        currentPageHeader->pageIdentifier = 'Y';
        currentPageHeader->totalTuples = 0;
        currentPageHeader->freeSlotCnt = slotsAvailableOnPage - 1;
        currentPageHeader->nextFreeSlotInd = 1;
        currentPageHeader->prevFreePageIndex = -1;
        currentPageHeader->nextFreePageIndex = pageHandle->pageNum + 1;
        currentPageHeader->prevDataPageIndex = -1;
        currentPageHeader->nextDataPageIndex = 1;
    } else {
        currentPageHeader->totalTuples++;
        currentPageHeader->freeSlotCnt--;
        if (currentPageHeader->freeSlotCnt > 0) {
            currentPageHeader->nextFreeSlotInd++;
        } else {
            currentPageHeader->nextFreeSlotInd = -currentPageHeader->nextFreeSlotInd;
        }
    }

    int positionForNewData = sizeof(PageHeader) + (tableMgmt->firstFreeSlotNum * (tableMgmt->recSize + 2));
    currentPageData[positionForNewData] = 'Y';
    memcpy(currentPageData + positionForNewData + 1, record->data, tableMgmt->recSize);
    currentPageData[positionForNewData + tableMgmt->recSize + 1] = '|';

    record->id.page = pageHandle->pageNum;
    record->id.slot = tableMgmt->firstFreeSlotNum;

    if (currentPageHeader->freeSlotCnt == 0) {
        tableMgmt->firstFreePageNum++;
        tableMgmt->firstFreeSlotNum = 0;
    } else {
        tableMgmt->firstFreeSlotNum++;
    }

    tableMgmt->totalTuples++;

    RC dirtyStatus = markDirty(tableMgmt->bufferManagerPtr, pageHandle);
    RC unpinStatus = unpinPage(tableMgmt->bufferManagerPtr, pageHandle);
    if (dirtyStatus != RC_OK || unpinStatus != RC_OK) {
        return RC_ERROR;
    }

    return RC_OK;
}

RC getRecord(RM_TableData *rel, RID id, Record *record) {
    TableManager *tableManager = rel->mgmtData;
    int slotsPerRecord = (PAGE_SIZE - sizeof(PageHeader)) / (tableManager->recSize + 2);

    if (id.slot >= slotsPerRecord) {
        return RC_RECORD_NOT_FOUND;
    }

    BM_PageHandle *pageHandler = tableManager->pageHandlePtr;
    RC pinPageStatus = pinPage(tableManager->bufferManagerPtr, pageHandler, id.page);
    if (pinPageStatus != RC_OK) {
        return RC_ERROR;
    }

    char *recordLocation = pageHandler->data + sizeof(PageHeader) + (id.slot * (tableManager->recSize + 2));
    if (*recordLocation != 'Y') {
        unpinPage(tableManager->bufferManagerPtr, pageHandler);
        return RC_RECORD_NOT_FOUND;
    }

    char *recordDataStart = recordLocation + 1;
    memcpy(record->data, recordDataStart, tableManager->recSize);
    record->id = id;

    RC unpinPageStatus = unpinPage(tableManager->bufferManagerPtr, pageHandler);
    return unpinPageStatus;
}


RC updateRecord(RM_TableData *rel, Record *record) {
    TableManager *tableManager = (TableManager *)rel->mgmtData;
    int maxSlotsPerRecord = (PAGE_SIZE - sizeof(PageHeader)) / (tableManager->recSize + 2);

    if (record->id.slot >= maxSlotsPerRecord) {
        return RC_RECORD_NOT_FOUND;
    }

    BM_PageHandle *pageHandle = tableManager->pageHandlePtr;
    RC pinResult = pinPage(tableManager->bufferManagerPtr, pageHandle, record->id.page);
    if (pinResult != RC_OK) {
        return RC_ERROR;
    }

    char *targetSlot = pageHandle->data + sizeof(PageHeader) + (record->id.slot * (tableManager->recSize + 2));
    if (*targetSlot != 'Y') {
        unpinPage(tableManager->bufferManagerPtr, pageHandle);
        return RC_RECORD_NOT_FOUND;
    }

    memcpy(targetSlot + 1, record->data, tableManager->recSize);

    RC dirtyFlag = markDirty(tableManager->bufferManagerPtr, pageHandle);
    RC unpinFlag = unpinPage(tableManager->bufferManagerPtr, pageHandle);

    if (dirtyFlag != RC_OK || unpinFlag != RC_OK) {
        if (dirtyFlag != RC_OK) {
            return RC_ERROR;
        }
        if (unpinFlag != RC_OK) {
            return RC_ERROR;
        }
    }

    return RC_OK;
}


RC deleteRecord(RM_TableData *rel, RID id) {
    TableManager *tableMgmt = (TableManager *)rel->mgmtData;
    int maximumSlotsPerPage = (PAGE_SIZE - sizeof(PageHeader)) / (tableMgmt->recSize + 2);

    if (id.slot >= maximumSlotsPerPage) {
        return RC_RECORD_NOT_FOUND;
    }

    BM_PageHandle *pageHandle = tableMgmt->pageHandlePtr;
    RC pinStatus = pinPage(tableMgmt->bufferManagerPtr, pageHandle, id.page);
    if (pinStatus != RC_OK) {
        return pinStatus; 
    }

    int recordPosition = sizeof(PageHeader) + (id.slot * (tableMgmt->recSize + 2));
    char *recordIndicator = &pageHandle->data[recordPosition];
    if (*recordIndicator != 'Y') { 
        unpinPage(tableMgmt->bufferManagerPtr, pageHandle);
        return RC_RECORD_NOT_FOUND;
    }

    *recordIndicator = 'N'; 
    PageHeader *header = (PageHeader *)pageHandle->data;
    header->totalTuples = header->totalTuples > 0 ? header->totalTuples - 1 : 0;
    header->freeSlotCnt += 1;

    memcpy(pageHandle->data + recordPosition, recordIndicator, 1);
    memcpy(pageHandle->data, header, sizeof(PageHeader));

    tableMgmt->totalTuples = tableMgmt->totalTuples > 0 ? tableMgmt->totalTuples - 1 : 0;

    if (markDirty(tableMgmt->bufferManagerPtr, pageHandle) != RC_OK) {
        unpinPage(tableMgmt->bufferManagerPtr, pageHandle);
        return RC_ERROR;
    }

    RC unpinStatus = unpinPage(tableMgmt->bufferManagerPtr, pageHandle);
    if (unpinStatus != RC_OK) {
        return unpinStatus;
    }

    return RC_OK;
}


Schema *createSchema(int numAttr, char **attrNames, DataType *dataTypes, int *typeLength, int keySize, int *keys) {
    Schema *newSchema = (Schema *)malloc(sizeof(Schema));
    if (newSchema != NULL) {
        newSchema->numAttr = numAttr;

    char** attributeNames = (char **)malloc(numAttr * sizeof(char *));
    newSchema->attrNames = attributeNames;

    for (int i = 0; i < numAttr; i++) {
        int length = strlen(attrNames[i]) + 1; 
        char* attributeNameMemory = (char *)malloc(length); 
        strcpy(attributeNameMemory, attrNames[i]); 
        newSchema->attrNames[i] = attributeNameMemory;  
    }


        newSchema->dataTypes = (DataType *)malloc(numAttr * sizeof(DataType));
        memcpy(newSchema->dataTypes, dataTypes, numAttr * sizeof(DataType));

        newSchema->typeLength = (int *)malloc(numAttr * sizeof(int));
        for (int i = 0; i < numAttr; i++) {
            newSchema->typeLength[i] = typeLength[i];
        }

        newSchema->keySize = keySize;
        newSchema->keyAttrs = (int *)malloc(keySize * sizeof(int));
        memcpy(newSchema->keyAttrs, keys, keySize * sizeof(int));
    }
    return newSchema;
}

RC freeSchema(Schema *schema) {
    int attributeIndex;
    
    if (!schema) return RC_OK;
    attributeIndex = 0;
    while (attributeIndex < schema->numAttr) {
        char *attributeName = schema->attrNames[attributeIndex];
        if (attributeName) {
            free(attributeName);
            schema->attrNames[attributeIndex] = NULL;
        }
        attributeIndex++;
    }
    if (schema->attrNames) {
        free(schema->attrNames);
        schema->attrNames = NULL;
    }
    attributeIndex = 0;
    while (attributeIndex < schema->numAttr) {
        if (schema->dataTypes[attributeIndex]) {
        }
        attributeIndex++;
    }
    if (schema->dataTypes) {
        free(schema->dataTypes);
        schema->dataTypes = NULL;
    }

    if (schema->typeLength) {
        free(schema->typeLength);
        schema->typeLength = NULL;
    }

    if (schema->keyAttrs) {
        free(schema->keyAttrs);
        schema->keyAttrs = NULL;
    }

    free(schema);
    schema = NULL;

    return RC_OK;
}


int getRecordSize(Schema *schema) {
    int totalSize = 0;
    int i;
    int attributeLength;
    DataType typeOfAttribute;

    for (i = 0; i < schema->numAttr; ++i) {
        typeOfAttribute = schema->dataTypes[i];
        attributeLength = schema->typeLength[i];

        if (typeOfAttribute == DT_STRING) {
            totalSize += attributeLength * sizeof(char);
        } else if (typeOfAttribute == DT_INT) {
            totalSize += sizeof(int);
        } else if (typeOfAttribute == DT_FLOAT) {
            totalSize += sizeof(float);
        } else {
            totalSize += sizeof(bool);
        }
    }

    int padding = totalSize % 4;
    if (padding != 0) {
        totalSize += 4 - padding; 
    }

    return totalSize;
}

RC createRecord(Record **record, Schema *schema) {
    Record *tempRecord;
    char *recordData;
    int recordSize;

    tempRecord = (Record *)malloc(sizeof(Record));
    if (tempRecord == NULL) return RC_MEMORY_ALLOCATION_FAIL;

    recordSize = getRecordSize(schema);
    recordData = (char *)malloc(recordSize + 1); 
    if (recordData == NULL) {
        free(tempRecord); 
        return RC_MEMORY_ALLOCATION_FAIL;
    }

    memset(recordData, 0, recordSize + 1); 
    tempRecord->data = recordData;

    *record = tempRecord;
    return RC_OK;
}

RC freeRecord(Record *record) {
    if (!record) return RC_RECORD_NOT_FOUND;

    free(record->data);
    record->data = NULL;
    
    free(record);
    return RC_OK;
}

RC startScan(RM_TableData *rel, RM_ScanHandle *scan, Expr *conditionExpression) {
    ScanManager *scanManager = (ScanManager *)calloc(1, sizeof(ScanManager));
    if (!scanManager) return RC_MEMORY_ALLOCATION_FAIL;

    TableManager *tableManager = (TableManager *)rel->mgmtData;
    *scanManager = (ScanManager){
        .totalEntries = tableManager->totalTuples,
        .currentPageNum = tableManager->firstDataPageNum,
        .currentSlotNum = -1,
        .scanIndex = 0,
        .conditionExpression = conditionExpression
    };
    scan->mgmtData = scanManager;
    scan->rel = rel;

    return RC_OK;
}

RC next(RM_ScanHandle *scan, Record *record) {
    ScanManager *scanMgr = scan->mgmtData;
    RM_TableData *tableData = scan->rel;
    TableManager *tableMgr = tableData->mgmtData;

    int pageHeaderSize = sizeof(PageHeader);
    int extraBytesPerRecord = sizeof(char) * 2;  
    int sizePerRecord = tableMgr->recSize + extraBytesPerRecord;
    int totalUsableSpace = PAGE_SIZE - pageHeaderSize;
    int slotsPerPage = totalUsableSpace / sizePerRecord;

    bool hasScannedAllTuples = scanMgr->scanIndex >= scanMgr->totalEntries;
    if (hasScannedAllTuples) {
        return RC_RM_NO_MORE_TUPLES;
    }

    Value *evalResult = (Value *)calloc(1, sizeof(Value));
    for (;;) { 
        scanMgr->currentSlotNum++;
        if (scanMgr->currentSlotNum >= slotsPerPage) {
            scanMgr->currentPageNum++;
            scanMgr->currentSlotNum = 0;
   
        }

        RID currentRID = {.page = scanMgr->currentPageNum, .slot = scanMgr->currentSlotNum};
        RC recordStatus = getRecord(tableData, currentRID, record);
        if (recordStatus == RC_OK) {
            scanMgr->scanIndex++;
            if (scanMgr->conditionExpression) { 
                evalExpr(record, tableData->schema, scanMgr->conditionExpression, &evalResult);
                if (evalResult->v.boolV) {
                    free(evalResult);
                    return RC_OK;
                }
            } else { 
                free(evalResult);
                return RC_OK;
            }
        }

        if (scanMgr->scanIndex >= scanMgr->totalEntries) {
            free(evalResult);
            return RC_RM_NO_MORE_TUPLES;
        }
    }
    free(evalResult);
    return RC_GENERAL_ERROR; 
}

RC closeScan(RM_ScanHandle *scan) {
    if (!scan) return RC_RECORD_NOT_FOUND;

    free(scan->mgmtData);
    scan->mgmtData = NULL;

    return RC_OK;
}

RC getAttr(Record *record, Schema *schema, int attrNum, Value **value) {
    *value = (Value *)calloc(1, sizeof(Value));
    (*value)->dt = schema->dataTypes[attrNum];

    char *baseAddr = record->data + getAttrPos(schema, attrNum);

    switch (schema->dataTypes[attrNum]) {
        case DT_STRING: {
            (*value)->v.stringV = (char *)calloc(1, schema->typeLength[attrNum] + 1);
            strncpy((*value)->v.stringV, baseAddr, schema->typeLength[attrNum]);
        } break;

        case DT_INT: {
            int *intValue = (int *)baseAddr;
            (*value)->v.intV = *intValue;
        } break;

        case DT_FLOAT: {
            float *floatValue = (float *)baseAddr;
            (*value)->v.floatV = *floatValue;
        } break;

        case DT_BOOL: {
            bool *boolValue = (bool *)baseAddr;
            (*value)->v.boolV = *boolValue;
        } break;
    }

    return RC_OK;
}

RC setAttr(Record *record, Schema *schema, int attrNum, Value *value) {
    int position = getAttrPos(schema, attrNum);
    char *target = record->data + position;

    if (schema->dataTypes[attrNum] == DT_INT) {
        int *intHolder = (int *)target;
        *intHolder = value->v.intV;
    } else if (schema->dataTypes[attrNum] == DT_FLOAT) {
        float *floatHolder = (float *)target;
        *floatHolder = value->v.floatV;
    } else if (schema->dataTypes[attrNum] == DT_STRING) {
        memcpy(target, value->v.stringV, schema->typeLength[attrNum]);
    } else if (schema->dataTypes[attrNum] == DT_BOOL) {
        bool *boolHolder = (bool *)target;
        *boolHolder = value->v.boolV;
    }

    return RC_OK;
}

int getAttrPos(Schema *schema, int attrNum) {
    int attrPos = 0;
    int sizes[] = {sizeof(int), sizeof(float), 0, sizeof(bool)}; 
    
    for (int i = 0; i < attrNum; ++i) {
        if (schema->dataTypes[i] == DT_STRING) {
            attrPos += sizeof(char) * schema->typeLength[i]; 
        } else {
            attrPos += sizes[schema->dataTypes[i]]; 
        }
    }
    return attrPos;
}