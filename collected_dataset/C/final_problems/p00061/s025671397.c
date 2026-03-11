#include <stdio.h>

int main(){
  int team[1000], p[1000], rank[1000], i = 0, j, k, num, point, flag, f, l;

  for(i = 0;;i++){
    scanf("%d,%d", &team[i], &p[i]);
    if(!team[i] && !p[i]) break;
  }
  for(f = 1, i = 30;i >= 0;i--){
    flag = 0;
    for(l = 0;;l++){
      if(!team[l] && !p[l]) break;
      else if(p[l] == i){
	rank[l] = f;
	flag++;
      }
    }
    if(flag != 0) f++;
  }

  while(scanf("%d", &f) != EOF){
    for(i = 0;;i++){
      if(f == team[i]){
	printf("%d\n", rank[i]);
	break;
      }
    }
  }
  return 0;
}