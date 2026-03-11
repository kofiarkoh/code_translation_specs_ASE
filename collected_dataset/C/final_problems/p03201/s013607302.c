#include <stdio.h>
#include <stdlib.h>

void MergeSort(long *A,long p,long q,long r) {
    long *t,i=p,j=q+1,k=0,a;
    
    t=(long *)malloc((r-p+1)*sizeof(long));
    
    while(k<r-p+1) {
        if(A[i]<A[j]) a=i++;
        else a=j++;
        
        t[k]=A[a];
        k++;
        
        if(i==q+1) a=j;
        else if(j==r+1) a=i;
        else continue;
        
        while(k<r-p+1) {
            t[k]=A[a];
            k++;
            a++;
        }
    }

    for(i=p;i<=r;i++) A[i]=t[i-p];
    free(t);
    return;
}

void merge(long *A,long s,long e) {
    long m=(s+e)/2;

    if(s!=e) {
        merge(A,s,m);
        merge(A,m+1,e);
        MergeSort(A,s,m,e);
    }

    return;
}

int main() {
    long *A,N,i,k=1,j,c=0;
    int *B;

    scanf("%ld",&N);
    A=(long *)malloc(N*sizeof(long));
    B=(int *)malloc(N*sizeof(int));

    if(A==NULL || B==NULL) exit(1);
    for(i=0;i<N;i++) {
        scanf("%ld",A+i);
        B[i]=0;
    }

    merge(A,0,N-1);
    while(k<A[N-1]) k*=2;

    for(i=N-1;i>=1;i--) {
        if(B[i]==1) continue;
        while(k/2>A[i]) k/=2; 

        for(j=0;j<i;j++) {
            if(A[j]+A[i]>k) break;
            if(B[j]==1) continue;
            if(A[j]+A[i]==k) {
                B[j]=1;
                c++;
                break;
            }
        }
    }

    printf("%ld\n",c);
    return 0;
}