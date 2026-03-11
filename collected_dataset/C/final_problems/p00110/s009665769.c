#include <stdio.h>
#include <string.h>
#include <math.h>

#define DIGIT (pow(10, (strlen(num) - 1 -j)))

int solved(char num[100], int i){
  int s = 0, j;
  for(j = 0;j < strlen(num);j++){
    if(num[j] == 'X')
      s += i * DIGIT;
    else 
      s += (num[j] - '0') * DIGIT;
  }
  return s;
}

int main(){
  int a, b, c, i, j;
  char calc[100], *f, *s, *ans;
  while(scanf("%s", calc) != EOF){
    f = strtok(calc, "+");
    s = strtok(NULL, "=");
    ans = strtok(NULL, ",");
    
    for(i = 0;i < 10;i++)
      if(solved(f, i) + solved(s, i) == solved(ans, i)){
	printf("%d\n", i);
	break;
      }
    if(i > 9)
      puts("NA");
  }
  return 0;
}