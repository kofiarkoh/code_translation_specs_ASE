#include <stdio.h>
#include <string.h>
#define min(x, y) ((x)>(y) ? (y) : (x))

int main(){
  int n;  scanf("%d", &n);
  int table[55][30] = {0};

  for(int i=1; i<=n; i++){
    char s[55];  scanf("%s", s);
    int len = strlen(s);
    for(int j=0; j<len; j++)
      table[i][(int)s[j]-(int)'a']++;
  }

  int ans[30] = {0};
  for(int i=0; i<=(int)'z'-(int)'a'; i++){
    int mini = 555;
    for(int j=1; j<=n; j++)
      mini = min(mini, table[j][i]);
    ans[i] = mini;
  }
  
  for(int i=0; i<=(int)'z'-(int)'a'; i++)
    for(int j=0; j<ans[i]; j++)
      printf("%c", (char)(i+(int)'a'));
  puts("");

  return 0;
}
