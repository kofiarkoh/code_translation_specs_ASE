#include<stdio.h>
int main(){
  int a,b,N;
  int start[100],finish[100];
  int i,flag=0;
  
  scanf("%d%d",&a,&b);
  scanf("%d",&N);

  for(i=0;i<N;i++){
    scanf("%d%d",&start[i],&finish[i]);
    if((a<=start[i] && start[i]<b) || (a<finish[i] && finish[i]<=b))
      flag=1;
  }
  printf("%d\n",flag);
   return 0;
}