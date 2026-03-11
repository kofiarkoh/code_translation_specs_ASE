#include <stdio.h>
#define MAX 100001

int main(){
  int i,j,slen,tlen;
  long long c;
  char s[MAX],t[MAX];
  
  scanf("%s",s);
  scanf("%s",t);
  
  for(i=0;s[i]!='\0';i++);
  slen = i;
  for(i=0;t[i]!='\0';i++);
  tlen = i;
  
  c=0;
  i=0;
  j=0;
  while(1){
    if(i==slen){
      c++;
      if(c==MAX){
        printf("-1");
        return 0;
      }
      i=0;
    }
    if(s[i]==t[j]){
      j++;
      if(j==tlen)break;
    }
    i++;
  }
  printf("%d",c*slen+i+1);
  return 0;
}