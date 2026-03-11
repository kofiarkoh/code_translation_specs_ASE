#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define lld long long int

void initStrArray(char** p, int n, int len){
    for(int i=0; i<n; i++){
        *(p + i) = (char*)malloc(sizeof(char) * (len + 1));
    }
}

int main(){
    int n;
    scanf("%d", &n);
    char* w[n];
    initStrArray(w, n, 101);
    int t[n];
    rep(i, 0, n){
        scanf("%s %d", w[i], &t[i]);
    }
    int ans = 0;
    char x[101];
    scanf("%s", x);
    int f=0;
    rep(i, 0, n){
        if(f){
            ans += t[i];
        }
        if(strcmp(w[i], x) == 0){
            f = 1;
        }
    }
    printf("%d", ans);

    return 0;
}