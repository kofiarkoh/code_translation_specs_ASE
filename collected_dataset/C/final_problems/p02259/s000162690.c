#include <stdio.h>

int main(){
  int N;
  int A[100];
  int count = 0,flag = 1,num = 0;
  int i;

  scanf("%d",&N);

  for(i=0;i<N;i++)scanf("%d",&A[i]);

  while(flag==1){
    flag=0;
    for(i=N-1;i>=1;i--){
      if(A[i]<A[i-1]){
	num=A[i];
	A[i]=A[i-1];
	A[i-1]=num;
	count++;
	flag=1;
      }
    }
  }

  for(i=0;i<N-1;i++)printf("%d ",A[i]);
  printf("%d\n",A[i]);

  printf("%d\n",count);
  return 0;
}
	  
    

