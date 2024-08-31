/* C bindings into Nuspell functions. */
#ifdef __cplusplus
extern "C" {
#endif

typedef struct NuspellHandle NuspellHandle;

NuspellHandle* Nuspell_create(const char* aff_path);

void Nuspell_destroy(NuspellHandle* dict);

/* note: it's a bool */
int Nuspell_spell(NuspellHandle* dict, const char* word);

#ifdef __cplusplus
}
#endif
