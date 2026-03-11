#include<stdio.h>
 
 
int main(){
  int k,x,i;
  int large,small;
  scanf("%d %d",&x,&k);
  x --;
  if(k+x >1000000){
    large = 1000000;
  }else{
    large = k+x;
  }
  if(k-x < -1000000){
    small = -1000000;
  }else{
    small = k-x;
  }
  for(i= 0;small<=large;small++){
    printf("%d ",small);
  }
  return 0;
}