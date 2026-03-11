#include<stdio.h>
#define sentinel 2000000000
#define MAX 500000
int n,cnt;
int L[MAX/2+2],R[MAX/2+2];
void merge(int A[],int left,int mid,int right){
    int n1,n2,i,j,k;
    n1=mid-left;
    n2=right-mid;
    for(i=0;i<n1;i++) L[i]=A[left+i];
    for(i=0;i<n2;i++) R[i]=A[mid+i];
 
    i=j=0;
    L[n1]=R[n2]=sentinel;
    for(k=left;k<right;k++){
    cnt++;
    if(L[i]<=R[j]) A[k]=L[i++];
    else A[k]=R[j++];
    }
}
 
void mergeSort(int A[],int left,int right){
    int mid;
    if(left+1<right){
    mid=(left+right)/2;
    mergeSort(A,left,mid);
    mergeSort(A,mid,right);
    merge(A,left,mid,right);
    }
}
 
int main(void){
    int i;
    int A[MAX];
 
    scanf("%d",&n);
    for(i=0;i<n;i++) scanf("%d",&A[i]);
 
    mergeSort(A,0,n);
     
    for(i=0;i<n;i++){
    if(i) printf(" ");
    printf("%d",A[i]);
    }
    printf("\n");
    printf("%d\n",cnt);
 
    return 0;
}
