#include <stdio.h>
#include <string.h>
int main () {
  char s[100001];
  scanf ("%s",s);
  int x;
  x = strlen(s);
  int n;
  int y = x;  //y = x/2
  y = (x+(x%2))/2;
  for (n=0;n<y;n++) {
    if (s[n] == 'b' && s[x-n-1] != 'd' || s[n] == 'd' && s[x-n-1] != 'b' || s[n] == 'p' && s[x-n-1] != 'q' || s[n] == 'q' && s[x-n-1] != 'p') {
      printf ("No\n");
      break;
    }
  }
  if (n == y) printf ("Yes\n");
  
  return 0;
}
