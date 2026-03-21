#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int a, b;
} Rect;

static int check_configuration(const Rect faces[6], const int rot[6]) {
    /* Face indices:
       0: Front, 1: Back, 2: Left, 3: Right, 4: Top, 5: Bottom

       For each face i, its oriented sides are:
         w(i) = rot[i] ? faces[i].b : faces[i].a
         h(i) = rot[i] ? faces[i].a : faces[i].b

       Adjacency equalities for a cuboid net in 3D:
         Front-Back: same dimensions
         Left-Right: same dimensions
         Top-Bottom: same dimensions

         Front shares vertical edges with Left/Right and horizontal edges with Top/Bottom:
           Front.h == Left.h == Right.h
           Front.w == Top.w == Bottom.w

         Left shares its other dimension with Top/Bottom:
           Left.w == Top.h == Bottom.h

       These constraints are sufficient and necessary for a consistent cuboid with edge lengths (X,Y,Z).
    */
    int w[6], h[6];
    for (int i = 0; i < 6; i++) {
        if (rot[i]) {
            w[i] = faces[i].b;
            h[i] = faces[i].a;
        } else {
            w[i] = faces[i].a;
            h[i] = faces[i].b;
        }
    }

    /* Opposite faces must match exactly (with their chosen orientations). */
    if (w[0] != w[1] || h[0] != h[1]) return 0;
    if (w[2] != w[3] || h[2] != h[3]) return 0;
    if (w[4] != w[5] || h[4] != h[5]) return 0;

    /* Around the box: */
    if (h[0] != h[2]) return 0;
    if (h[0] != h[3]) return 0;

    if (w[0] != w[4]) return 0;
    if (w[0] != w[5]) return 0;

    if (w[2] != h[4]) return 0;
    if (w[2] != h[5]) return 0;

    return 1;
}

static int search(const Rect input[6]) {
    int perm[6] = {0, 1, 2, 3, 4, 5};

    /* Generate all permutations of 6 rectangles assigned to 6 faces. */
    for (int i0 = 0; i0 < 6; i0++) {
        perm[0] = i0;
        for (int i1 = 0; i1 < 6; i1++) if (i1 != i0) {
            perm[1] = i1;
            for (int i2 = 0; i2 < 6; i2++) if (i2 != i0 && i2 != i1) {
                perm[2] = i2;
                for (int i3 = 0; i3 < 6; i3++) if (i3 != i0 && i3 != i1 && i3 != i2) {
                    perm[3] = i3;
                    for (int i4 = 0; i4 < 6; i4++) if (i4 != i0 && i4 != i1 && i4 != i2 && i4 != i3) {
                        perm[4] = i4;
                        for (int i5 = 0; i5 < 6; i5++) if (i5 != i0 && i5 != i1 && i5 != i2 && i5 != i3 && i5 != i4) {
                            perm[5] = i5;

                            Rect faces[6];
                            for (int f = 0; f < 6; f++) faces[f] = input[perm[f]];

                            /* For each assignment, try all independent rotations (2^6). */
                            for (int mask = 0; mask < 64; mask++) {
                                int rot[6];
                                for (int f = 0; f < 6; f++) rot[f] = (mask >> f) & 1;
                                if (check_configuration(faces, rot)) return 1;
                            }
                        }
                    }
                }
            }
        }
    }
    return 0;
}

int main(void) {
    Rect r[6];
    for (int i = 0; i < 6; i++) {
        if (scanf("%d %d", &r[i].a, &r[i].b) != 2) {
            return 1; /* produce no result line */
        }
    }

    if (search(r)) {
        puts("YES");
    } else {
        puts("NO");
    }
    return 0;
}
