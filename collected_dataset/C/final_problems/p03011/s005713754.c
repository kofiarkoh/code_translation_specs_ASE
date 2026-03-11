#include <stdio.h>

int main(int argc, char const *argv[])
{
    int p = 0, q = 0, r = 0, sum = 0;

    scanf("%d %d %d", &p, &q, &r);
    if ((p <= q) && (p <= r) && (r <= q)) sum += (p+r);
    else if ((p <= q) && (p <= r) && (q <= r)) sum += (p+q);
    else if ((p <= q) && (p <= r) && (q <= r)) sum += (p+q);
    else if ((q <= p) && (q <= r) && (r <= p)) sum += (q+r);
    else if ((q <= p) && (q <= r) && (r <= p)) sum += (q+r);

    printf("%d\n", sum);

    return 0;
}
