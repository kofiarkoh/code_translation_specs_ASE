#include <stdio.h>
int main(void)
{
  char S[4];
  scanf("%s", S);
  if (S[0] == S[1]) {
    if(S[2] == S[3]) {
      printf("Yes");
      return 0;
    }
    else {
      printf("No");
      return 0;
    }
  }
  else if (S[0] == S[2]) {
    if(S[1] == S[3]) {
      printf("Yes");
      return 0;
    }
    else {
      printf("No");
      return 0;
    }
  }
  else if (S[0] == S[3]) {
    if(S[1] == S[2]) {
      printf("Yes");
      return 0;
    }
    else {
      printf("No");
      return 0;
    }
  }
  else {
    printf("No");
    return 0;
  }
}