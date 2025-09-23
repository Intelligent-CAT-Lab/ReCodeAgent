#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "simple_vector.h"
#include "simple_xml.h"

// Initialize for XMLElement `e` with `tag_name` and `value`
// Example
//    >>> <programmer>Kien Nguyen Trung</programmer>
//    => tag_name = 'programmer'
//       value = 'Kien Nguyen Trung'
XMLElement* XMLElement_create(XMLElement *e, char* tag_name, char* value) {
  e = malloc(sizeof(XMLElement));   
  e->tag_name = tag_name;
  e->value = value;
  e->parent = NULL;
  e->children = vector_create(e->children);
  return e;
}

// Release XMLElement
void XMLElement_release(XMLElement *e) {
  e->parent = NULL;
  free(e->tag_name);
  free(e->value);
  vector_release(e->children);
  free(e);
  e = NULL;
}

#define BEGIN_TAG_TOKEN '<'
#define END_TAG_TOKEN '>'
#define SPLASH_TOKEN '/'

typedef enum {
  BEGIN_OPEN_TAG = 0,
  BEGIN_CLOSE_TAG,
  END_TAG, 
  TEXT = 3,
} XMLTokenType;

typedef struct XMLToken {
  XMLTokenType type;
  char* data;
} XMLToken;

typedef enum {
  STATE1 = 0,
  STATE2,
  STATE3,
  STATE4,
  STATE5,
  STATE6,
  STATE7,
  STATE8,
  STATE_ERROR
} ParseState;

// We apply the model
// <xml> = <open_tag> (TEXT | [<xml>]*)  <close_tag>
// <open_tag> = BEGIN_OPEN_TAG TEXT END_TAG
// <close_tag> = BEGIN_CLOSE_TAG TEXT END_TAG
//
// Model
// STATE1 --- (BEGIN_OPEN_TAG) ----> STATE2
// STATE2 --- (TEXT) --------------> STATE3
// STATE3 --- (END_TAG) -----------> STATE4
// STATE4 --- (BEGIN_OPEN_TAG) ----> STATE2
// STATE4 --- (TEXT) --------------> STATE5
// STATE5 --- (BEGIN_CLOSE_TAG) ---> STATE6
// STATE6 --- (TEXT) --------------> STATE7
// STATE7 --- (END_TAG) -----------> STATE8
// STATE8 --- (BEGIN_OPEN_TAG) ----> STATE2
static ParseState state_translate[8][4] = {
  //BEGIN_OPEN_TAG  BEGIN_CLOSE_TAG   END_TAG      TEXT
  { STATE2,         STATE_ERROR,      STATE_ERROR, STATE_ERROR }, // STATE1
  { STATE_ERROR,    STATE_ERROR,      STATE_ERROR, STATE3 },      // STATE2
  { STATE_ERROR,    STATE_ERROR,      STATE4,      STATE_ERROR }, // STATE3
  { STATE2,         STATE_ERROR,      STATE_ERROR, STATE5 },      // STATE4
  { STATE_ERROR,    STATE6,           STATE_ERROR, STATE_ERROR }, // STATE5
  { STATE_ERROR,    STATE_ERROR,      STATE_ERROR, STATE7 },      // STATE6
  { STATE_ERROR,    STATE_ERROR,      STATE8,      STATE_ERROR }, // STATE7
  { STATE2,         STATE6,           STATE_ERROR, STATE_ERROR }, // STATE8
};

typedef struct XMLParser {
  char* _input;
  int _pos;
  int _depth;

  ParseState state;

  Vector* value_stack;
  Vector* tag_stack;
  Vector* element_stack;
} XMLParser;

// Initialize a parser
static XMLParser* XMLParser_create(XMLParser *p) {
  p = malloc(sizeof(XMLParser));
  p->_depth = 0;
  p->state = STATE1;
  p->tag_stack = vector_create(p->tag_stack);
  p->value_stack = vector_create(p->value_stack);
  p->element_stack = vector_create(p->element_stack);
  return p;
}

// Release a parse
static void XMLParser_release(XMLParser *p) {
  vector_release(p->element_stack);
  vector_release(p->value_stack);
  vector_release(p->tag_stack);
  free(p);
  p = NULL;
}

// Return text token from `from` position to `to` position in `input`
// Example: input = "<name>Kien</name>"
//     parser_get_text_token(parser, 1, 4) => 'name'
static XMLToken* parser_get_text_token(XMLParser* parser, int from, int to) {
  XMLToken* token;
  int str_size;

  token = malloc(sizeof(XMLToken)); 
  token->type = TEXT;
  
  // trim
  while (parser->_input[from] == ' ') from++;
  while (parser->_input[to] == ' ') to--;
  

  token->type = TEXT;

  if (to >= from) {
    str_size = to - from + 1;
    token->data = (char *)malloc(sizeof(char) * (str_size + 1));
    strncpy(token->data, parser->_input + from, str_size);
    token->data[str_size] = '\0';
  } else {
    token->data = NULL;
  }

  return token; 
}

