#include <stdio.h>
#include <stdlib.h>
#define LEN 100005

typedef struct pp{
  char name[100];
  int t;
}P;

P Q[LEN+1];
int head, tail, n, q;
int j=1;

void enqueue(P );
void dequeue(void);

int main(){
  int elaps = 0, c;
  int i;
  P u;
  scanf("%d %d", &n, &q);

  for ( i = 1; i <= n; i++){
    scanf("%s", Q[i].name);
    scanf("%d", &Q[i].t);
  }

  head = 1;
  tail = 0;
  while(1){
    if(head == tail) break;
    if(Q[head].t <= q){
      elaps+=Q[head].t;
      printf("%s %d\n",Q[head].name, elaps);
      dequeue();
    }
    else{
      elaps+=q;
      enqueue( Q[head] );
      dequeue();
    }
  }
  return 0;
}

void enqueue(P x){
  Q[tail] = x;
  Q[tail].t = x.t - q;
  tail++;
  if(tail > n) tail = 0;
}

void dequeue(void){
  head++;
  if(head > n) head = 0;
}

