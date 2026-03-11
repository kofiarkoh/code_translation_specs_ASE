#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <assert.h>

#define KILO 1000
#define MEGA (1000*1000)

#define MAX_T (1 * MEGA)
#define MAX_P (1 * KILO)
#define MAX_PATTERN_NUMBER (10 * KILO)

#define TYPE_NODE 1
#define TYPE_LIST 2
#define TYPE_EDGE 3

typedef struct node {
  char type;
  struct list *end;
  struct edge_list *edge;
} NODE;

const NODE INITIAL_NODE = { TYPE_NODE, NULL, NULL };

typedef struct list {
  char type;
  int index;
  struct list *next;
} LIST;

const LIST INITIAL_LIST = { TYPE_LIST, 0, NULL };

typedef struct edge_list {
  char type;
  char c;
  struct edge_list *next;
  struct node *node;
} EDGE;

const EDGE INITIAL_EDGE = { TYPE_EDGE, '\0', NULL, NULL };

typedef union unit {
  struct node node;
  struct list list;
  struct edge_list edge;
} UNIT;

const size_t MEM_MAX = 100;
//const size_t MEM_MAX = 80 * KILO;
UNIT *mem = NULL;
size_t cnt = 0;

NODE* get_initial_node() {
  assert(cnt < MEM_MAX);
  NODE *res = &(mem[cnt++].node);
  *res = INITIAL_NODE;
  return res;
}

LIST* get_initial_list() {
  assert(cnt < MEM_MAX);
  LIST *res = &(mem[cnt++].list);
  *res = INITIAL_LIST;
  return res;
}

EDGE* get_initial_edge() {
  assert(cnt < MEM_MAX);
  EDGE *res = &(mem[cnt++].edge);
  *res = INITIAL_EDGE;
  return res;
}

NODE *root = NULL;

NODE * make_or_get_next_node(NODE *cur, char c) {
  //printf("make_or_get_next_node(%lx, %c)\n", (size_t)cur, c);
  EDGE *next = cur->edge;
  if (next == NULL) {
    next = cur->edge = get_initial_edge();
    next->c = c;
    return (next->node = get_initial_node());
  }
  while (next->next != NULL && next->c != c) next = next->next;
  if (next->c != c) {
    next = next->next = get_initial_edge();
    next->c = c;
    return (next->node = get_initial_node());
  } else {
    return next->node;
  }
}

void add_index(NODE *node, int index) {
  if (node->end == NULL) {
    node->end = get_initial_list();
    node->end->index = index;
  } else {
    LIST *list = node->end;
    while (list->next != NULL) list = list->next;
    list = list->next = get_initial_list();
    list->index = index;
  }
}

void add(int index, char *str) {
  char c = '\0';
  NODE *node = root;
  while ((c = (*(str++))) != '\0')
    node = make_or_get_next_node(node, c);
  add_index(node, index);
}

void match(char *str, char *res) {
  NODE *node = root;
  LIST *list = NULL;
  char c = '\0';
  EDGE *edge = NULL;
  for (int i = 0; i < 10; ++i) {
  //while (1) {
    //printf("in match: node = %lx\n", (size_t)node);
    assert(node != NULL);
    if ((list = node->end) != NULL) {
      while (list != NULL) {
        res[list->index] = '1';
        list = list->next;
      }
    }
    if ((c = *(str++)) == '\0') return;
    //printf("in match: c = %c\n", c);
    edge = node->edge;
    if (edge == NULL) return;
    while (edge->c != c) {
      edge = edge->next;
      if (edge == NULL) return;
    }
    node = edge->node;
  }
}

void print_tree(NODE *node, int offset) {
  for (int i = 0; i < offset; ++i) printf(" ");
  printf("* ");
  for (LIST *i = node->end; i != NULL; i = i->next) printf("%d ", i->index);
  printf("\n");
  for (EDGE *i = node->edge; i != NULL; i = i->next) {
    for (int j = 0; j < offset; ++j) printf(" ");
    printf("%c\n", i->c);
    print_tree(i->node, offset + 2);
  }
}

void print_node(NODE *node) {
  assert(node->type == TYPE_NODE);
  printf("end-list = %lx, edge-list = %lx\n",
    (size_t)node->end, (size_t)node->edge);
}

int main() {
  assert(sizeof(UNIT) <= 24);
  mem = calloc(MEM_MAX, sizeof(UNIT));
  assert(mem);
  char *str = malloc((MAX_T+1) * sizeof(char));
  assert(str);
  char *pat = malloc((MAX_P+1) * sizeof(char));
  assert(pat);
  char *res = malloc(MAX_PATTERN_NUMBER * sizeof(char));
  assert(res);
  for (int i = 0; i < MAX_PATTERN_NUMBER; ++i) res[i] = '0';

  //printf("mem = %lx\n", (size_t)mem);
  
  scanf("%s", str);
  int q = 0;
  scanf("%d", &q);

  root = get_initial_node();
  for (int i = 0; i < q; ++i) {
    scanf("%s", pat);
    add(i, pat);
  }
  //print_tree(root, 0);
  for (char *s = str; s[0] != '\0'; ++s) match(s, res);
  
  for (int i = 0; i < q; ++i) printf("%c\n", res[i]);

  free(mem);
  free(str);
  free(pat);
  free(res);
  return 0;
}