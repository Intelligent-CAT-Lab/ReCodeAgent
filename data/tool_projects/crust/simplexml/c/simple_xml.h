#ifndef SIMPLE_XML_H_
#define SIMPLE_XML_H_

typedef struct XMLElement {
  char* tag_name;
  char* value;
  struct XMLElement* parent;
  struct Vector* children; 
} XMLElement;

// Initialize for XMLElement `e` with `tag_name` and `value`
// Example
//    >>> <programmer>Kien Nguyen Trung</programmer>
//    => tag_name = 'programmer'
//       value = 'Kien Nguyen Trung'
XMLElement* XMLElement_create(XMLElement *e, char* tag_name, char* value);

// Release XMLElement
void XMLElement_release(XMLElement *e);

// Parse xml from text
// Return XMLElement represent for input
XMLElement* parse_xml(char *text);

#endif
