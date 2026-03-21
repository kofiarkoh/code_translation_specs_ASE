#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int a, b;
} Rect;

/* Apply rotation choice: rot==0 => (a,b), rot==1 => (b,a) */
static void oriented(const Rect *r, int rot, int *x, int *y) {
    if (rot) { *x = r->b; *y = r->a; }
    else     { *x = r->a; *y = r->b; }
}

/*
  Fixed face positions: 0..5 correspond to:
    0: Front, 1: Back, 2: Left, 3: Right, 4: Top, 5: Bottom

  For each face, we interpret its oriented sides as:
    - For Front/Back:   (width, height)
    - For Left/Right:   (depth, height)
    - For Top/Bottom:   (width, depth)

  Adjacency constraints (shared edges must match):
    Front-Back:   width and height
    Left-Right:   depth and height
    Top-Bottom:   width and depth
    Front-Left:   height and depth
    Front-Right:  height and depth
    Back-Left:    height and depth
    Back-Right:   height and depth
    Front-Top:    width and depth
    Front-Bottom: width and depth
    Back-Top:     width and depth
    Back-Bottom:  width and depth
    Left-Top:     width and height
    Left-Bottom:  width and height
    Right-Top:    width and height
    Right-Bottom: width and height
*/
static int valid_configuration(const Rect faces[6], const int rot[6]) {
    int fw, fh, bw, bh, ld, lh, rd, rh, tw, td, bw2, bd;

    oriented(&faces[0], rot[0], &fw, &fh);   /* Front:  (w,h) */
    oriented(&faces[1], rot[1], &bw, &bh);   /* Back:   (w,h) */
    oriented(&faces[2], rot[2], &ld, &lh);   /* Left:   (d,h) */
    oriented(&faces[3], rot[3], &rd, &rh);   /* Right:  (d,h) */
    oriented(&faces[4], rot[4], &tw, &td);   /* Top:    (w,d) */
    oriented(&faces[5], rot[5], &bw2, &bd);  /* Bottom: (w,d) */

    /* Opposite faces must match exactly (both dimensions) */
    if (fw != bw || fh != bh) return 0;      /* Front vs Back */
    if (ld != rd || lh != rh) return 0;      /* Left vs Right */
    if (tw != bw2 || td != bd) return 0;     /* Top vs Bottom */

    /* Front/Back with Left/Right: shared edges are (h) and (d) */
    if (fh != lh || ld != td) return 0;      /* Front-Left: h, d */
    if (fh != rh || rd != td) return 0;      /* Front-Right: h, d */
    if (bh != lh || ld != td) return 0;      /* Back-Left: h, d */
    if (bh != rh || rd != td) return 0;      /* Back-Right: h, d */

    /* Front/Back with Top/Bottom: shared edges are (w) and (d) */
    if (fw != tw || td != ld) return 0;      /* Front-Top: w, d */
    if (fw != bw2 || bd != ld) return 0;     /* Front-Bottom: w, d */
    if (bw != tw || td != ld) return 0;      /* Back-Top: w, d */
    if (bw != bw2 || bd != ld) return 0;     /* Back-Bottom: w, d */

    /* Left/Right with Top/Bottom: shared edges are (w) and (h) */
    if (tw != fw || lh != fh) return 0;      /* Left-Top: w, h */
    if (bw2 != fw || lh != fh) return 0;     /* Left-Bottom: w, h */
    if (tw != fw || rh != fh) return 0;      /* Right-Top: w, h */
    if (bw2 != fw || rh != fh) return 0;     /* Right-Bottom: w, h */

    return 1;
}

static void swap_int(int *a, int *b) {
    int t = *a; *a = *b; *b = t;
}

int main(void) {
    Rect input[6];
    for (int i = 0; i < 6; i++) {
        if (scanf("%d %d", &input[i].a, &input[i].b) != 2) {
            /* Early termination with non-zero status and no output */
            return 1;
        }
    }

    /* Enumerate all permutations of rectangles across 6 face positions */
    int p[6] = {0, 1, 2, 3, 4, 5};

    /* Heap's algorithm for permutations */
    int c[6] = {0, 0, 0, 0, 0, 0};

    for (;;) {
        Rect faces[6];
        for (int i = 0; i < 6; i++) faces[i] = input[p[i]];

        /* Enumerate all 2^6 rotation choices */
        for (int mask = 0; mask < (1 << 6); mask++) {
            int rot[6];
            for (int i = 0; i < 6; i++) rot[i] = (mask >> i) & 1;

            if (valid_configuration(faces, rot)) {
                printf("yes\n");
                return 0;
            }
        }

        /* Next permutation (Heap) */
        int i = 0;
        while (i < 6) {
            if (c[i] < i) {
                if (i % 2 == 0) swap_int(&p[0], &p[i]);
                else            swap_int(&p[c[i]], &p[i]);
                c[i]++;
                break;
            } else {
                c[i] = 0;
                i++;
            }
        }
        if (i == 6) break; /* exhausted all permutations */
    }

    printf("no\n");
    return 0;
}
