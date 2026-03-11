#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int main()
{
  int cnt=0;
  int max=0;
  char S[1001], T[1001];
  scanf("%s %s", S, T);
  int len1=strlen(S);
  int len2=strlen(T);
  
  for(int i=0; i<len1-len2; i++){
    cnt=0;
    for(int j=i; j<len2; j++) {
    	if(S[j]==T[j-i]){
      		cnt++;
        }
    }
    if(cnt>max)max=cnt;
  }
  printf("%d", len2-max);
  
}