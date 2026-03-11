#include<stdio.h>

int func(int t, int* r, int* g, int* b){
    int ans = 0;
    int f=1;
    if(t == *r){
        if(f){
            *r += 1;
        }
        ans++;
        f=0;
    }
    if(t == *g){
        if(f){
            *g += 1;
        }
        ans++;
        f=0;
    }
    if(t == *b){
        if(f){
            *b += 1;
        }
        ans++;
        f=0;
    }
    return ans;
}

int main(){
    int n;
    scanf("%d", &n);
    int r=0, g=0, b=0;
    long long int ans = 1;
    for(int i=0; i<n; i++){
        int a;
        scanf("%d", &a);
        ans = ans*func(a, &r, &g, &b);
        ans = ans % 1000000007;
    }
    printf("%lld", ans);

    return 0;
}