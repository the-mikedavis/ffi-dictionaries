
#ifndef NUSPELL_EXPORT_H
#define NUSPELL_EXPORT_H

#ifdef NUSPELL_STATIC_DEFINE
#  define NUSPELL_EXPORT
#  define NUSPELL_NO_EXPORT
#else
#  ifndef NUSPELL_EXPORT
#    ifdef nuspell_EXPORTS
        /* We are building this library */
#      define NUSPELL_EXPORT __attribute__((visibility("default")))
#    else
        /* We are using this library */
#      define NUSPELL_EXPORT __attribute__((visibility("default")))
#    endif
#  endif

#  ifndef NUSPELL_NO_EXPORT
#    define NUSPELL_NO_EXPORT __attribute__((visibility("hidden")))
#  endif
#endif

#ifndef NUSPELL_DEPRECATED
#  define NUSPELL_DEPRECATED __attribute__ ((__deprecated__))
#endif

#ifndef NUSPELL_DEPRECATED_EXPORT
#  define NUSPELL_DEPRECATED_EXPORT NUSPELL_EXPORT NUSPELL_DEPRECATED
#endif

#ifndef NUSPELL_DEPRECATED_NO_EXPORT
#  define NUSPELL_DEPRECATED_NO_EXPORT NUSPELL_NO_EXPORT NUSPELL_DEPRECATED
#endif

#if 0 /* DEFINE_NO_DEPRECATED */
#  ifndef NUSPELL_NO_DEPRECATED
#    define NUSPELL_NO_DEPRECATED
#  endif
#endif

#endif /* NUSPELL_EXPORT_H */
