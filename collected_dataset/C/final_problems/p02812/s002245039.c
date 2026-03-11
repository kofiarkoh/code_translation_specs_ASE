#include<stdio.h>
#include<string.h>

int main(){
  int n;
  char s[56];
  char key[4] = "ABC";
  char *p;
  int i;
  int cnt = 0;
  
  scanf("%d %s", &n, s);

  p = s;
  while(*p != '\0'){
    if(strncmp(p,key,3) == 0) cnt++;

    p++;
  }

  printf("%d\n", cnt);
  
  return 0;
}