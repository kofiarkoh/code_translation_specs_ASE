#include <stdio.h>
typedef unsigned long long ll;

ll cal(ll h)
{
    //printf("%lld\n", h);
    if (h == 1)
        return 1;
    return 2 * cal(h / 2) + 1;
}

int main(void)
{
    ll h;
    ll ans = 0;

    scanf("%lld", &h);
    printf("%lld\n", cal(h));

    return 0;
}