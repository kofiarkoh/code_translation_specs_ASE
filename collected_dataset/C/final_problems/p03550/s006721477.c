#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define revrep(i,begin,end) for(int i=begin; i>end; i--)
#define lld long long int

int main(){
    int n, z, w;
    scanf("%d %d %d", &n, &z, &w);
    int a[n];
    rep(i, 0, n){
        scanf("%d", &a[i]);
    }
    int ans;
    if(n == 1){
        ans = abs(a[0] - w);
    }else{
        int p, q;
        p = abs(a[n-1] - w);
        q = abs(a[n-2] - a[n-1]);
        ans = p > q ? p : q;
    }
    printf("%d", ans);

    return 0;
}