#include<stdio.h>
#define ARR_SUM(a, b) (  ((a) + (b))*((b) - (a) + 1) / 2 )

int main()
{
    int N, L, ans;
    scanf("%d %d", &N, &L);
    int max = L + N - 1;
    if(L >= 0) { ans = ARR_SUM(L+1, max); }
    else if(L < 0 && max >= 0) { ans = ARR_SUM(L, max); }
    else { ans = ARR_SUM(L, max-1); }
    printf("%d\n", ans);
    return 0;
}