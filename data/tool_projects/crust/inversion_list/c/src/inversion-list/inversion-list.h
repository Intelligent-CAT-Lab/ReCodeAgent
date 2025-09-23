#ifndef INVERSION_LIST_INVERSION_LIST_H_
#define INVERSION_LIST_INVERSION_LIST_H_

/**
 * @file inversion-list.h
 */

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * `InversionList` is an opaque structure able to handle sets of unsigned
 * integers.
 */
typedef struct _InversionList InversionList;

/**
 * `InversionListIterator` is an opaque structure able to handle iterator
 * of an inversion list
 */
typedef struct _InversionListIterator InversionListIterator;

/**
 * `InversionListCoupleIterato` is an opaque structure able to handle iterator over couples
 * of an inversion list.
 */
typedef struct _InversionListCoupleIterator InversionListCoupleIterator;

/**
 * Initialise the library.
 */
extern bool inversion_list_init(void);
/**
 * Terminate the library.
 */
extern bool inversion_list_finish(void);

/**
 * Create a set of integers.
 *
 * @param capacity the capacity not to be reached
 * @param count the number of elements of @p values
 * @param values the values to be inserted in the set
 *
 * @return a new set of integers or `NULL` if there is an error
 *
 * @note a `NULL` return indicates an error which can be retrieved in
 * `errno`
 */
extern InversionList *inversion_list_create(unsigned int capacity,
                                            size_t count,
                                            const unsigned int *values);
/**
 * Destroy a set of integers.
 *
 * @param set the set to be destroyed
 */
extern void inversion_list_destroy(InversionList *set);

/**
 * Get the set capacity.
 *
 * @param set the set whose capacity is requested
 *
 * @return the @p set capacity
 */
extern unsigned int inversion_list_capacity(const InversionList *set);
/**
 * Get the set support.
 *
 * @param set the set whose support is requested
 *
 * @return the @p set support
 */
extern unsigned int inversion_list_support(const InversionList *set);

/**
 * Get the membership of an element.
 *
 * @param set the set for which the membership of the @p value is requested
 * @param value the value
 *
 * @return the membership of the @p value in the @p set
 */
extern bool inversion_list_member(const InversionList *set,
                                  unsigned int value);
/**
 * Create a clone of set of integers.
 *
 * @param set a set whose clone is requested
 *
 * @return a @p set clone or `NULL` if there is an error
 *
 * @note a `NULL` return indicates an error which can be retrieved in
 * `errno`
 */
extern InversionList *inversion_list_clone(const InversionList *set);
/**
 * Create a complement of a set of integers.
 *
 * @param set a set whose complement is requested
 *
 * @return a @p set complement or `NULL` if there is an error
 *
 * @note a `NULL` return indicates an error which can be retrieved in
 * `errno`
 */
extern InversionList *inversion_list_complement(const InversionList *set);

/**
 * Convert a set of integers to a string.
 *
 * @param set a set whose string conversion is requested
 *
 * @return the string representation of @p set or `NULL` if there is an
 * error or if @p set is equal to `NULL`.
 *
 * @note a `NULL` return may indicate an error which can be retrieved in
 * `errno`
 */
extern const char *inversion_list_to_string(const InversionList *set);

/**
 * Compare two sets to know if they are equal.
 *
 * @param set1 a set that will be compared
 * @param set2 a set that will be compared
 *
 * @return true if @p set1 equal @p set2 or false if not equal
 *
 */
extern bool inversion_list_equal(const InversionList *set1,
                                 const InversionList *set2);

/**
 * Compare two sets to know if they are not equal.
 *
 * @param set1 a set that will be compared
 * @param set2 a set that will be compared
 *
 * @return true if @p set1 not equal @p set2 or false if equal
 *
 */
extern bool inversion_list_not_equal(const InversionList *set1,
                                     const InversionList *set2);

/**
 * Compare two sets to know if one of them is less than the other.
 *
 * @param set1 a set that will be compared
 * @param set2 a set that will be compared
 *
 * @return true if @p set1 less than @p set2 or false if not
 *
 */
extern bool inversion_list_less(const InversionList *set1,
                                const InversionList *set2);

/**
 * Compare two sets to know if one of them is less or equal than the other.
 *
 * @param set1 a set that will be compared
 * @param set2 a set that will be compared
 *
 * @return true if @p set1 less or equal than @p set2 or false if not
 *
 */
extern bool inversion_list_less_equal(const InversionList *set1,
                                      const InversionList *set2);

/**
 * Compare two sets to know if one of them is greater than the other.
 *
 * @param set1 a set that will be compared
 * @param set2 a set that will be compared
 *
 * @return true if @p set1 greater than @p set2 or false if not
 *
 */
extern bool inversion_list_greater(const InversionList *set1,
                                   const InversionList *set2);

/**
 * Compare two sets to know if one of them is greater or equal than the
 * other.
 *
 * @param set1 a set that will be compared
 * @param set2 a set that will be compared
 *
 * @return true if @p set1 greater or equal than @p set2 or false if not
 *
 */
