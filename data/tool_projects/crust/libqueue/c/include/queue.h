/**
 * @file queue.h
 * @brief Entry header file to the library.
 *
 * @author Saurav Pal (resyfer)
 * @bug No known bugs
 */

#ifndef __LIBQUEUE_H
#define __LIBQUEUE_H

/* libc */
#include <stdbool.h>
#include <sys/types.h>

/**
 * @mainpage
 * A queue data structure to which elements of any data type can be pushed
 * and popped from in a first in first out basis.
 */

/**
 * @brief Nodes in the queue.
 *
 * The building blocks of queue
 */
typedef struct queue_node_t {
	void* value; /*!< Value stored in the node */
	struct queue_node_t* next; /*!< Next node in the linked list */
} queue_node_t;

/**
 * @brief Queue instance
 *
 * Implemented as a singly linked list, such that elements get pushed behind
 * the tail, while elements get popped from head.
 */
typedef struct {
	u_int32_t size; /*!< Size of the queue */
	queue_node_t* head; /*!< First node in the linked list */
	queue_node_t* tail; /*!< Last node in the linked list */
} queue_t;

/**
 * @brief Initialize a queue
 *
 * @param queue A queue instance to initialize
 */
void queue_new(queue_t *queue);

/* CRUD */

/**
 * @brief Push elements to the back of the queue.
 *
 * @param queue Queue instance
 * @param value Value to push
 */
void queue_push(queue_t *queue, void *value);

/**
 * @brief Pop elements from the front of the queue.
 *
 * @param queue Queue instance
 * @return void* Value of the element popped.
 */
void* queue_pop(queue_t *queue);

/**
 * @brief Peek the front element of the queue.
 *
 * @param queue Queue Instance
 * @return void* Front element
 */
void* queue_front(queue_t *queue);

/**
 * @brief Peek the last element of the queue.
 *
 * @param queue Queue instance.
 * @return void* Last element
 */
void* queue_back(queue_t *queue);

/**
 * @brief Check if queue is empty
 *
 * @param queue
 * @return true If queue is empty
 * @return false If queue is not empty
 */
bool queue_empty(queue_t *queue);

/**
 * @brief Free the queue's elements.
 *
 * @warning This does not free the queue data structure itself
 * 			but the elemental blocks it uses. Also it does not
 * 			free the elements added to the queue.
 *
 * @param queue Queue instance
 */
void queue_free(queue_t *queue);

#endif