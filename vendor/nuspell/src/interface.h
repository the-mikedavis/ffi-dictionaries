/* C bindings into Nuspell functions. */
#ifdef __cplusplus
extern "C" {
#endif

typedef struct NuspellHandle NuspellHandle;

NuspellHandle* Nuspell_create(const char* aff_path);

void Nuspell_destroy(NuspellHandle* dict);

/* note: it's a bool */
int Nuspell_spell(NuspellHandle* dict, const char* word);

int Nuspell_suggest(const NuspellHandle*, char*** slst, const char* word);

void Nuspell_free_list(char*** slst, int n);

#ifdef __cplusplus
}
#endif
