#include<stdio.h>
int main(void){
  int A,B,i=0;
  char N[12]={'\0'};
  scanf("%d %d",&A,&B);
  scanf("%s",N);
  while(N[i]!='\0'){
    if((i!=A && N[i]=='-') || (i==A && N[i]!='-')){
      printf("No\n");
      return 0;
    }
    i++;
  }
  printf("Yes\n");
  return 0;
}