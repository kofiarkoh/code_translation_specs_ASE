#include <stdio.h>

typedef struct {
    int k;
    int x[10];
    int y;
} rewrite_t;


typedef struct _seqelm_t seqelm_t;
struct _seqelm_t {
    int v;
    int len;
    seqelm_t *sibling;
    seqelm_t *right;
};

seqelm_t seqelm[100000];
int elmidx;

seqelm_t *seq_new_elm(int v) {
    seqelm_t *elm = &seqelm[elmidx++];
    elm->v = v;
    elm->len = 0;
    elm->sibling = NULL;
    elm->right = NULL;
    return elm;
}

seqelm_t *seq_right(seqelm_t *left, int x) {
    seqelm_t *elm = left->right;
    while (elm != NULL) {
        if (elm->v == x) return elm;
        elm = elm->sibling;
    }
    return elm;
}

seqelm_t *seq_put_right(seqelm_t *left, int x) {
    seqelm_t *elm = seq_new_elm(x);
    elm->sibling = left->right;
    left->right = elm;
    return elm;
}

int seq_exists(seqelm_t *root, int s[], int n) {
    int i;
    seqelm_t *elm, *left = root;
    for (i = 0; i < n; i ++) {
        elm = seq_right(left, s[i]);
        if (elm == NULL) return 0;
        left = elm;
    }
    if (left->len > 0) return 1;
    return 0;
}

// return: 0: already existing, length: new pattern
int seq_register(seqelm_t *root, int s[], int n) {
    int i;
    seqelm_t *elm, *left = root;
    for (i = 0; i < n; i++) {
        elm = seq_right(left, s[i]);
        if (elm == NULL) {
            left = seq_put_right(left, s[i]);
            if (i == n - 1) {
                left->len = n;
                return n;
            }
        }
        else {
            if (i == n - 1) { // last
                if (elm->len > 0) {
                    return 0;
                }
                else {
                    elm->len = n;
                    return n;
                }
            }
            else {
                left = elm;
            }
        }
    }
}

int _max_length;

void _max_match_length(seqelm_t *elm, int s[], int p, seqelm_t *troot) {
    s[p] = elm->v;
    if (elm->len > 0) {
        if (seq_exists(troot, s, p+1) && _max_length < elm->len) {
            _max_length = elm->len;
        }
    }
    if (elm->right != NULL) _max_match_length(elm->right, s, p+1, troot);
    if (elm->sibling != NULL) _max_match_length(elm->sibling, s, p, troot);
}

int max_match_length(seqelm_t *aroot, seqelm_t *broot) {
    int s[25];
    _max_length = 0;
    _max_match_length(broot->right, s, 0, aroot);
    return _max_length;
}

void print_seq(int s[], int n) {
    int i;
    printf("/ ");
    for (i = 0; i < n; i++) {
        printf("%d", s[i]);
    }
    printf("\n");
}

int _print_line;

void _print_seq_patterns(seqelm_t *elm, int s[], int p) {
    int i;
    s[p] = elm->v;
    if (elm->len > 0) {
        printf("%d: ", ++_print_line);
        for (i = 0; i <= p; i++) {
            printf("%d", s[i]);
        }
        printf("\n");
    }
    if (elm->right != NULL) _print_seq_patterns(elm->right, s, p+1);
    if (elm->sibling != NULL) _print_seq_patterns(elm->sibling, s, p);
}

void print_seq_patterns(seqelm_t *root) {
    int s[25];
    _print_line = 0;
    printf("---\n");
    _print_seq_patterns(root->right, s, 0);
}

void seq_rotate(const int s[], int n, int t[]) {
    int i;
    for (i = 0; i < n; i++) {
        t[(i + n - 1) % n] = s[i];
    }
}

void _make_all_patterns(seqelm_t *root, int s[], int n, rewrite_t rewrite[], int r) {
    int t[25];
    int i, j, l, m, p, q;
    rewrite_t *rw;
    int result = seq_register(root, s, n);
    if (result == 0) return;

    seq_rotate(s, n, t);  _make_all_patterns(root, t, n, rewrite, r);
    for (i = 0; i < r; i++) {
        rw = &rewrite[i];
        for (j = 0, l = n-rw->k+1; j < l; j++) {
            for (m = 0; m < rw->k; m++) {
                if (s[j+m] != rw->x[m]) break;
            }
            if (m == rw->k) {
                for (p = 0, q = 0; p < n; q++) {
                    if (p == j) {
                        t[q] = rw->y;
                        p += rw->k;
                    }
                    else {
                        t[q] = s[p];
                        p++;
                    }
                }
                
                _make_all_patterns(root, t, n-rw->k+1, rewrite, r);
                goto next_rule;
            }
        }
        next_rule:;
    }
}

seqelm_t *make_all_patterns(int s[], int n, rewrite_t rewrite[], int r) {
    seqelm_t *root = seq_new_elm(0);
    _make_all_patterns(root, s, n, rewrite, r);
    return root;
}

int main (void) {
    int n, m, r, i, j, t, length;
    int a[25], b[25];
    rewrite_t rewrite[60], *rw;
    seqelm_t *aseq, *bseq;

    while (scanf("%d%d%d\n", &n, &m, &r) != EOF && (n || m || r)) {
        for (i = 0; i < n; i++) scanf("%d", &a[i]);
        for (i = 0; i < m; i++) scanf("%d", &b[i]);
        for (i = 0; i < r; i++) {
            rw = &rewrite[i];
            scanf("%d", &(rw->k));
            for (j = 0; j < rw->k; j++) {
                scanf("%d", &(rw->x[j]));
            }
            scanf("%d", &(rw->y));
        }

        // make a variation
        elmidx = 0;
        aseq = make_all_patterns(a, n, rewrite, r);
        // print_seq_patterns(aseq);
        bseq = make_all_patterns(b, m, rewrite, r);
        // print_seq_patterns(bseq);
        length = max_match_length(aseq, bseq);
        printf("%d\n", length ? length: -1);
    }
    return 0;
}