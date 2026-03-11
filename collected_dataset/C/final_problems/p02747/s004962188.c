#include<stdio.h>
#include<string.h>
int main(void){
  int i,k;
  char a[10];
  scanf("%s",a);
  k=strlen(a);
  for(i=0;i<k;i+=2){
    if(!(a[i]=='h'&&a[i+1]=='i')){
      printf("No");
      return 0;
    }
  }
    printf("Yes");
    return 0;
  }