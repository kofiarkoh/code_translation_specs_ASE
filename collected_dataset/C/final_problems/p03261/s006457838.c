#pragma GCC diagnostic ignored "-Wunused-result"

#include <stdio.h>
#include <string.h>

#define MAX_LEN 10
#define MAX_WORDS 101 // 100 chars + 1 '\0'

static char W[MAX_WORDS][MAX_LEN];

// return 1 : found
// return 0 : not found
static inline int search(const char* searh_word, int num_word) {
  for (int i=0; i<num_word; i++) {
    if (strcmp(W[i],searh_word)==0) {
      // printf("found\n");
      return 1;
    }
  }
  // printf("not found\n");
  return 0;
}

int main(void) {
  int N; // 2<=N<=100
  char last_char = 0;
  int last;
  
  scanf("%d", &N);

  for (int i=0; i<N; i++) {
    char tmp_last_char;
    scanf("%s",W[i]);
    if (i==0) last_char = W[i][0];
    // printf("%s:%c\n",W[i],tmp_last_char);
    if ((W[i][0] != last_char) || (search(W[i], i)==1)) {
      printf("No\n");
      goto end;
    }
    last = strlen(W[i])-1;
    last_char = W[i][last];
  }
  printf("Yes\n");

 end:
  return 0;

}
