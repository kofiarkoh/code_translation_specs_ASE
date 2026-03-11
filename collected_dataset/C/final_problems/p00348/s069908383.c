#include <stdio.h>
#include <stdlib.h>

typedef struct data{
  int num, dat;
  struct data *next;
}Data;

Data *insert(int x);
int sort(Data *a, int n);

int main(void){
  int i, n, x, ans;
  Data *start, *temp;

  scanf("%d", &n);
  start = (Data *)malloc(sizeof(Data));
  if(start == NULL){
    printf("?????¢????¢??????¨??????\n");
    return -1;
  }
  start->num = 0;
  temp = start;
  for(i = 0; i < n; i++){
    scanf("%d", &x);
    temp->next = insert(x);
    temp = temp->next;
    if(temp == NULL){
      printf("?????¢????¢??????¨??????\n");
      return -1;
    }
  }
  temp->next = NULL;

  ans = sort(start, n);
  printf("%d\n", ans);
  free(start);
  return 0;
}


Data *insert(int x){
  Data *p;

  p = (Data *)malloc(sizeof(Data));
  if(p != NULL){
    p->dat = x;
    p->num = 0;
  }
  return p;
}


int sort(Data *start, int n){
  int i, j, count = 0;
  Data *small, *end, *temp, *k, *x;

  for(i = 0; i < n; i++){
    x = NULL;
    for(temp = start->next; temp != NULL; temp = temp->next){
      if(x == NULL){
	if(temp->num == 0){
	  x = temp;
	}
	continue;
      }
      if(temp->num == 0 && x->dat > temp->dat){
	x = temp;
      }
    }
    x->num = i+1;
  }
  for(end = start; end->next != NULL; end = end->next);

  for(i = 0; i < n; i++){
    j = 0;
    for(x = start; x->next->num != i+1; x = x->next){
      j++;
    }
    temp = x->next;
    if(x == start){
      start->next = temp->next;
      free(temp);
      continue;
    }
    count += j;
    small = start;
    for(k = start; k->next != temp; k = k->next){
      if(small->next->num > k->next->num){
	small = k;
      }
    }
    if(small->next != x){
      x->next = small->next;
      x = small->next;
      small->next = small->next->next;
    }
    x->next = NULL;
    end->next = start->next;
    start->next = temp->next;
    end = x;
    free(temp);
  } 
  return count;
}