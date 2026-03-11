#include<stdio.h>

int main(){
  int N,ct,i;
  char s[200001];
  scanf("%d",&N);
  scanf("%s",s);
  ct = 0;
  for(i = 0;i < N;i++){
    if(s[i] == '#'){ct++;}
  }
  printf("%d\n",ct>N/2?N-ct:ct);
  return 0;
}