#include<stdio.h>
#include<string.h>

int main(){
  int n;
  char s[1000];
  scanf("%d",&n);
  scanf("%s",s);
  
  int len;
  len=strlen(s);
  
  for(int i=0;i<len;i++){
    s[i]+=n;
    if(s[i]>90) s[i]-=26;
  }
  
  printf("%s",s);
 
  return 0;
}