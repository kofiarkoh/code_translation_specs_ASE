#include<stdio.h>
#include<string.h>

int main(){
  int n;
  char s1[100], s2[100];
  int t = 0, h = 0;
  scanf("%d", &n);
  for(;n > 0; n--){
    scanf("%s %s", s1, s2);
    if ( strcmp(s1, s2) == 0 ) {t++;h++;}
    else if ( strcmp(s1, s2) > 0 ) t += 3;
    else if ( strcmp(s1, s2) < 0 ) h += 3;
    else {}
  }
  printf("%d %d\n", t, h);
  return 0;
}
