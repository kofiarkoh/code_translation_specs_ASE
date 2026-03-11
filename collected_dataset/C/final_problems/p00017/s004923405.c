#include <stdio.h>
#include <string.h>

char *caesar(char str[], char caesar_str[], int shift) {
  int i;

  for (i = 0; str[i] != '\0'; i++) {
    if (str[i] == ' ')
      caesar_str[i] = ' ';
    else if (str[i] == '.')
      caesar_str[i] = '.';
    else if (str[i] + shift <= 122)
      caesar_str[i] = str[i] + shift;
    else
      caesar_str[i] = str[i] + shift - 26;
  }
  caesar_str[i] = '\0';

  return caesar_str;
}

int main(void) {
  int i;
  char str[81], caesar_str[81];

  fgets(str, sizeof(str), stdin);

  for (i = 0; i < 26; i++) {
    if (strstr(caesar(str, caesar_str, i), "the"))
      printf("%s\n", caesar_str);
    else if (strstr(caesar(str, caesar_str, i), "this"))
      printf("%s\n", caesar_str);
    else if (strstr(caesar(str, caesar_str, i), "that"))
      printf("%s\n", caesar_str);
  }

  return 0;
}