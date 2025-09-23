#ifndef INVERSION_LIST_INC_
#define INVERSION_LIST_INC_

struct _InversionList {
  unsigned int capacity;
  unsigned int support;
  size_t size; 
 /* union {
    uint8_t* uint8;
    uint16_t* uint16;
    uint32_t* uint32;
  } couples;*/
unsigned int couples[];
};

struct _InversionListIterator {
  const InversionList *set;
  unsigned int index;
  unsigned int n[];
};

struct _InversionListCoupleIterator {
  const InversionList *set;

  unsigned int index;
};

#endif  // INVERSION_LIST_INC_
