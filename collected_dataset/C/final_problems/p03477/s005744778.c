#include<stdio.h>
int main(){
  int A,B,C,D,l,r;
  scanf("%d %d %d %d",&A,&B,&C,&D);
  l=A+B;
  r=C+D;
  
  if(l<r){
    puts("Right");
  } else if (l==r){
    puts("Balanced");
  } else{
    puts("Left");
  }
  
  return 0;
}