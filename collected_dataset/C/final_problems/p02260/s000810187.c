#include <stdio.h>

int main() {
    int i,j,minj,temp,A[100],N,flag=0;
    
    scanf("%d",&N);
    for(i=0;i<N;i++){
        scanf("%d",&A[i]);
    }
    
    for(i=0;i<N;i++){
        minj = i;
        for(j=i;j<N;j++){
            if(A[j] < A[minj])minj = j;
        }
        temp=A[i];
        A[i] = A[minj] ;
        A[minj]=temp;
        if(temp!=A[i])flag++;
    }
    
    for(i=0;i<N;i++){
        printf("%d ",A[i]);
    }
    printf("\n%d\n",flag);
    
}


