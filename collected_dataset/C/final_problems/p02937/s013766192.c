#include <stdio.h>
#include <stdlib.h>


int main(void) {
  long long s_index, t_index, found_flag, cnt;
  char s[100001];
  char t[100001];


  scanf("%s", s);
  scanf("%s", t);


  t_index = 0;
  s_index = 0;
  found_flag = 0;
  cnt = 0;
  while(t[t_index] != '\0') {
    while(s[s_index] != t[t_index]) {
      if (s[s_index] == '\0') {
        if (found_flag == 0) {
          printf("-1");
          return 0;
        } else {
          found_flag = 0;
          s_index = 0;
        }
      } else {
        s_index++;
        cnt++;
      }
    }
    found_flag = 1;
    s_index++;
    cnt++;
    t_index++;
  }

  printf("%lld", cnt);
  return 0;
}