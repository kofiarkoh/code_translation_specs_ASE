#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXLEN 205 /* up to 204 chars + NUL */

static int cmp_cstr(const void *a, const void *b) {
    const char * const *pa = (const char * const *)a;
    const char * const *pb = (const char * const *)b;
    return strcmp(*pa, *pb);
}

static void reverse_copy(char *dst, const char *src) {
    size_t n = strlen(src);
    for (size_t i = 0; i < n; i++) dst[i] = src[n - 1 - i];
    dst[n] = '\0';
}

/* lower_bound: first index i in [0,n] with arr[i] >= key */
static size_t lower_bound(char **arr, size_t n, const char *key) {
    size_t lo = 0, hi = n;
    while (lo < hi) {
        size_t mid = lo + (hi - lo) / 2;
        if (strcmp(arr[mid], key) < 0) lo = mid + 1;
        else hi = mid;
    }
    return lo;
}

/* upper_bound: first index i in [0,n] with arr[i] > key */
static size_t upper_bound(char **arr, size_t n, const char *key) {
    size_t lo = 0, hi = n;
    while (lo < hi) {
        size_t mid = lo + (hi - lo) / 2;
        if (strcmp(arr[mid], key) <= 0) lo = mid + 1;
        else hi = mid;
    }
    return lo;
}

/* Count words equal to key in sorted array */
static size_t count_equal(char **arr, size_t n, const char *key) {
    size_t lb = lower_bound(arr, n, key);
    size_t ub = upper_bound(arr, n, key);
    return (ub >= lb) ? (ub - lb) : 0;
}

/*
  Count words with given prefix in sorted array.
  Uses range [prefix, prefix_with_last_byte_incremented) when possible,
  otherwise falls back to scanning from lower_bound(prefix) while prefix matches.
*/
static size_t count_prefix(char **arr, size_t n, const char *prefix) {
    size_t plen = strlen(prefix);
    size_t start = lower_bound(arr, n, prefix);

    if (plen == 0) return n;

    char hi[MAXLEN];
    if (plen >= MAXLEN) plen = MAXLEN - 1; /* undefined per spec if exceeded; clamp defensively */
    memcpy(hi, prefix, plen + 1);

    /* Find last position that is not 0xFF to increment; if none, cannot form strict upper bound */
    ssize_t i;
    for (i = (ssize_t)plen - 1; i >= 0; i--) {
        unsigned char c = (unsigned char)hi[i];
        if (c != 0xFF) {
            hi[i] = (char)(c + 1);
            hi[i + 1] = '\0';
            break;
        }
    }

    if (i >= 0) {
        size_t end = lower_bound(arr, n, hi);
        return (end >= start) ? (end - start) : 0;
    } else {
        /* Fallback: scan while prefix matches */
        size_t cnt = 0;
        for (size_t k = start; k < n; k++) {
            if (strncmp(arr[k], prefix, plen) == 0) cnt++;
            else break;
        }
        return cnt;
    }
}

/* Find first '?' in s; returns index or -1 */
static int first_qmark(const char *s) {
    const char *p = strchr(s, '?');
    if (!p) return -1;
    return (int)(p - s);
}

/* Build a variant of pattern by replacing first '?' with ch; other '?' remain literal */
static void replace_first_qmark(char *out, const char *pattern, char ch) {
    strcpy(out, pattern);
    int pos = first_qmark(out);
    if (pos >= 0) out[pos] = ch;
}

