#include <stdio.h>
#include <string.h>

#define MAXN 1004

static unsigned char g[MAXN][MAXN]; /* feasibility matrix: g[left][right] in {0,1} */

static int N;

/* Matching arrays:
   - matchR[r] = left matched to right r, or -1
   - matchL[l] = right matched to left l, or -1
*/
static int matchR[MAXN];
static int matchL[MAXN];

/* DFS visitation marks for Kuhn algorithm */
static unsigned char visR[MAXN];
static int visToken;

/* Optional forced edge (forceL -> forceR) for feasibility test */
static int forceL, forceR;

static int dfs_augment(int l) {
    for (int r = 0; r < N; r++) {
        if (!g[l][r]) continue;
        if (visR[r] == (unsigned char)visToken) continue;
        visR[r] = (unsigned char)visToken;

        int l2 = matchR[r];
        if (l2 == -1 || dfs_augment(l2)) {
            matchR[r] = l;
            matchL[l] = r;
            return 1;
        }
    }
    return 0;
}

/* Computes maximum matching size under current g[][], optionally forcing (forceL,forceR).
   Returns size of matching found (Kuhn; sufficient for N<=1004).
*/
static int max_matching_with_optional_force(int useForce) {
    for (int i = 0; i < N; i++) {
        matchL[i] = -1;
        matchR[i] = -1;
    }

    int matched = 0;

    if (useForce) {
        /* If forced edge is not allowed, impossible */
        if (forceL < 0 || forceL >= N || forceR < 0 || forceR >= N) return 0;
        if (!g[forceL][forceR]) return 0;

        matchL[forceL] = forceR;
        matchR[forceR] = forceL;
        matched = 1;
    }

    /* Try to match all left nodes (skip forced left if any) */
    for (int l = 0; l < N; l++) {
        if (useForce && l == forceL) continue;
        visToken++;
        if (visToken == 0) { /* handle unsigned char wraparound */
            memset(visR, 0, sizeof(visR));
            visToken = 1;
        }
        if (dfs_augment(l)) matched++;
    }

    return matched;
}

int main(void) {
    int N_in;
    if (scanf("%d", &N_in) != 1) return 0;

    N = N_in - 1;

    /* Per spec: behavior undefined if N out of bounds; we do not add explicit handling. */
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            int x;
            if (scanf("%d", &x) != 1) return 0;
            g[i][j] = (unsigned char)(x ? 1 : 0);
        }
    }

    if (N == 0) {
        /* Acceptance and no per-left lines */
        puts("Yes");
        return 0;
    }

    /* First, decide if a perfect matching exists */
    visToken = 1;
    memset(visR, 0, sizeof(visR));
    int mm = max_matching_with_optional_force(0);
    if (mm != N) {
        puts("No");
        return 0;
    }

    puts("Yes");

    /* For each left i, output a right index (1..N) that can be part of some perfect matching.
       Narrow interpretation: choose the smallest right r such that forcing (i,r) still allows
       a perfect matching. This guarantees existence of a perfect matching containing (i,r).
    */
    for (int i = 0; i < N; i++) {
        int chosen = -1;
        for (int r = 0; r < N; r++) {
            if (!g[i][r]) continue;
            forceL = i;
            forceR = r;

            visToken = 1;
            memset(visR, 0, sizeof(visR));
            int mm2 = max_matching_with_optional_force(1);
            if (mm2 == N) {
                chosen = r;
                break;
            }
        }

        /* Under acceptance invariant, a choice must exist for each i. */
        if (chosen < 0) chosen = 0; /* unspecified fallback; should not occur if spec holds */

        printf("%d\n", chosen + 1);
    }

    return 0;
}
