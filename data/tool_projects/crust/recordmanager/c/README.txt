___________________________________________________________________________________________________________________________________________________________________
___________________________________________________________________________________________________________________________________________________________________

                                                Record Manager Module for CS525 - Advanced Database Organization
___________________________________________________________________________________________________________________________________________________________________
___________________________________________________________________________________________________________________________________________________________________

* Overview
The Record Manager module is a crucial component of the coursework for the CS525 Advanced Database Organization. 
It facilitates direct interaction with database records, enabling operations such as creation, modification, deletion, and querying of records within tables. 
This module abstracts the complexities of disk-based storage by providing a higher-level interface for managing records, which are stored in tables with predefined schemas.

-------------------------------------------------------------------------------------------------------------------------------------------------------


* Definitions and Key Concepts
Record Manager
A subsystem in a database management system (DBMS) designed to handle the efficient storage, retrieval, modification, and deletion of records in a database.

-------------------------------------------------------------------------------------------------------------------------------------------------------


* Schema
Defines the structure of a table in a database, specifying the fields present in each record, their data types, and the constraints that apply to them.

-------------------------------------------------------------------------------------------------------------------------------------------------------


* Buffer Manager
An intermediary layer that manages the in-memory buffering of disk-based data, optimizing data access patterns and reducing the number of direct disk I/O operations.

-------------------------------------------------------------------------------------------------------------------------------------------------------


* Record ID (RID)
A unique identifier assigned to each record in a table, typically composed of a page number and a slot number within that page, enabling fast retrieval.

-------------------------------------------------------------------------------------------------------------------------------------------------------


* Building and Running
Ensure a C compiler is installed and navigate to the project's root directory. Use the Makefile to compile:

-------------------------------------------------------------------------------------------------------------------------------------------------------
* Commands to Run

``make``

``./test_assign3``

-------------------------------------------------------------------------------------------------------------------------------------------------------

* Functionality Overview
The Record Manager module, implemented in record_mgr.c, provides a comprehensive API for managing records in a database. Key functionalities include:

-------------------------------------------------------------------------------------------------------------------------------------------------------


* Table Management
1). initRecordManager: Initializes the record manager environment.
2). shutdownRecordManager: Cleans up and shuts down the record manager, ensuring all resources are released.
3). createTable: Creates a new table with a specified schema.
4). openTable: Opens an existing table for record operations.
5). closeTable: Closes an opened table, writing any unsaved changes to disk.
6). deleteTable: Removes a table and its data from the database.
7). Record Handling
8). insertRecord: Adds a new record to a table.
9). deleteRecord: Removes a record from a table based on its RID.
10). updateRecord: Modifies the contents of an existing record.
11). getRecord: Retrieves a record from a table using its RID.
12). Scanning
13). startScan: Initializes a scan operation on a table, optionally filtering records based on a specified condition.
14). next: Retrieves the next record that matches the scan condition.
15). closeScan: Finalizes a scan operation, releasing any allocated resources.
16). Schema and Attribute Management
17). createSchema: Defines a new schema for table creation.
18). freeSchema: Releases the resources associated with a schema.
19). createRecord: Allocates and initializes a new record based on a schema.
20). freeRecord: Deallocates a record.
21). getAttribute: Retrieves the value of a specified attribute from a record.
22). setAttribute: Sets the value of a specified attribute in a record.

-------------------------------------------------------------------------------------------------------------------------------------------------------

* Extensions and Optional Features
Implementations may include optional features such as support for NULL values, primary key constraints, and advanced scanning capabilities. 
Each feature enhances the module's functionality, providing a more robust and feature-complete record management system.