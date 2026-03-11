#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int check(char *str_a, char *str_b, char *str_c, int a, int b, int c, int turn){
  int ret;
  if (turn == 1){
    if (str_a[a] == 'a'){
      ret = check(str_a, str_b, str_c, a+1, b, c, 1);
    }
    else if(str_a[a] == 'b'){
      ret = check(str_a, str_b, str_c, a+1, b, c, 2);
    }
    else if(str_a[a] == 'c'){
      ret = check(str_a, str_b, str_c, a+1, b, c, 3);
    }
    else if(str_a[a] == '\0'){
      return 1;
    }
  }
  else if (turn == 2){
    if (str_b[b] == 'a'){
      ret = check(str_a, str_b, str_c, a, b+1, c, 1);
    }
    else if(str_b[b] == 'b'){
      ret = check(str_a, str_b, str_c, a, b+1, c, 2);
    }
    else if(str_b[b] == 'c'){
      ret = check(str_a, str_b, str_c, a, b+1, c, 3);
    }
    else{
      return 2;
    }
  }
  else if (turn == 3){
    if (str_c[c] == 'a'){
      ret = check(str_a, str_b, str_c, a, b, c+1, 1);
    }
    else if(str_c[c] == 'b'){
      ret = check(str_a, str_b, str_c, a, b, c+1, 2);
    }
    else if(str_c[c] == 'c'){
      ret = check(str_a, str_b, str_c, a, b, c+1, 3);
    }
    else{
      return 3;
    }
  }
  return ret;
}

int main(){
  char s_a[100];
  char s_b[100];
  char s_c[100];
  int a=0;
  int b=0;
  int c=0;
  int r;

  scanf("%s", &s_a[0]);
  scanf("%s", &s_b[0]);
  scanf("%s", &s_c[0]);

  r = check(s_a, s_b, s_c, a,b,c, 1);
  if(r==1){
    printf("A\n");
  }
  if(r==2){
    printf("B\n");
  }
  if(r==3){
    printf("C\n");
  }

  return 0;
}
