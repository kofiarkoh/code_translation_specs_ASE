#include<stdio.h>
int main() {
    int N, s, d, f, g, w, e;
    scanf("%d", &N);
    s = N % 10;
    d = N / 10;
    f = d % 10;
    g = d / 10;
    w = g % 10;
    e = g / 10;

    if (s == f && (f == w || f == e)) {
        printf("Yes\n");
    } else if ((w == f || s == w) && w == e) {
        printf("Yes\n");
    } else {
        printf("No\n");
    }
    return 0;
    }