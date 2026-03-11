#include <stdio.h>
#include <stdlib.h>

/* ?????\??? */
typedef struct queue{
  int front, rear, size;  
  int start, end;         /* ????????¨???????????\??????????´? */
  int *data;
}Queue;


Queue *murge(Queue *u, Queue *v);
Queue *createQ(int n);
void inQ(Queue *a, int x);
int deQ(Queue *a);


int main(void){
  int i, j, n, x, setA = 0, setB, min, ans = 0;
  Queue **set;

  scanf("%d", &n);
  set = (Queue **)malloc(sizeof(Queue *) * 2 * n);
  if(set == NULL){
    printf("?????¢????¢??????¨??????\n");
    return -1;
  }
  for(i = 0; i < n; i++){
    scanf("%d", &x);
    set[i] = createQ(1);
    inQ(set[i], x);
  }

  setB = n;
  for(i = 0; i < n; i++){
    min = setA;
    for(j = setA + 1; j < setB; j++){
      if(set[j]->start < set[min]->start){
	min = j;
      }
    }
    if(setA == min && set[min]->size == 1){
      setA++;
      free(set[min]->data);
      free(set[min]);
      continue;
    }
    deQ(set[min]);
    (set[min]->size)--;
    for(j = setA; j < min; j++){
      set[j+1] = murge(set[j+1], set[j]);
    }
    setA = min + 1;
    ans += set[min]->size;
    set[setB++] = set[min];
  }
  printf("%d\n", ans);
  return 0;
}


Queue *murge(Queue *u, Queue *v){
  Queue *temp, *p;
  int i = 0, j = 0;

  if(u->size < v->size){
    temp = u;
    u = v;
    v = temp;
  }
  p = createQ(u->size + v->size);
  while(i < u->size || j < v->size){
    if(j >= v->size || (i < u->size && u->start < v->start)){
      inQ(p, deQ(u));
      i++;
    }else{
      inQ(p, deQ(v));
      j++;
    }
  }
  free(u->data);
  free(v->data);
  free(u);
  free(v);
  return p;
}


Queue *createQ(int n){
  Queue *p;

  p = (Queue *)malloc(sizeof(Queue));
  p->data = (int *)malloc(sizeof(int) * n);
  if(p != NULL){
    p->front = 0;
    p->rear = 0;
    p->size = n;
  }
  return p;
}


void inQ(Queue *a, int x){
  if(a->rear == a->front){
    a->start = x;
  }
  a->end = x;
  a->data[(a->rear)++] = x;
}

int deQ(Queue *a){
  int x;

  x = a->data[(a->front)++];
  if(a->front < a->rear){
    a->start = a->data[a->front];
  }
  return x;
}