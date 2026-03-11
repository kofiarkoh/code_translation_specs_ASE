#include <stdio.h>
int main(){
 int K,N,max,i,j,temp;
 scanf("%d %d",&K,&N);
  
  int A[N];
  for(i=0;i<N;i++){
    scanf("%d",&A[i]);
  }
  
  
  for(j=0;j<N-1;j++){
  for(i=0;i<N-j-1;i++){
    if(A[i]>A[i+1]){
      temp=A[i+1];
      A[i+1]=A[i];
      A[i]=temp;
      }
  }
 
  }
      
 
  temp=A[1]-A[0];
  for(i=0;i<N-2;i++){
   if((A[i+1]-A[i])<(A[i+2]-A[i+1]))
     temp=A[i+2]-A[i+1];
  }


  if(temp<K+A[0]-A[N-1])
    temp=K-A[N-1]+A[0];
  
  printf("%d",K-temp);
  
}