#include<stdio.h>
#include<stdlib.h>

int compUp(const void* a, const void* b){
    return *(int*)a - *(int*)b;
}

int main(void){
    int N, K;
    scanf("%d%d", &N, &K);
    int h[N];
    int i;
    for(i = 0; i < N; i++){scanf("%d", &h[i]);}

    int d;
    int min = 100000001;
    qsort(h, N, sizeof(int), compUp);
    for(i = 0; (i+K-1) < N; i++){
        d = h[i+K-1] - h[i];
        if(min > d){min = d;}
    }

    printf("%d\n", min);
    return 0;
}