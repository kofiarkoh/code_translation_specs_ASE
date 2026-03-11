#include <stdio.h>

int main(){
  int A,B;
  scanf("%d %d",&A,&B);
  
  if(9<A){
    printf("-1\n");
  }else if(9<B){
   printf("-1\n");
  }else{
   printf("%d\n",A*B);
  }
  return 0;
}