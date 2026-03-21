#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INBUF_SIZE 1000000
#define OUTBUF_SIZE 2000000

static char outbuf[OUTBUF_SIZE];
static size_t outpos = 0;

static void out_ch(char c) {
    /* Undefined behavior if outbuf overflows, per spec. */
    outbuf[outpos++] = c;
}

static void out_str(const char *s) {
    while (*s) out_ch(*s++);
}

static void out_int(int v) {
    char tmp[64];
    /* Use C int formatting; overflow of sum is int overflow (UB) as in C. */
    snprintf(tmp, sizeof(tmp), "%d", v);
    out_str(tmp);
}

static int is_empty_at(const char *s, size_t i) {
    return s[i] == '(' && s[i + 1] == ')';
}

/* Parse a subtree starting at s[*i] and advance *i to the first char after it.
   Assumes well-formed input per spec; no validation. */
static void skip_subtree(const char *s, size_t *i) {
    if (is_empty_at(s, *i)) {
        *i += 2;
        return;
    }

    /* Non-empty node: (<left>)[<value>](<right>) */
    (*i)++;                 /* '(' */
    skip_subtree(s, i);     /* left */
    (*i)++;                 /* ')' */

    (*i)++;                 /* '[' */
    while (s[*i] != ']') (*i)++; /* value text */
    (*i)++;                 /* ']' */

    (*i)++;                 /* '(' */
    skip_subtree(s, i);     /* right */
    (*i)++;                 /* ')' */
}

static int parse_value_at(const char *s, size_t *i) {
    /* *i points at '[' */
    (*i)++; /* skip '[' */
    size_t start = *i;
    while (s[*i] != ']') (*i)++;
    size_t end = *i; /* points at ']' */
    (*i)++; /* skip ']' */

    /* Interpret as atoi on the substring. */
    char tmp[128];
    size_t len = end - start;
    if (len >= sizeof(tmp)) len = sizeof(tmp) - 1; /* UB avoided minimally; spec allows UB on overflow anyway */
    memcpy(tmp, s + start, len);
    tmp[len] = '\0';
    return atoi(tmp);
}

/* Combine subtrees at positions *i1 and *i2, writing to outbuf.
   Advances *i1 and *i2 past their respective subtrees. */
static void combine(const char *s1, size_t *i1, const char *s2, size_t *i2) {
    if (is_empty_at(s1, *i1) || is_empty_at(s2, *i2)) {
        /* Output empty and do not recurse below; still must consume both inputs' subtrees. */
        out_str("()");
        skip_subtree(s1, i1);
        skip_subtree(s2, i2);
        return;
    }

    /* Both non-empty nodes: (<L1>)[v1](<R1>) and (<L2>)[v2](<R2>) */
    out_ch('(');
    (*i1)++; /* '(' */
    (*i2)++; /* '(' */
    combine(s1, i1, s2, i2); /* left */
    (*i1)++; /* ')' */
    (*i2)++; /* ')' */
    out_ch(')');

    out_ch('[');
    (*i1)++; /* '[' */
    (*i2)++; /* '[' */
    /* parse_value_at expects *i at '[', so step back one */
    (*i1)--;
    (*i2)--;
    int v1 = parse_value_at(s1, i1);
    int v2 = parse_value_at(s2, i2);
    int sum = v1 + v2;
    out_int(sum);
    out_ch(']');

    out_ch('(');
    (*i1)++; /* '(' */
    (*i2)++; /* '(' */
    combine(s1, i1, s2, i2); /* right */
    (*i1)++; /* ')' */
    (*i2)++; /* ')' */
    out_ch(')');
}

int main(void) {
    static char t1[INBUF_SIZE];
    static char t2[INBUF_SIZE];

    if (scanf("%999999s %999999s", t1, t2) != 2) {
        return 0; /* No explicit error handling required. */
    }

    size_t i1 = 0, i2 = 0;
    outpos = 0;
    combine(t1, &i1, t2, &i2);

    /* Null-terminate for printing convenience; overflow is UB per spec. */
    outbuf[outpos] = '\0';
    fputs(outbuf, stdout);
    fputc('\n', stdout);
    return 0;
}
