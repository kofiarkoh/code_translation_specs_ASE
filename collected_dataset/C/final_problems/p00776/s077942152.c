#include <stdio.h>
#include <string.h>

#define MAX_CIPHER_LEN 20
#define append(str, pos, c) ((str)[(pos)] = (c), (str)[(pos)+1] = '\0')
#define ring(x)  ((x) >= 10 ? ((x) - 5) % 5 + 5 : (x))
#define initResult() (resultPos = nResult = 0)

char result[10][MAX_CIPHER_LEN + 1];
int resultPos = 0;
int nResult = 0;

void addResult(const char candidate[])
{
  nResult++;
  strcpy(result[resultPos], candidate);
  resultPos = ring(resultPos + 1);
}

void showResult()
{
  int i;
  printf("%d\n", nResult);
  if (nResult <= 10) {
    for (i = 0; i < nResult; i++) {
      printf("%s\n", result[i]);
    }
  } else {
    for (i = 0; i < 5; i++) {
      printf("%s\n", result[i]);  /* first five */
    }
    for (i = 0; i < 5; i++) {
      printf("%s\n", result[ring(i + resultPos)]);  /* last five */
    }
  }
}

/**
 * ?§£?????¢?´¢??????.
 * @param cipher ?????????
 * @param pos    ?????????????§£???????????????
 * @oaram candidate ?????¨?§???????????§£????£?
 * @return ?§£????????°
 */
void search(const char cipher[], int pos, char candidate[])
{
  char c = cipher[pos];
  if (c == '\0') { /* ?§£????????? */
    addResult(candidate);
    return;
  }

  if (c == 'a' || strchr(candidate, c) != NULL) {
    append(candidate, pos, c);
    search(cipher, pos + 1, candidate);
    candidate[pos] = '\0';
  }
  if (c != 'z' && strchr(candidate, c + 1) == NULL) {
    append(candidate, pos, c + 1);
    search(cipher, pos + 1, candidate);
    candidate[pos] = '\0';
  }
}

int main()
{
  static char cipher   [MAX_CIPHER_LEN + 1];
  static char candidate[MAX_CIPHER_LEN + 1];
  for (;;) {
    scanf("%s", cipher);
    if (strcmp(cipher, "#") == 0) break;

    candidate[0] = '\0';
    initResult();
    search(cipher, 0, candidate); /* ?§£?????¢?´¢ */
    showResult();
  }
  return 0;
}