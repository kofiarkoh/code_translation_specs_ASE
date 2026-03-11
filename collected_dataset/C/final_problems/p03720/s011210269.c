#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <stdio.h>
#define REP(i, a, b) for(int i = a; i < b; i++)

int compare(const void* a, const void* b) {
    return *(int*)a - *(int*)b;
}

int main(void){
    int n,m;
    scanf("%d%d",&n,&m);
    int a[m],b[m],count[n];
    REP(i, 0, m) {
        scanf("%d%d",&a[i],&b[i]);
    }
    REP(i, 0, n) {
        count[i] = 0;
    }
    REP(i, 0, m) {
        count[a[i] - 1]++;
        count[b[i] - 1]++;
    }
    REP(i, 0, n) {
        printf("%d\n",count[i]);
    }
    return 0;
}