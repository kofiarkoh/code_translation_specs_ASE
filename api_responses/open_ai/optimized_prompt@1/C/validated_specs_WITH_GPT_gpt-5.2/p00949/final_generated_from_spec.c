#include <stdio.h>
#include <string.h>
#include <stdint.h>

#define MAX_LINE 4096

/* Fixed-capacity membership structure:
   We store signatures of all length-k substrings of A in a fixed array.
   Capacity is bounded by MAX_LINE (max substrings when k>=1). */
typedef struct {
    uint16_t cnt[26]; /* counts for each letter; uint16_t is enough for MAX_LINE<=4096 */
} Sig;

static int read_token_line(char *buf, size_t cap) {
    /* Reads one line, then truncates at first whitespace (including newline).
       Returns 1 on success, 0 on EOF/no data. */
    if (!fgets(buf, (int)cap, stdin)) return 0;
    size_t n = strcspn(buf, " \t\r\n\v\f");
    buf[n] = '\0';
    return 1;
}

static void sig_init_from_window(Sig *s, const char *str, int k) {
    memset(s, 0, sizeof(*s));
    for (int i = 0; i < k; i++) {
        unsigned idx = (unsigned)(str[i] - 'a');
        s->cnt[idx]++;
    }
}

static void sig_slide(Sig *s, char outc, char inc) {
    s->cnt[(unsigned)(outc - 'a')]--;
    s->cnt[(unsigned)(inc - 'a')]++;
}

static int sig_equal(const Sig *a, const Sig *b) {
    /* Compare 26 counts */
    return memcmp(a->cnt, b->cnt, sizeof(a->cnt)) == 0;
}

static int exists_anagram_match_for_k(const char *A, int nA, const char *B, int nB, int k,
                                      Sig *setA, int *setA_count) {
    /* Build membership set of all length-k substring signatures of A (distinctness not required
       for correctness; we keep all, bounded by MAX_LINE). */
    int cap = MAX_LINE; /* fixed capacity */
    int count = 0;

    if (k <= 0) {
        *setA_count = 0;
        return 1; /* not used; caller won't ask for k<=0 */
    }
    if (k > nA || k > nB) {
        *setA_count = 0;
        return 0;
    }

    Sig cur;
    sig_init_from_window(&cur, A, k);
    if (count < cap) setA[count++] = cur;

    for (int i = k; i < nA; i++) {
        sig_slide(&cur, A[i - k], A[i]);
        if (count < cap) setA[count++] = cur;
    }

    *setA_count = count;

    /* For each length-k substring of B, check membership by linear scan. */
    Sig curB;
    sig_init_from_window(&curB, B, k);

    for (int j = 0; j < count; j++) {
        if (sig_equal(&curB, &setA[j])) return 1;
    }

    for (int i = k; i < nB; i++) {
        sig_slide(&curB, B[i - k], B[i]);
        for (int j = 0; j < count; j++) {
            if (sig_equal(&curB, &setA[j])) return 1;
        }
    }

    return 0;
}

int main(void) {
    char A[MAX_LINE];
    char B[MAX_LINE];

    if (!read_token_line(A, sizeof(A))) return 0;
    if (!read_token_line(B, sizeof(B))) return 0;

    int nA = (int)strlen(A);
    int nB = (int)strlen(B);
    int maxk = (nA < nB) ? nA : nB;

    /* Fixed-size membership storage; bounded and does not scale beyond MAX_LINE. */
    Sig setA[MAX_LINE];
    int setA_count = 0;

    int answer = 0;
    for (int k = maxk; k >= 1; k--) {
        if (exists_anagram_match_for_k(A, nA, B, nB, k, setA, &setA_count)) {
            answer = k;
            break;
        }
    }

    printf("%d\n", answer);
    return 0;
}
