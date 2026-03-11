#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
  int y, m, d;
  char s[20];
  char *g;
  
  while (1) {
    fgets(s, sizeof(s), stdin);
    if (s[0] == '#') break;
    s[strlen(s) - 1] = '\0';
    g = strtok(s, " ");
    
    g = strtok(NULL, " ");
    y = atoi(g);
    g = strtok(NULL, " ");
    m = atoi(g);
    g = strtok(NULL, " ");
    d = atoi(g);

    if (y <= 30) {
      printf("HEISEI %d %d %d\n", y, m, d);
    } else if (y >=32) {
      printf("? %d %d %d\n", y - 30, m, d);
    } else {
      if (m >= 5) {
        printf("? 1 %d %d\n", m, d);
      } else {
        printf("HEISEI %d %d %d\n", y, m, d);
      }
    }
  }

  return 0;
}

