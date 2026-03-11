#include<stdio.h>
#include<stdlib.h>
typedef struct node
{
  int id;
  struct node *next;
} *nodePointer;
nodePointer *queueHead;
nodePointer *queueTail;
void enqueue(int,int);
void front(int);
void dequeue(int);
void init(int);
nodePointer make1node(int);
int main(void)
{
  int i;
  int n,q;
  int x1,x2,x3;
  scanf("%d",&n);
  init(n);
  scanf("%d",&q);
  for(i=0;i<q;i++)
    {
      scanf("%d",&x1);
      switch(x1)
        {
        case 0:
          scanf("%d%d",&x2,&x3);
          enqueue(x2,x3);
          break;
        case 1:
          scanf("%d",&x2);
          front(x2);
          break;
        case 2:
          scanf("%d",&x2);
          dequeue(x2);
          break;
        }
    }
  return 0;
}
void enqueue(int t,int x)
{
  nodePointer p=make1node(x);
  if(queueHead[t]==NULL)
    {
      queueHead[t]=p;
      queueTail[t]=p;
    }
  else
    {
      queueTail[t]->next=p;
      queueTail[t]=p;
    }
}
void front(int t)
{
  if(queueHead[t]!=NULL) printf("%d\n",queueHead[t]->id);
}
void dequeue(int t)
{
  if(queueHead[t]!=NULL)
    {
      queueHead[t]=queueHead[t]->next;
      if(queueHead[t]==NULL) queueTail[t]=NULL;
    }
}
void init(int n)
{
  int i;
  queueHead=(nodePointer*)malloc(sizeof(nodePointer)*n);
  for(i=0;i<n;i++) queueHead[i]=NULL;
  queueTail=(nodePointer*)malloc(sizeof(nodePointer)*n);
  for(i=0;i<n;i++) queueTail[i]=NULL;
}
nodePointer make1node(int x)
{
  nodePointer p=(nodePointer)malloc(sizeof(struct node));
  p->id=x;
  p->next=NULL;
  return p;
}
