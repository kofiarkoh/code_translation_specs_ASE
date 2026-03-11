#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

#define REP(i,n) for(int i=0;i<n;i++)
#define ll long long
#define PI M_PI

int max(int array[], int size){
    int tmp = array[0];
    REP(i,size-1){
        if(array[i] <= array[i+1]) tmp = array[i+1];
    }
    return tmp;
}

int min(int array[], int size){
    int tmp = array[0];
    REP(i,size-1){
        if(array[i] >= array[i+1]) tmp = array[i+1];
    }
    return tmp;
}

int main(void){
    int i;
    int N, M;
    scanf("%d""%d",&N, &M);

    int L[M], R[M];
    REP(i,M){
        scanf("%d""%d",&L[i], &R[i]);
    }
    
    printf("%d",min(R,M)-max(L,M)+1);
    
    
    //printf("%d",ctr);
    return 0;
}