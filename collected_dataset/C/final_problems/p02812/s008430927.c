#include <stdio.h>

int main(void){
    int N;
    char S[51];
    char A[] = {"ABC"};
    //printf("%s\n",A);
    int count=0;
    scanf("%d %s",&N,S);
    for(int i=0; i<=N-3; i++){
        for(int j=0; j<3; j++){
            //printf("i=%d j=%d S[i+j]=%c A[j]=%c\n",i,j,S[i+j],A[j]);
            if(S[i+j] == A[j]){
                if(j==2) count++;
            }else{
                break;
            }
        }
    }
    printf("%d",count);
}