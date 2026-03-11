#include<stdio.h>
int main(){
  int a;
  char S[256];

  scanf("%d",&a);
  scanf("%s",S);
  
  if(a>3199){
    printf("%s\n",S);
  }else{
    printf("red\n");
  }
  return 0;
}
