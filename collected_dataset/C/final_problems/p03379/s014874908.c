#include<stdio.h>

int partition(int A[], int n, int p, int r){
    int i,j;
    int x,t;
    x=A[r];
    i=p-1;
    for(j=p;j<r;j++){
        if(A[j]<=x){  //xより小さいとき
            i++;
            t=A[i];
            A[i]=A[j];
            A[j]=t;
        }
    }
    t=A[i+1];
    A[i+1]=A[r];
    A[r]=t;
    return i+1;
}

void quickSort(int A[], int n, int p, int r){
    int q;
    if(p<r){
        q=partition(A,n,p,r);
        quickSort(A,n,p,q-1);
        quickSort(A,n,q+1,r);
    }
}

int main(void){
    int n,i;
    scanf("%d",&n);
    int x[n],b[n];
    for(i=0;i<n;i++){
        scanf("%d",&x[i]);
        b[i] = x[i];
    }
    
    quickSort(b,n,0,n-1);
    
    for(i=0;i<n;i++){
        if(x[i]<b[n/2]){
            printf("%d\n",b[n/2]);
        }else{
            printf("%d\n",b[(n/2)-1]);
        }
    }
    return 0;
}
