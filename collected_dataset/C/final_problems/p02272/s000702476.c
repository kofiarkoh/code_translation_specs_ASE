#include<stdio.h>
#include<stdlib.h>
#define N 500000
#define S 1000000000
#define SENTINEL 100000000000
void Merge(int *,int,int,int);
void MergeSort(int *,int,int);
int A[N],count;
int main(){
  int n,i,left,right;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&A[i]);
    //printf("%d",A[i]);
  }
  right = n;
  left=0;
  MergeSort(A,left,right);

  for(i=0;i<n-1;i++){
    printf("%d ",A[i]);
  }
  printf("%d\n%d\n",A[n-1],count);
  return 0;
}

void MergeSort(int *A,int left, int right){
  int mid;
  if(left+1<right){
    mid =(left + right) / 2;
    MergeSort(A, left, mid);
    MergeSort(A, mid, right);
    Merge(A,left,mid,right);
      }
}

void Merge(int *A, int left, int mid, int right){
  int n1= mid -left;
  int n2 = right - mid;
  int i,j,k;//,*L,*R;
    int L[n1+1],R[n2+1];
  //L=(int *)malloc(sizeof(int)*n1);
  //R=(int *)malloc(sizeof(int)*n2);
  for(i=0;i<n1;i++){
    L[i]=A[left+i];
  }
  for(i=0;i<n2;i++){
    R[i]=A[mid+i];
  }
  L[n1]=SENTINEL;
  R[n2]=SENTINEL;
  i=0;
  j=0;
  for(k=left;k<right;k++){
    if(L[i]<=R[j]){
      A[k]=L[i];
      i++;
    }
    else{
      A[k]=R[j];
      j++;
    }
    count++;
  }
}