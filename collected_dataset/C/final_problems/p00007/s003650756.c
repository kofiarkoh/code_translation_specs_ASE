#include <stdio.h>
int main(){
  long int a=100000,b=0;
  int i,j;
  scanf("%d",&i);
  for(j=0;j<i;j++){
    a=(a*0.05)+a;
    b=a;
    a=a/1000;
    if(b%100 != 0){
      a+=1;
    }
    a=a*1000;
  }
  printf("%d\n",a);
  return(0);
}