#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define revrep(i,begin,end) for(int i=begin; i>end; i--)
#define lld long long int

int main(){
    int n;
    scanf("%d", &n);
    int a[n+2], sum[n+2];
    a[0] = 0; sum[0] = 0;
    rep(i, 1, n+1){
        scanf("%d", &a[i]);
        sum[i] = sum[i-1] + abs(a[i] - a[i-1]);
    }
    a[n+1] = a[n-1]; sum[n+1] = sum[n-1];
    /*rep(i, 0, n+2){
        printf("%d ", a[i]);
    }
    printf("\n");
    rep(i, 0, n+2){
        printf("%d ", sum[i]);
    }
    printf("\n");*/
    rep(i, 1, n+1){
        int ans, l = n;
        if(i==n){
            l = n-1;
        }
        int x = sum[i-1], y = abs(a[i-1] - a[i+1]), z = sum[l] - sum[i+1];
        ans = x + y + z + abs(a[l]);
        printf("%d\n", ans);
    }

    return 0;
}