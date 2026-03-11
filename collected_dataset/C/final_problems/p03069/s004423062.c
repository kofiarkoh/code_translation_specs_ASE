#include <stdio.h>

int
main(int argc, char *argv[])
{
  int n; char s[400000] = {}; int k = -1;
  scanf("%d%s", &n, s);
  for(int i = 0; i < n; i++){
    if(s[i] == '#'){
      k = i;
      break;
    }
    else if(i == n - 1){
      printf("%d\n", 0);
    }
  }

  int white = 0; int black = 0; int y = 0;

  if(k >= 0 && k < n - 1){
    for(int i = k + 1; i < n; i++){
      if(s[i] == '#'){
	black++;
      }
      else{
	white++;
      }
      if(s[i] == '#' && s[i + 1] == '.'){
	y++;
      }
    }

    if(y == 0){
      printf("%d\n", 1);
    }

    else{
      

    if(black + 1 < white){
      printf("%d\n", black + 1);
    }
    else{
      printf("%d\n", white);

    }
  }

  
  }
  
  else if(k == n - 1){
    printf("%d\n", 0);
  }
  return 0;
}
