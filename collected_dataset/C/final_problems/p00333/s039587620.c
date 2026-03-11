#include <stdio.h>

int main(void)
{

    int w, h, c, k, p;

    scanf("%d %d %d", &w, &h, &c);

    k = w * h;

    if(w > h)
        p = h;
    else
        p = w;

    while(k % (p * p))
        p--;

    printf("%d\n", (k / (p * p)) * c);

    return 0;
}