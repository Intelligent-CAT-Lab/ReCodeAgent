#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "simple_xml.h"
#include "simple_vector.h"

void test_vector() {
  Vector *v;
  int a, b, c, i;
  a = 10; 
  b = 20; 
  c = 30;

  // test allocate
  v = vector_create(v);
  assert(v->_size == 0 && "create size FAILED");
  assert(v->_capacity == 8 && "create capacity FAILED");

  // test push back
  vector_push_back(v, &a);
  vector_push_back(v, &b);
  assert(vector_size(v) == 2 && "insert element FAILED");
  assert(vector_get_element_at(v, 0) == &a && "insert element FAILED");
  assert(vector_get_element_at(v, 1) == &b && "insert element FAILED");
  
  // test push front 
  vector_push_front(v, &a);
  vector_push_front(v, &b);
  vector_push_front(v, &c);
  assert(vector_size(v) == 5);
  assert(vector_get_element_at(v, 0) == &c);
  assert(vector_get_element_at(v, 1) == &b);
  assert(vector_get_element_at(v, 2) == &a);
  assert(vector_get_element_at(v, 3) == &a);
  assert(vector_get_element_at(v, 4) == &b);
  
  // test insert
  vector_insert_at_index(v, &c, 2);
  assert(vector_size(v) == 6);
  assert(vector_get_element_at(v, 0) == &c);
  assert(vector_get_element_at(v, 1) == &b);
  assert(vector_get_element_at(v, 2) == &c);
  assert(vector_get_element_at(v, 3) == &a);
  assert(vector_get_element_at(v, 4) == &a);
  assert(vector_get_element_at(v, 5) == &b);

  // test remove
  vector_remove_element_at_index(v, 2);
  assert(vector_size(v) == 5);
  assert(vector_get_element_at(v, 0) == &c);
  assert(vector_get_element_at(v, 1) == &b);
  assert(vector_get_element_at(v, 2) == &a);
  assert(vector_get_element_at(v, 3) == &a);
  assert(vector_get_element_at(v, 4) == &b);

  // test re-allocate
  for (i = 0; i < 100; ++i) {
    vector_push_back(v, &a);
  }
  assert(vector_size(v) == 105);
  assert(vector_get_element_at(v, 0) == &c);
  assert(vector_get_element_at(v, 1) == &b);
  assert(vector_get_element_at(v, 2) == &a);
  assert(vector_get_element_at(v, 3) == &a);
  assert(vector_get_element_at(v, 4) == &b);
  for (i = 5; i < 105; ++i) {
    assert(vector_get_element_at(v, i) == &a);
  }

  // test top
  assert(vector_top_back(v) == &a);
  assert(vector_top_front(v) == &c);
  assert(vector_size(v) == 105);

  vector_release(v);

  printf("PASSED Test vector1\n");
}

#define SIZE 100000
void test_vector2() {
  int i;
  Vector *v;

  v = vector_create(v);
  // init element
  for (i = 0; i < SIZE; ++i) {
    XMLElement *e;
    char *tag_name, *value;
    tag_name = malloc(30 * sizeof(char));
    value = malloc(30 * sizeof(char));
    sprintf(tag_name, "tag %d", i);
    sprintf(value, "value %d", i);
    e = XMLElement_create(e, tag_name, value);
    vector_push_back(v, e);
  }

  // assert element
  for (i = 0; i < vector_size(v); ++i) {
    XMLElement *e;
    char *tag_name, *value;
    tag_name = malloc(30 * sizeof(char));
    value = malloc(30 * sizeof(char));
    sprintf(tag_name, "tag %d", i);
    sprintf(value, "value %d", i);

    e = (XMLElement *) vector_get_element_at(v, i);
    assert(strcmp(e->tag_name, tag_name) == 0);
    assert(strcmp(e->value, value) == 0);
    free(tag_name);
    free(value);
  }

  // release
  for (i = 0; i < vector_size(v); ++i) {
    XMLElement *e;
    e = vector_get_element_at(v, i);
    XMLElement_release(e);
  }
  vector_release(v);

  printf("PASSED Test vector2\n");
}

void test_xml() {
  int i;
  char* s = 
    "<programmer>\
    <name>Kien Nguyen Trung</name>\
    <languages>\
    <language>C</language>\
    <language>C++</language>\
    <language>Python</language>\
    <language>Ruby</language>\
    <language>Objective C</language>\
    <language>Java</language>\
    <language>Javascript</language>\
    <language>Lua</language>\
    <language>C#</language>\
    <language>PHP</language>\
    </languages> \
    </programmer>";

  XMLElement *elem, *child1, *child2;
  elem = (XMLElement *)parse_xml_from_text(s); 
  assert(strcmp(elem->tag_name, "programmer") == 0);
  assert(vector_size(elem->children) == 2);

  child1 = (XMLElement *)vector_get_element_at(elem->children, 0); 
  assert(strcmp(child1->tag_name, "name") == 0);
  assert(vector_size(child1->children) == 0);
  assert(strcmp(child1->value, "Kien Nguyen Trung") == 0);

  child2 = (XMLElement *)vector_get_element_at(elem->children, 1); 
  assert(strcmp(child2->tag_name, "languages") == 0);
  assert(vector_size(child2->children) == 10);
  char* list_languages[10] = {
    "C", "C++", "Python", "Ruby", "Objective C", "Java", "Javascript", "Lua", "C#", "PHP"
  };

  for (i = 0; i < 10; i++)  {
    XMLElement *child;
    child = (XMLElement *) vector_get_element_at(child2->children, i);
    assert(strcmp(child->value, list_languages[i]) == 0);
    assert(strcmp(child->tag_name, "language") == 0);
  }

  printf("PASSED Test parser xml\n");
}

int main(int argc, char** argv) {
  test_vector();
  test_vector2();
  test_xml();
  return 0;
}
