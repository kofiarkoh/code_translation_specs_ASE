#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define BUF_SIZE 5000000
#define MEMBER 3

int comp(const void* a, const void* b) {
  return *(int*)a - *(int*)b;
}
int main(void) {
  FILE *fp = stdin;
  char line[BUF_SIZE];
  if(!fgets(line, BUF_SIZE, fp)) return 0;
  int groups;
  sscanf(line, "%d", &groups);

  int i;
  int members = groups*MEMBER;
  int *arr = malloc(sizeof(int)*members);
  if(!fgets(line, BUF_SIZE, fp)) return 0;
  char *tmpbuf = line;
  for(i = 0; i < members; i++) {
    char *tmp = strtok(tmpbuf, " ");
    arr[i] = strtol(tmp, NULL, 10);
    tmpbuf = NULL;
  }
  qsort(arr, members, sizeof(int), comp);
  int trashes = 0;
  long sum = 0;
  for(i = members-1; i >= trashes; i -= 2) {
    sum += arr[i-1];
    trashes += 1;
  }
  printf("%ld\n", sum);
  return 0;
}