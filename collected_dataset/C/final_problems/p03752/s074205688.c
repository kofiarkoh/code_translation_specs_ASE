#include <stdio.h>
long long int min(long long int a, long long int b){return a < b ? a : b;}
int max(int a, int b){return a > b ? a : b;}
int main(void){
    int n, k;
    scanf("%d%d", &n, &k);
    int a[15];
    int i, j;
    for(i = 0; i < n; i++){
        scanf("%d", &a[i]);
    }
    long long int ans = 999999999999999;
    int bit;
    int a_temp[15];
    int can_see = 0;
    int maxh = 0;
    long long int cost;
    for(bit = 0; bit < (1 << n); ++bit){
        cost = 0;
        can_see = 0;
        for(i = 0; i < n; i++){
            a_temp[i] = a[i];
        }
        for(i = 0; i < n; i++){
            if(bit & (1 << i)){
                can_see++;
                //0番目から(i-1)番目までの建物の高さの最大値を求める
                //ただしi = 0のときは0
                maxh = 0;
                for(j = 0; j <= i - 1; j++){
                    maxh = max(maxh, a_temp[j]);
                }
                //i番目の高さをmaxh+1以上にする
                if(a_temp[i] < maxh + 1){
                    cost += maxh + 1 - a_temp[i];
                    a_temp[i] = maxh + 1;
                }
            }
        }
        if(can_see >= k){
            ans = min(ans, cost);
        }
    }
    printf("%lld\n", ans);
    return 0;
}