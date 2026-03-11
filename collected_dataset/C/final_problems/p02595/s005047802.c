#include <stdio.h>
int main(void){
    // Your code here!
    long long int i,n, d,x[200001], y[200001], ans = 0;
    scanf("%lld%lld", &n, &d);
    for (i = 0; i < n; i++)
    {
        scanf("%lld%lld", x + i, y + i);
        if (x[i]*x[i]+y[i]*y[i]<=d*d)
            ans++;
    }
    printf("%lld",ans);
}
