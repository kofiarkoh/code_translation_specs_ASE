#include<stdio.h>
#include<string.h>
int main(){
  char s[100002];
  scanf("%s",s);
  int n,i,count=0;
  n=strlen(s);
  for(i=0;i<n;i++){
    if(i%2==0 && s[i]=='p')
      count--;
    else if(i%2!=0 && s[i]=='g')
      count++;
  }
  printf("%d\n",count);
  return 0;
}
