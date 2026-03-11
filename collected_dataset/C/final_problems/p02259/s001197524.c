#include <stdio.h>

int main(){
  int N;
  scanf("%d",&N);
  int A[N],i,j,cnt=0,temp;
  for(i=0;i<N;i++)scanf("%d",&A[i]);

  for(i=0;i<N;i++){
  temp=0;
    for(j=i;j<N;j++){
        if(A[i]>A[j]){
            temp=A[i];
            A[i]=A[j];
            A[j]=temp;
            cnt++;
        }
    }
  }
  for(i=0;i<N;i++)printf("%d ",A[i]);
  printf("\n");
  printf("%d\n",cnt);
  return 0;
}