int main(void) {
    int N_in, M_in;
    if (scanf("%d %d", &N_in, &M_in) != 2) return 0;

    if (N_in < 0) N_in = 0; /* precondition says non-negative; keep narrow behavior */
    if (M_in < 0) M_in = 0;

    size_t N = (size_t)N_in;
    size_t M = (size_t)M_in;

    char **dict = (char **)malloc(N * sizeof(char *));
    char **rdict = (char **)malloc(N * sizeof(char *));
    if (!dict || !rdict) return 0;

    for (size_t i = 0; i < N; i++) {
        char buf[MAXLEN];
        if (scanf("%204s", buf) != 1) buf[0] = '\0';
        size_t len = strlen(buf);

        dict[i] = (char *)malloc(len + 1);
        rdict[i] = (char *)malloc(len + 1);
        if (!dict[i] || !rdict[i]) return 0;

        memcpy(dict[i], buf, len + 1);
        reverse_copy(rdict[i], buf);
    }

    qsort(dict, N, sizeof(char *), cmp_cstr);
    qsort(rdict, N, sizeof(char *), cmp_cstr);

    for (size_t qi = 0; qi < M; qi++) {
        char pat[MAXLEN];
        if (scanf("%204s", pat) != 1) pat[0] = '\0';

        size_t plen = strlen(pat);
        int has_star = 0;
        int star_at_start = 0;
        int star_at_end = 0;

        if (plen > 0 && pat[0] == '*') {
            has_star = 1;
            star_at_start = 1;
        }
        if (plen > 0 && pat[plen - 1] == '*') {
            has_star = 1;
            star_at_end = 1;
        }

        /* If both ends are '*', spec says unsupported/unspecified; we choose narrow handling:
           treat as leading '*' (suffix match) after stripping only the first '*'. */
        if (star_at_start && star_at_end && plen >= 2) {
            star_at_end = 0;
        }

        char core[MAXLEN];
        if (has_star) {
            if (star_at_end) {
                /* trailing '*' => prefix match on core = pat without last char */
                if (plen >= 1) {
                    size_t clen = plen - 1;
                    if (clen >= MAXLEN) clen = MAXLEN - 1;
                    memcpy(core, pat, clen);
                    core[clen] = '\0';
                } else {
                    core[0] = '\0';
                }
            } else if (star_at_start) {
                /* leading '*' => suffix match on core = pat without first char */
                if (plen >= 1) {
                    size_t clen = plen - 1;
                    if (clen >= MAXLEN) clen = MAXLEN - 1;
                    memcpy(core, pat + 1, clen);
                    core[clen] = '\0';
                } else {
                    core[0] = '\0';
                }
            } else {
                /* '*' not at ends: unspecified; treat as literal (no star) */
                has_star = 0;
                strcpy(core, pat);
            }
        } else {
            strcpy(core, pat);
        }

        int qpos = first_qmark(core);
        size_t total = 0;

        if (!has_star) {
            if (qpos < 0) {
                total = count_equal(dict, N, core);
            } else {
                char variant[MAXLEN];
                for (int c = 0; c < 26; c++) {
                    replace_first_qmark(variant, core, (char)('a' + c));
                    total += count_equal(dict, N, variant);
                }
            }
        } else if (star_at_end) {
            /* prefix match on dict */
            if (qpos < 0) {
                total = count_prefix(dict, N, core);
            } else {
                char variant[MAXLEN];
                for (int c = 0; c < 26; c++) {
                    replace_first_qmark(variant, core, (char)('a' + c));
                    total += count_prefix(dict, N, variant);
                }
            }
        } else if (star_at_start) {
            /* suffix match via reversed dict and reversed core (becomes prefix match) */
            char rcore[MAXLEN];
            reverse_copy(rcore, core);

            int rqpos = first_qmark(rcore);
            if (rqpos < 0) {
                total = count_prefix(rdict, N, rcore);
            } else {
                char variant[MAXLEN];
                for (int c = 0; c < 26; c++) {
                    replace_first_qmark(variant, rcore, (char)('a' + c));
                    total += count_prefix(rdict, N, variant);
                }
            }
        } else {
            /* Should not happen; treat as exact */
            if (qpos < 0) total = count_equal(dict, N, core);
            else {
                char variant[MAXLEN];
                for (int c = 0; c < 26; c++) {
                    replace_first_qmark(variant, core, (char)('a' + c));
                    total += count_equal(dict, N, variant);
                }
            }
        }

        printf("%zu\n", total);
    }

    for (size_t i = 0; i < N; i++) {
        free(dict[i]);
        free(rdict[i]);
    }
    free(dict);
    free(rdict);
    return 0;
}
