#include <stdio.h>

int N, Q, i, a, b, b_prev, c, l, r, d[200000];
char s[200000], t[200000], u;

int simulate(int b) {
        for (i = 0; i < Q; i++) {
                if (s[b] == t[i]) {
                        b += d[i];
                        if (b < 0) return -1;
                        if (b >= N) return 1;
                }
        }
        return 0;
}

int main() {
        scanf("%d%d", &N, &Q);
        scanf("%s", s);
        for (i = 0; i < Q; i++) {
                scanf(" %c %c", &t[i], &u);
                d[i] = (u == 'L') ? -1 : 1;
        }

        a = 0;
        b_prev = -1;
        c = N - 1;
        while (1) {
                b = (a + c + 1) / 2;
                if (b == b_prev) break;
                if (simulate(b) < 0)
                        a = b;
                else
                        c = b;
                b_prev = b;
        }
        l = a;

        a = 0;
        b_prev = -1;
        c = N - 1;
        while (1) {
                b = (a + c + 1) / 2;
                if (b == b_prev) break;
                if (simulate(b) > 0)
                        c = b;
                else
                        a = b;
                b_prev = b;
        }
        r = a;

        printf("%d\n", r - l);
        return 0;
}
