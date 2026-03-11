#include <stdio.h>
#include <math.h>
int main(){
  int x;
  int i;
  int flag=0;
  scanf("%d",&x);
  
  while(1){
    for(i=2;i<x;i++){
      	if(x%i==0){
          flag=1;
          break;
        }
    }
    if(flag==1){
      x++;
      flag=0;
    } else {
      break;
    }
  }
  printf("%d",x);
}
