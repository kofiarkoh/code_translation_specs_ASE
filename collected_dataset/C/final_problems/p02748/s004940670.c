#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

int *A,*B;

int an,bn,mn;

int min(int a,int b){
    return a > b ? b : a;
}

int main(void){
    int i,j;
    int sp;

    scanf("%d %d %d",&an,&bn,&mn);
    
    A = (int*)malloc(sizeof(int)*an);
    B = (int*)malloc(sizeof(int)*bn);

    int sa = INT_MAX,sb = INT_MAX;
    for(i = 0;i < an;i++){
        scanf("%d",&(A[i]));
        sa = min(sa,A[i]);
    }
    for(i = 0;i < bn;i++){
        scanf("%d",&(B[i]));
        sb = min(sb,B[i]);
    }

    sp = sa + sb;
    for(i = 0;i < mn;i++){
        int x,y,c;
        scanf("%d %d %d",&x,&y,&c);
        if(A[x-1] + B[y-1] - c < sp){
            sp = A[x-1] + B[y-1] - c;
        }
    }
    printf("%d\n",sp);

    return 0;
}