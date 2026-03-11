#include <stdio.h>
#include <string.h>

int main(){
  char s[200010] = {};
  int l;
  int i_A, i_Z;

  scanf("%s", s);
  l = strlen(s);

  for(int i = 0; i < l; i++){
    if(s[i] == 'A'){
      i_A = i;
      break;
    }
  }
  for(int i = 0; i < l; i++){
    if(s[i] == 'Z'){
      i_Z = i;
    }
  }
  printf("%d\n", i_Z - i_A + 1);
  return 0;
}
