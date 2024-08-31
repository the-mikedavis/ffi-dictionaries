#include "interface.h"
#include "dictionary.hxx"

using nuspell::Dictionary;

NuspellDictionary* Dictionary_create(const char* aff_path) {
  auto dict = new Dictionary();
  dict->load_aff_dic(aff_path);
  return reinterpret_cast<NuspellDictionary*>(dict);
}

void Dictionary_destroy(NuspellDictionary* dict) {
  delete reinterpret_cast<Dictionary*>(dict);
}

int Dictionary_spell(NuspellDictionary* dict, const char* word) {
  return reinterpret_cast<Dictionary*>(dict)->spell(word);
}
