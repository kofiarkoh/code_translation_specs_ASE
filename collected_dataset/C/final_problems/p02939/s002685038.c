#define _CRT_SECURE_NO_WARNINGS
#include<stdio.h>

int main() {
	int i,j = 0,ans = 1;
  int flag = 0;
  char S[200000];
	scanf("%s", S);
  for (i = 1; S[i] != '\0'; i++) {
    if (S[i - 1] == S[i]) {
      j++;
    }
    else if (j != 0) {
      ans -= (j + 2) / 3;
      if ((j + 3) % 3 == 1) {
        if (flag == 1) {
          ans++;
          flag = 0;
        }
        else {
          flag = 1;
        }
      }
      j = 0;
    }
    else {
      flag = 0;
    }
    ans++;
  }
  if (j != 0) {
    ans -= (j + 2) / 3;
    if ((j + 3) % 3 == 1) {
      if (flag == 1) {
        ans++;
      }
    }
  }
  printf("%d", ans);
	return 0;
}