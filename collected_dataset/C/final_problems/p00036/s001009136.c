#include <stdio.h>
#include <math.h>

int main(void) {
    int pat[4], h, i, j, v, minv;
    char *r, buf[9];

    while (1) {
        for (i = 0, h = 0, minv = 255;
             i < 8 && (r = gets(buf)) != NULL; i++) {
            for (j = 0, v = 0; j < 8; j++) {
                v <<= 1;
                v += buf[j] == '1';
            }
            if (v > 0) {
                pat[h++] = v;
                if (v < minv) minv = v;
            }
        }
        if (r == NULL) return 0;
        gets(buf); // blank line;
        for (i = 0; i < h; i++) {
            pat[i] /= minv;
        }
        if (h == 1) {
            puts("C");
        }
        else if (h == 4) {
            puts("B");
        }
        else if (h == 2) {
            if (pat[0] == 2) {
                puts("E");
            }
            else if (pat[1] == 2) {
                puts("G");
            }
            else {
                puts("A");
            }
        }
        else {
            if (pat[0] == 1) {
                puts("D");
            }
            else {
                puts("F");
            }
        }
    }

    return 0;
}