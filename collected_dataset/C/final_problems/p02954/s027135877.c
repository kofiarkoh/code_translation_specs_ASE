#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int main(void){
    char s[1000000];
    long *A;
    long *B;
    int N;
    scanf("%s", s);
    N = strlen(s);
    A = (long *)calloc(N, sizeof(long));
    B = (long *)calloc(N, sizeof(long));
    for(int i=0; i<N; i++)
        B[i]=1;
    int k=0;
    int b=0;
    while (1)
    {   
        if(b==5)break;
        for(int i=0; i<N; i++){
            if(s[i]=='R'){
               A[i+1]+=B[i];
            }
            if(s[i]=='L'){
                A[i-1]+=B[i];
            }
        }
        for(int i=0; i<N; i++){
                B[i]=A[i];
                A[i]=0;
        }
        k++;
        if(k==100){k=0; b+=1;}
    }
    for(int i=0; i<N; i++)
        printf("%ld ", B[i]);
    printf("\n");
    free(A);
    free(B);
    return 0;
}
