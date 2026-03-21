#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <ctype.h>

#define MAXLEN 1024

typedef struct {
    const char *s;
    int n;
    int i;
    int prec[3];   /* precedence: higher value binds tighter */
    int assoc[3];  /* 0 = left, 1 = right */
} Parser;

static int op_index(char c) {
    if (c == '+') return 0;
    if (c == '-') return 1;
    return 2; /* '*' */
}

static int64_t apply_op(int64_t a, int64_t b, char op) {
    if (op == '+') return a + b;
    if (op == '-') return a - b;
    return a * b; /* '*' */
}

static int64_t parse_expr(Parser *p, int min_prec);

static int64_t parse_primary(Parser *p) {
    /* Preconditions guarantee validity; behavior undefined otherwise. */
    if (p->i < p->n && p->s[p->i] == '(') {
        p->i++;
        int64_t v = parse_expr(p, 0);
        if (p->i < p->n && p->s[p->i] == ')') p->i++;
        return v;
    }

    int64_t v = 0;
    while (p->i < p->n && isdigit((unsigned char)p->s[p->i])) {
        v = v * 10 + (p->s[p->i] - '0');
        p->i++;
    }
    return v;
}

static int64_t parse_expr(Parser *p, int min_prec) {
    int64_t lhs = parse_primary(p);

    while (p->i < p->n) {
        char op = p->s[p->i];
        if (op != '+' && op != '-' && op != '*') break;

        int oi = op_index(op);
        int prec = p->prec[oi];
        if (prec < min_prec) break;

        int next_min = prec + (p->assoc[oi] == 0 ? 1 : 0); /* left: +1, right: +0 */
        p->i++; /* consume operator */
        int64_t rhs = parse_expr(p, next_min);
        lhs = apply_op(lhs, rhs, op);
    }

    return lhs;
}

static int64_t eval_with_scheme(const char *s, int n, const int prec[3], const int assoc[3]) {
    Parser p;
    p.s = s;
    p.n = n;
    p.i = 0;
    p.prec[0] = prec[0]; p.prec[1] = prec[1]; p.prec[2] = prec[2];
    p.assoc[0] = assoc[0]; p.assoc[1] = assoc[1]; p.assoc[2] = assoc[2];
    return parse_expr(&p, 0);
}

int main(void) {
    char buf[MAXLEN];
    if (scanf("%1023s", buf) != 1) return 0;

    int n = (int)strlen(buf);

    /* 13 fixed precedence/associativity schemes.
       Precedence values: 0 (lowest), 1, 2 (highest). */
    static const int schemes_prec[13][3] = {
        /* 6 permutations, all left-associative */
        {2,1,0}, /* + > - > * */
        {2,0,1}, /* + > * > - */
        {1,2,0}, /* - > + > * */
        {0,2,1}, /* - > * > + */
        {1,0,2}, /* * > + > - */
        {0,1,2}, /* * > - > + */

        /* 6 permutations, all right-associative */
        {2,1,0},
        {2,0,1},
        {1,2,0},
        {0,2,1},
        {1,0,2},
        {0,1,2},

        /* all equal precedence, left-associative */
        {0,0,0}
    };

    static const int schemes_assoc[13][3] = {
        /* left */
        {0,0,0},{0,0,0},{0,0,0},{0,0,0},{0,0,0},{0,0,0},
        /* right */
        {1,1,1},{1,1,1},{1,1,1},{1,1,1},{1,1,1},{1,1,1},
        /* equal-left */
        {0,0,0}
    };

    int64_t best = eval_with_scheme(buf, n, schemes_prec[0], schemes_assoc[0]);
    for (int k = 1; k < 13; k++) {
        int64_t v = eval_with_scheme(buf, n, schemes_prec[k], schemes_assoc[k]);
        if (v > best) best = v;
    }

    printf("%lld\n", (long long)best);
    return 0;
}
