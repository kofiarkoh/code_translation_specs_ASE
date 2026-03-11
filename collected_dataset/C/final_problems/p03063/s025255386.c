#include <stdio.h>
int main(void){
  int N,j=0,cnt=0;
  char s[200000],tmp[0];
  scanf("%d",&N);
  scanf("%s",s);
  for(int i=0;i<=N-1;i++){
    if(s[i]=='#' && s[i+1]=='.'){
    	for(;s[j]=='.';j++);
      	s[i+1]='#';
      	s[j]='.';
        cnt++;
    }
  }
  printf("%d\n",cnt);
  return 0;
}