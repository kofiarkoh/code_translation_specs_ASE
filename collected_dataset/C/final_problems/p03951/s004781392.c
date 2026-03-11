#include <stdio.h>

int main(){
  int n;  scanf("%d", &n);
  char s[102], t[102];
  scanf("%s %s", s, t);
  int ans = 0;
  for(int i=0; i<n; i++){
    int flag = 1;
    for(int j=0; j+i<n; j++)
      if(s[j+i] != t[j]){
	flag = 0;
	break;
      }
    if(flag){
      ans = i + n;
      break;
    }
  }
  if(!ans)  ans = n * 2;

  printf("%d\n", ans);
  return 0;
}
