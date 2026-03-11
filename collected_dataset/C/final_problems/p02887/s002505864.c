#include "stdio.h"
  
int main(){
  int n,a=1;
  char s[100000];
  scanf("%d ",&n);
  for(int i=0;i<n;i++){scanf("%c",&s[i]);}
  for(int i=0;i<n-1;i++){
    if(s[i]!=s[i+1])a++;
  }
  printf("%d\n",a);
  //for(int i=0;i<n;i++){printf("%c",s[i]);}
  return 0;
}