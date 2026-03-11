#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>

#define REP(i,n) for(i=0;i<n;i++)
#define ll long long

int main(void){
    int i;
    int N, sum = 0;
    scanf("%d",&N);

    int a[N], b[N], c[N-1];
    REP(i,N){
        scanf("%d",&a[i]);
    }
    REP(i,N){
        scanf("%d",&b[i]);
        sum += b[i];
    }
    REP(i,N-1){
        scanf("%d",&c[i]);
    }

    REP(i,N-1){
        if(a[i+1]==a[i]+1) sum += c[a[i]-1];
    }

    printf("%d",sum);
    
    return 0;
}