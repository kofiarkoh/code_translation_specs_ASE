#include <stdio.h>
#include <stdlib.h>

int judge(int i, int n, char *s, char *t, char animal) {
  if (i == n) {
    if ((t[i - 1] == 'S' && s[i - 1] == 'o') || (t[i - 1] == 'W' && s[i - 1] == 'x')) {
      if (t[i - 2] != t[0])
	return -1;
    } else {
      if (t[i - 2] == t[0])
	return -1;
    }
    if ((t[0] == 'S' && s[0] == 'o') || (t[0] == 'W' && s[0] == 'x')) {
      if (t[i - 1] != t[1])
	return -1;
    } else {
      if (t[i - 1] == t[1])
	return -1;
    }
    return 0;
  }
  
  t[i] = animal;
  
  if ((t[i] == 'S' && s[i] == 'o') || (t[i] =='W' && s[i] == 'x'))
    return judge(i + 1, n, s, t, t[i - 1]);
  else {
    if (t[i - 1] == 'S')
      return judge(i + 1, n, s, t, 'W');
    else
      return judge(i + 1, n, s, t, 'S');
  }
}

int main() {
  long int n;
  char *s, *t;
  
  scanf("%ld\n", &n);
  s = (char *)malloc(sizeof(char) * (n + 1));
  t = (char *)malloc(sizeof(char) * (n + 1));
  scanf("%s", s);
  t[n] = '\0';
  t[0] = 'S';
  if (judge(1, n, s, t, 'S') == 0)
    printf("%s\n", t);
  else if (judge(1, n, s, t, 'W') == 0)
    printf("%s\n", t);
  else {
    t[0] = 'W';
    if (judge(1, n, s, t, 'S') == 0)
      printf("%s\n", t);
    else if (judge(1, n, s, t, 'W') == 0)
      printf("%s\n", t);
    else
      printf("-1\n");
  }
  free(s);
  free(t);
  return 0;
}