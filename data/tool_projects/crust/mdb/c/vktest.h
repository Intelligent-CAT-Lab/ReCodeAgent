/**
 * VK-Test is a tiny header-only test framework for C
 * Please note that this file should only be included in one file,
 * and this test should not be used for multi-thread case
 */

#ifndef VKTEST_H
#define VKTEST_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

static unsigned __g_passing_tests, __g_all_tests;
static unsigned __g_passing_tests_section, __g_all_tests_section;

void __vk_maybe_terminate() {
#ifdef VK_TERMINATE_ON_FAIL
  fprintf(stderr, "VK_TERMINATE_ON_FAIL was set, terminating test...\n");
  fprintf(stderr, "Test terminated, %u of %u passing.\n",
          __g_passing_tests_section, __g_all_tests_section);
  abort();
#endif
}

#define VK_TEST_BEGIN fprintf(stderr, "VKTest starting...\n");

#define VK_TEST_END \
  fprintf(stderr, "All test finished, %u of %u passing\n",\
          __g_passing_tests, __g_all_tests);

#define VK_TEST_SECTION_BEGIN(section_name) \
  { \
    fprintf(stderr, "Running test: %s...\n", section_name); \
    __g_passing_tests_section = 0; \
    __g_all_tests_section = 0; \
  }

#define VK_TEST_SECTION_END(section_name) \
  { \
    fprintf(stderr, "Test %s finished, %u of %u passing.\n", \
            section_name, \
            __g_passing_tests_section, __g_all_tests_section); \
  }

#define VK_ASSERT(condition) \
  { \
    __g_all_tests++; \
    __g_all_tests_section++; \
    fprintf(stderr, "[%u] Testing Assert(%s)... ", __g_all_tests_section, \
            #condition); \
    if (condition) { \
      fprintf(stderr, "Passing\n"); \
      __g_passing_tests++; \
      __g_passing_tests_section++; \
    } \
    else { \
      fprintf(stderr, "Failing\n"); \
      __vk_maybe_terminate(); \
    } \
  }

#define VK_ASSERT_NOT(condition) \
  { \
    __g_all_tests++; \
    __g_all_tests_section++; \
    fprintf(stderr, "[%u] Testing AssertNot(%s)... ", __g_all_tests_section, \
            #condition); \
    if (!condition) { \
      fprintf(stderr, "Passing\n"); \
      __g_passing_tests++; \
      __g_passing_tests_section++; \
    } \
    else { \
      fprintf(stderr, "Failing\n"); \
      __vk_maybe_terminate(); \
    } \
  }

#define VK_ASSERT_EQUALS(standard, expr) \
  { \
    __g_all_tests++; \
    __g_all_tests_section++; \
    fprintf(stderr, "[%u] Testing AssertEquals(%s, %s)... ", \
            __g_all_tests_section, \
            #standard, #expr); \
    if ((standard) == (expr)) { \
      fprintf(stderr, "Passing\n"); \
      __g_passing_tests++; \
      __g_passing_tests_section++; \
    } \
    else { \
      fprintf(stderr, "Failing\n"); \
      __vk_maybe_terminate(); \
    } \
  }

#define VK_ASSERT_EQUALS_S(standard, expr) \
  { \
    __g_all_tests++; \
    __g_all_tests_section++; \
    fprintf(stderr, "[%u] Testing AssertEquals(\"%s\", \"%s\")... ",\
            __g_all_tests_section, \
            standard, expr); \
    if (!strcmp(standard, expr)) { \
      fprintf(stderr, "Passing\n"); \
      __g_passing_tests++; \
      __g_passing_tests_section++; \
    } \
    else { \
      fprintf(stderr, "Failing\n"); \
      __vk_maybe_terminate(); \
    } \
  }

#define VK_EPSILON 1e-5

#define VK_ASSERT_EQUALS_F(standard, expr) \
  { \
    __g_all_tests++; \
    __g_all_tests_section++; \
    fprintf(stderr, "[%u] Testing AssertEquals(%s, %s)... ", \
            __g_all_tests_section, \
            #standard, #expr); \
    if (fabs((standard) - (expr)) <= VK_EPSILON) { \
      fprintf(stderr, "Passing\n"); \
      __g_passing_tests++; \
      __g_passing_tests_section++; \
    } \
    else { \
      fprintf(stderr, "Failing\n"); \
      __vk_maybe_terminate(); \
    } \
  }

#define VK_ASSERT_NOT_EQUALS(standard, expr) \
  { \
  __g_all_tests++; \
  __g_all_tests_section++; \
  fprintf(stderr, "[%u] Testing AssertNotEquals(%s, %s)... ", \
          __g_all_tests_section, \
          #standard, #expr); \
    if ((standard) != (expr)) { \
      fprintf(stderr, "Passing\n"); \
      __g_passing_tests++; \
      __g_passing_tests_section++; \
    } \
    else { \
      fprintf(stderr, "Failing\n"); \
      __vk_maybe_terminate(); \
    } \
  }

#else
  #error Multiple inclusion for vktest?
#endif
