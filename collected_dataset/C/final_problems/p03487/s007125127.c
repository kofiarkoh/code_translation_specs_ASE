#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define revrep(i,begin,end) for(int i=begin; i>end; i--)
#define lld long long int


int cmpAsc(const void* a, const void* b){
    int x = *(int*)a, y = *(int*)b;
    if(x > y){
        return 1;
    }
    if(x < y){
        return -1;
    }
    return 0;
}

int main(){
    int n;
    scanf("%d", &n);
    int a[n];
    int ans = 0;
    rep(i, 0, n){
        scanf("%d", &a[i]);
    }
    qsort(a, n, sizeof(int), cmpAsc);
    int x = 0;
    while(x < n){
        int y = x+1;
        while(y<n){
            if(a[x] != a[y]){
                break;
            }
            y++;
        }
        if(y - x < a[x]){
            ans += y - x;
        }else if(a[x] < y - x){
            ans += (y - x) - a[x];
        }
        x = y;
    }
    printf("%d", ans);

    return 0;
}