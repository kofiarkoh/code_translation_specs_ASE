#include<stdio.h>

void run(void){
  int n;
  scanf("%d",&n);
  int i;
  for(i=0;i<n;i++){
    int a;
    scanf("%d",&a);
    if(a%2!=0) break;
  }
  printf("%s\n",i<n?"first":"second");
}

int main(void){
  run();
  return 0;
}