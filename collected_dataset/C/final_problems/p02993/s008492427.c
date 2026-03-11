#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

int main(){

  char s[5];
  int i;

  scanf("%s", s);

  for(i = 0;i < 3;i++){
    if(s[i] == s[i + 1]){
      printf("Bad\n");
      return 0;
    }
  }

  printf("Good\n");

  return 0;
}
