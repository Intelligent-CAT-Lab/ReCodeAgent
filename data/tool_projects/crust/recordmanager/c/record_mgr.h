#ifndef RECORD_MGR_H
#define RECORD_MGR_H

#include "dberror.h"
#include "expr.h"
#include "tables.h"
#include "buffer_mgr.h"
// Bookkeeping for scans
typedef struct RM_ScanHandle
{
	RM_TableData *rel;
	void *mgmtData;
} RM_ScanHandle;




/*Structure to store the table manager information*/
typedef struct TableManager
{
     int totalTuples;
     int recSize;
     int firstFreePageNum;
     int firstFreeSlotNum;
     int firstDataPageNum;
     BM_BufferPool *bufferManagerPtr;
     BM_PageHandle *pageHandlePtr;
}TableManager;

/*Structure of the Page Header*/
typedef struct PageHeader
{
    char pageIdentifier;
    int totalTuples;
    int freeSlotCnt;
    int nextFreeSlotInd;
    int prevFreePageIndex;
    int nextFreePageIndex;
    int prevDataPageIndex;
    int nextDataPageIndex;
}PageHeader;

/*Structure to store the table manager information*/
typedef struct ScanManager
{
     int totalEntries;
     int scanIndex;
     int currentPageNum;
     int currentSlotNum;
     Expr *conditionExpression;
     BM_PageHandle *scanPageHandlePtr;
}ScanManager;

// table and manager
extern RC initRecordManager (void *mgmtData);
extern RC shutdownRecordManager ();
extern RC createTable (char *name, Schema *schema);
extern RC openTable (RM_TableData *rel, char *name);
extern RC closeTable (RM_TableData *rel);
extern RC deleteTable (char *name);
extern int getNumTuples (RM_TableData *rel);

// handling records in a table
extern RC insertRecord (RM_TableData *rel, Record *record);
extern RC deleteRecord (RM_TableData *rel, RID id);
extern RC updateRecord (RM_TableData *rel, Record *record);
extern RC getRecord (RM_TableData *rel, RID id, Record *record);

// scans
extern RC startScan (RM_TableData *rel, RM_ScanHandle *scan, Expr *cond);
extern RC next (RM_ScanHandle *scan, Record *record);
extern RC closeScan (RM_ScanHandle *scan);

// dealing with schemas
extern int getRecordSize (Schema *schema);
extern Schema *createSchema (int numAttr, char **attrNames, DataType *dataTypes, int *typeLength, int keySize, int *keys);
extern RC freeSchema (Schema *schema);

// dealing with records and attribute values
extern RC createRecord (Record **record, Schema *schema);
extern RC freeRecord (Record *record);
extern RC getAttr (Record *record, Schema *schema, int attrNum, Value **value);
extern RC setAttr (Record *record, Schema *schema, int attrNum, Value *value);

#endif // RECORD_MGR_H
