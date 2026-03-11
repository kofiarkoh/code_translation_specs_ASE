#include <stdio.h>
#define N 100000
#define M 50000
int binary (int, int, int *,int *);

int main (){
  int m,n,a,b,A[N],B[M],ans,i;
  
  scanf("%d",&m);
  for(i=0;i<m;i++){
    scanf("%d",&a);
    A[i]=a;
  }
  
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&b);
    B[i]=b;
  }

  ans =  binary(m,n,B,A);

  printf("%d\n",ans);

  return 0;
}

int binary (int m,int n,int B[],int A[]){
  int i,left,right,center;
  int count = 0;
  for(i=0;i<n;i++){
    left=0;
    right=m;

    while(left<right){
    center=(left+right)/2;
    if(A[center]==B[i]){
      count++;
      break;
    }
    else if(B[i]<A[center])
      right = center;
    else left = center+1;
    }
    
    
  }
  
  return count;
}

