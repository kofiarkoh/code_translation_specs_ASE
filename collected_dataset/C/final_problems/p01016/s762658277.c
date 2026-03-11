#include <stdio.h>
#include <string.h>

int main(void)
{
  char a[1111],b[1111];
  int al, bl, i, j;
  scanf("%s%s",a,b);
  al = strlen(a);
  bl = strlen(b);
  for( i = 0; i < al; i++ ) {
    int ok = 1;
    for( j = 0; j < bl; j++ ) {
      if( a[i+j] != b[j] && b[j] != '_' ) ok = 0;
    }
    if( ok ) {
      puts("Yes");
      return 0;
    }
  }
  puts("No");
  return 0;
}
