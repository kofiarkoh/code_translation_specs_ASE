
#include <stdio.h>
#define M 2000001
#define N 100000
int partition(int *,int,int);

int main(){
  int i,k,j,n,r,p=0;
  int A[M];

  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&A[i]);
  }
  r=n-1;
  
  k = partition(A, p, r);
  for(i=0;i<=r;i++){
    if(i>0){
      printf(" ");
    }
    
    if(i==k){
      printf("[%d]",A[i]);
    }
    else{      
      printf("%d",A[i]);
    }
  }
  printf("\n");
  return 0;
}

int partition(int *A,int q,int s)
{
  int x,i,j,t=0,l=0;
  x=A[s];
  i=q-1;
  for(j=q;j<=s-1;j++){
    if(x>=A[j]){
      i++;
      t=A[i];
      A[i]=A[j];
      A[j]=t;
    }
  }
  l=A[s];
  A[s]=A[i+1];
  A[i+1]=l;
  return i+1; 
}