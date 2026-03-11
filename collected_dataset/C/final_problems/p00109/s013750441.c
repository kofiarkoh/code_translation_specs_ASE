#include <stdio.h>

int expr(char **s);

int number(char **s) {
  int val = 0;
  while ('0' <= **s && **s <= '9') {
    val = val * 10 + **s - '0';
    (*s)++;
  }
  return val;
}

int factor(char **s) {
  if (**s == '(') {
    (*s)++;
    int val = expr(s);
    (*s)++;
    return val;
  }
  return number(s);
}

int term(char **s) {
  int val = factor(s);
  while (1) {
    if (**s == '*') {
      (*s)++;
      val *= factor(s);
    } else if (**s == '/') {
      (*s)++;
      val /= factor(s);
    } else {
      break;
    }
  }
  return val;
}

int expr(char **s) {
  int val = term(s);
  while (1) {
    if (**s == '+') {
      (*s)++;
      val += term(s);
    } else if (**s == '-') {
      (*s)++;
      val -= term(s);
    } else {
      break;
    }
  }
  return val;
}

int main() {
  int n;
  scanf("%d", &n);
  char s[101];
  while (n--) {
    scanf("%s", s);
    char *p = s;
    printf("%d\n", expr(&p));
  }
}