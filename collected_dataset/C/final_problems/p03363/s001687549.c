#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define BUF_SIZE 2500000
#define HASH_SIZE 1024

struct list {
  struct list *next;
  long num;
  int cnt;
};
// create hash
struct list *create_hashtable(int size) {
  struct list *htable = malloc(size*sizeof(struct list));
  int i = 0;
  for(i = 0; i < size; i++) {
    htable[i].next = &htable[i];
  }
  return htable;
}

long get_key(long num) {
  return num & (HASH_SIZE-1);
}

void insert(struct list *htable, long num) {
  int key = get_key(num);
  struct list *head = &htable[key];
  struct list *elem;
  for(elem = head->next; elem != head; elem = elem->next) {
    if(elem->num == num) {
      elem->cnt++;
      return;
    }
  }

  // create
  elem = malloc(sizeof(struct list));
  elem->next = head->next;
  elem->num = num;
  elem->cnt = 1;
  head->next = elem;
  return;
}


int main(void) {
  FILE *fp = stdin;
  char line[BUF_SIZE];
  int num;
  long cum[200001] = {0};
  struct list *htable = create_hashtable(HASH_SIZE);
  if(!fgets(line, BUF_SIZE, fp)) return 0;
  sscanf(line, "%d", &num);

  if(!fgets(line, BUF_SIZE, fp)) return 0;
  char *tmpbuf = line;
  int i, j;
  insert(htable, 0); // cum[0] = 0;
  for(i = 0; i < num; i++) {
    char *tmp = strtok(tmpbuf, " ");
    long num = strtol(tmp, NULL, 10);
    tmpbuf = NULL;
    cum[i+1] = cum[i] + num;
    insert(htable, cum[i+1]);
  }

  long ans = 0;
  for(i = 0; i < HASH_SIZE; i++) {
    struct list *tmp;
    struct list *head = &htable[i];
    for(tmp = head->next; tmp != head; tmp = tmp->next) {
      int cnt = tmp->cnt;
      ans += (cnt-1)*cnt/2; // nP2
    }
  }
  printf("%ld\n", ans);

}