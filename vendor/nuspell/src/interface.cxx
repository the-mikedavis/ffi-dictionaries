#include "interface.h"
#include "dictionary.hxx"
#include <cstring>

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


// All of the below copies or based on Hunspell functions from `hunspell.cxx` of the same names
// (or swap Nuspell for Hunspell).

char* stringdup(const std::string& s) {
  size_t sl = s.size() + 1;
  char* d = (char*)malloc(sl);
  if (d)
    memcpy(d, s.c_str(), sl);
  return d;
}

int munge_vector(char*** slst, const std::vector<std::string>& items) {
  if (items.empty()) {
    *slst = NULL;
    return 0;
  } else {
    *slst = new char*[items.size()];
    for (size_t i = 0; i < items.size(); ++i)
      (*slst)[i] = stringdup(items[i]);
  }
  return items.size();
}

int Nuspell_suggest(const NuspellHandle* dict, char*** slst, const char* word) {
  std::vector<std::string> out = {};
  reinterpret_cast<const Dictionary*>(dict)->suggest(word, out);
  return munge_vector(slst, out);
}

void Nuspell_free_list(char*** slst, int n) {
  if (slst && *slst) {
    for (int i = 0; i < n; i++)
      free((*slst)[i]);
    delete[] *slst;
    *slst = NULL;
  }
}
