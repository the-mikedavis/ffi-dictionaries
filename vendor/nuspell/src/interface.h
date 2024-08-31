/* C bindings into Nuspell functions. */
#ifdef __cplusplus
extern "C" {
#endif

typedef struct NuspellDictionary NuspellDictionary;

NuspellDictionary* Dictionary_create(const char* aff_path);

void Dictionary_destroy(NuspellDictionary* dict);

/* note: it's a bool */
int Dictionary_spell(NuspellDictionary* dict, const char* word);

#ifdef __cplusplus
}
#endif
