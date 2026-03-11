#include <stdio.h>
#include <stdlib.h>

int main()
{
    int n, k, x;
    scanf("%d", &n);
    scanf("%d", &k);
    int ans = 0;
    while(n){
        scanf("%d", &x);
        if(abs(x - k) < x) ans+= abs(x - k) * 2;
        else ans += x * 2;
        n--;
    }
    printf("%d", ans);
    return 0;
}
