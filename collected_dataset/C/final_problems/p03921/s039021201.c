#include <stdio.h>
#include <stdlib.h>

typedef struct ll{
  int data[100000];
  int num;
  struct ll *next;
} DL;

DL *createList(int *data, int n){
  DL *new;
  new = (DL *)malloc(sizeof(DL));
  int i;
  for(i=0; i<n; i++){
    new->data[i] = data[i];
  }
  new->num = n;

  return new;
}

void addList(DL **head, DL *add){
  add->next = *head;
  *head = add;
  return;
}

void deleteList(DL **pt){
  DL *del = *pt;
  *pt = del->next;

  free(del);
  return;
}

void printList(DL *head){
  int i, n, *data;
  puts("LIST---------------");
  while(head != NULL){
    n = head->num;
    data = head->data;
    for(i=0; i<n; i++) printf("%d ", data[i]);
    putchar('\n');
    head = head->next;
  }
  puts("---------------LIST");
  return;
}

int main()
{
  int M, N;
  int i, j, n, index, flg, find;
  int *ss;
  int data[10000]={0};
  int node[10000];

  DL *head=NULL;
  DL **search = NULL;
  scanf("%d%d", &N, &M);

  scanf("%d", &n);
  for(i=0; i<n; i++){
    scanf("%d", &index);
    data[index]++;
  }

  for(i=1; i<N; i++){
    scanf("%d", &n);
    find=0;
    for(j=0; j<n; j++){
      scanf("%d", node+j);
      if(data[node[j]]) find++;
    }
    if(find){
      for(j=0; j<n; j++){
        data[node[j]]=1;
      }
    }else{
      DL *add = createList(node, n);
      addList(&head, add);
    }
  }

  flg = 1;
  while(flg && head != NULL){
    search = &head;
    flg = 0;
    while(*search != NULL){
      n = (*search)->num;
      ss = (*search)->data;
      find=0;
      for(i=0; i<n; i++){
        if(data[ss[i]]) find++;
      }
      if(find){
        for(i=0; i<n; i++){
          data[ss[i]]=1;
        }
        deleteList(search);
        flg++;
      }else search = &(*search)->next;
    }
  }
  printf("%s\n", head==NULL?"YES":"NO");
  return(0);
}
