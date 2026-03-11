#include <stdio.h>
int main(){
    int A,B,K,i,temp;
    
    scanf("%d %d %d",&A,&B,&K);
    
    for(i=0;i<K;i++){
        if(A%2==1){
        A=(A-1)/2;
        temp=A;
        B+=A;
        A=B;
        B=temp;
        }
        else{
        A=A/2;
        temp=A;
        B+=A;
        A=B;
        B=temp;
        }
    }
    
    if(K%2==1)
    printf("%d %d",B,A);
    else
    printf("%d %d",A,B);
    
    return(0);
}