// Get next token of input 
static XMLToken* parser_get_next_token(XMLParser *parser) {
  int length, begin_pos;
 
  begin_pos = parser->_pos; 
  length = strlen(parser->_input);

  if (begin_pos >= length)
    return NULL;

  while (parser->_pos < length) {
    char ch;
    ch = parser->_input[parser->_pos];
    parser->_pos++;

    switch(ch) {
      case BEGIN_TAG_TOKEN: {
        if (parser->_pos >  begin_pos + 1) {
          parser->_pos--;
          return parser_get_text_token(parser, begin_pos, parser->_pos - 1);
        } else {
          char next_char = parser->_input[parser->_pos];
          if (next_char == SPLASH_TOKEN) {
            XMLToken *token;
            token = malloc(sizeof(XMLToken));
            token->type = BEGIN_CLOSE_TAG;
            parser->_pos++;
            return token;
          } else {
            XMLToken *token;
            token = malloc(sizeof(XMLToken));
            token->type = BEGIN_OPEN_TAG;
            return token;
          }
        }
      }

      case END_TAG_TOKEN: 
        if (parser->_pos > begin_pos + 1) {
          parser->_pos--;
          return parser_get_text_token(parser, begin_pos, parser->_pos - 1);
        } else {
          XMLToken *token;
          token = malloc(sizeof(XMLToken));
          token->type = END_TAG;
          return token;
        }

      default:
        break;
    }
  }
  
  return parser_get_text_token(parser, begin_pos, parser->_pos - 1);  
}


typedef struct StackElement {
  XMLElement *element;
  int depth; 
} StackElement;

static StackElement* StackElement_create(StackElement *se) {
  se = malloc(sizeof(StackElement));
  return se;
}

static void StackElement_release(StackElement *se) {
  free(se);
  se = NULL;
} 

// Parse xml from text
// Return XMLElement represent for input
XMLElement* parse_xml_from_text(char *text) {
  XMLToken *token;
  XMLParser *parser;

  parser = XMLParser_create(parser);
  parser->_input = text;
  parser->_pos = 0;
  parser->state = STATE1;

  while (1) {
    ParseState state;

    token = parser_get_next_token(parser); 
    if (token == NULL) break;
    if (token->type == TEXT && token->data == NULL) 
      continue;
    
    state = state_translate[parser->state][token->type];
    if (state != STATE_ERROR) {
      switch (parser->state) {

        case STATE1:
          break;

        case STATE2:
          if (token->type == TEXT) {
            vector_push_back(parser->tag_stack, token->data);
            parser->_depth++;
          }
          break;

        case STATE3:
          break;

        case STATE4:
          vector_push_back(parser->value_stack, token->data);
          break;

        case STATE5:
          break;

        case STATE6:
          if (token->type == TEXT) {
            assert(strcmp(token->data, vector_top_back(parser->tag_stack)) == 0); 
          }
          break;

        case STATE7:
          if (token->type == END_TAG) {
            char *current_tag, *current_value;
            int i, length;
            XMLElement *top;
            XMLElement *current;
            StackElement *se;

            current_tag = vector_top_back(parser->tag_stack);
            current_value = vector_top_back(parser->value_stack);
            length = vector_size(parser->element_stack);
            parser->_depth--;

            current = XMLElement_create(current, current_tag, current_value);
            se = StackElement_create(se);
            se->element = current;
            se->depth = parser->_depth;
            
            // find children of current elem
            for (i = 0; i < length; ++i) {
              StackElement *elem = (StackElement *)vector_top_back(parser->element_stack);
              if (elem->depth <= se->depth) break;

              vector_push_front(current->children, elem->element);
              vector_pop_back(parser->element_stack);
            }
            
            // push to stack
            vector_push_back(parser->element_stack, se);

            vector_pop_back(parser->tag_stack);
            vector_pop_back(parser->value_stack);
          }
          break;

        case STATE8:
          break;
        default:
          break;
      }
    }

    free(token);
    if (state == STATE_ERROR)
      assert(state != STATE_ERROR && "error while parsing");
    parser->state = state; 
  } 

  StackElement* stackElem = (StackElement *) vector_top_back(parser->element_stack);
  XMLElement *xmlElem = stackElem->element;
  StackElement_release(stackElem);
  XMLParser_release(parser);
  return xmlElem;
}
