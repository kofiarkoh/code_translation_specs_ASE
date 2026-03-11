#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
int main(void){
  int n,a=0,b=0,c=0;
  char s[100005];
  scanf("%s",s);
  n=strlen(s);
  for(int i=0;i<n;i++){
    if(s[i]=='a') a++;
    if(s[i]=='b') b++;
    if(s[i]=='c') c++;
  }
  if(fabs(a-b)<=1&&fabs(a-c)<=1&&fabs(c-b)<=1){
    printf("YES");                                                               
  }else{
    printf("NO");                                                                       }
  return 0;
}                                                                                     