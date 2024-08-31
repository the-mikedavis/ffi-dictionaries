#include "interface.h"
#include "dictionary.hxx"

using nuspell::Dictionary;

NuspellHandle* Nuspell_create(const char* aff_path) {
  auto dict = new Dictionary();
  dict->load_aff_dic(aff_path);
  return reinterpret_cast<NuspellHandle*>(dict);
}

void Nuspell_destroy(NuspellHandle* dict) {
  delete reinterpret_cast<Dictionary*>(dict);
}

int Nuspell_spell(NuspellHandle* dict, const char* word) {
  return reinterpret_cast<Dictionary*>(dict)->spell(word);
}