extern bool inversion_list_greater_equal(const InversionList *set1,
                                         const InversionList *set2);

/**
 * Compare two sets to know if they are disjoint.
 *
 * @param set1 a set that will be compared
 * @param set2 a set that will be compared
 *
 * @return true if @p set1 and @p set2 are disjoint or false if not
 *
 */
extern bool inversion_list_disjoint(const InversionList *set1,
                                    const InversionList *set2);

/**
 * Create a set that is the symmetric difference of two sets.
 *
 * @param set1 a set that will be compared
 * @param set2 a set that will be compared
 *
 * @return the symmetric difference of @p set1 and @p set2
 *
 */
extern InversionList *inversion_list_symmetric_difference(
    const InversionList *set1, const InversionList *set2);

/**
 * Create a set that is the union of n sets.
 *
 * @param set1 a set that will be compared
 *
 * @return the union of @p set1 and n @p set
 *
 * @note the last element must be `NULL`
 */
extern InversionList *inversion_list_union(const InversionList *set, ...);

/**
 * Create a set that is the intersection of n sets.
 *
 * @param set1 a set that will be compared
 *
 * @return the intersection of @p set1 and n @p set
 *
 * @note the last element must be `NULL`
 */
extern InversionList *inversion_list_intersection(const InversionList *set,
                                                  ...);

/**
 * Create a set that is the difference of n sets.
 *
 * @param set1 a set that will be compared
 *
 * @return the difference of @p set1 and n @p set
 *
 * @note the last element must be `NULL`
 */
extern InversionList *inversion_list_difference(const InversionList *set1,
                                                ...);

/**
 * Create an iterator over the integers of an inversion list.
 *
 * @param set1
 *
 * @return a new iterator an iterator over the integers of @p set1.
 *
 */
extern InversionListIterator *inversion_list_iterator_create(
    const InversionList *set);

/**
 * Destroy an iterator over the integers of an inversion list.
 *
 * @param iterator to be destroy
 *
 */
extern void inversion_list_iterator_destroy(
    InversionListIterator *iterator);

/**
 * Implementation of the index.
 *
 * @param iterator the iterator that will have its index implemented
 *
 * @return @p iterator with his index implemented.
 *
 */
extern InversionListIterator *inversion_list_iterator_next(
    InversionListIterator *iterator);

/**
 * Reset of the index.
 *
 * @param iterator the iterator that will have its index reset
 *
 * @return @p iterator with his index rest.
 *
 */
extern InversionListIterator *inversion_list_iterator_rewind(
    InversionListIterator *iterator);

/**
 * Check of the iterator is stii valid.
 *
 * @param iterator the iterator that will be checked
 *
 * @return true if iterator is valid, false otherwise.
 *
 */
extern bool inversion_list_iterator_valid(
    const InversionListIterator *iterator);

/**
 * Get the current member of the iterator.
 *
 * @param iterator the iterator that will give his current member
 *
 * @return the current membre of the @p iterator
 *
 */
extern unsigned int inversion_list_iterator_get(
    InversionListIterator *iterator);

/**
 * Create an iterator over the couples of an inversion list.
 *
 * @param set
 *
 * @return a new iterator over the couples of @p set1.
 *
 */
extern InversionListCoupleIterator *inversion_list_couple_iterator_create(
    const InversionList *set);

/**
 * Destroy an iterator over the couples of an inversion list.
 *
 * @param iterator to be destroy
 *
 */
extern void inversion_list_couple_iterator_destroy(
    InversionListCoupleIterator *iterator);

/**
 * Implementation of the index.
 *
 * @param iterator the iterator that will have its index implemented
 *
 * @return @p iterator with his index implemented.
 *
 */
extern InversionListCoupleIterator *inversion_list_couple_iterator_next(
    InversionListCoupleIterator *iterator);

/**
 * Reset of the index.
 *
 * @param iterator the iterator that will have its index reset
 *
 * @return @p iterator with his index rest.
 *
 */
extern InversionListCoupleIterator *inversion_list_couple_iterator_rewind(
    InversionListCoupleIterator *iterator);

/**
 * Check of the iterator is stii valid.
 *
 * @param iterator the iterator that will be checked
 *
 * @return true if iterator is valid, false otherwise.
 *
 */
extern bool inversion_list_iterator_couple_valid(
    const InversionListCoupleIterator *iterator);

/**
 * Get the current inferior member of the current couple.
 *
 * @param iterator the iterator that will give his current inferior member
 *
 * @return the current inferior membre of the @p iterator
 *
 */
extern unsigned int inversion_list_couple_iterator_get_inf(
    const InversionListCoupleIterator *iterator);

/**
 * Get the current superior member of the current couple.
 *
 * @param iterator the iterator that will give his current superior member
 *
 * @return the current superior membre of @p iterator
 *
 */
extern unsigned int inversion_list_couple_iterator_get_sup(
    const InversionListCoupleIterator *iterator);
#endif  // INVERSION_LIST_INVERSION_LIST_H_
