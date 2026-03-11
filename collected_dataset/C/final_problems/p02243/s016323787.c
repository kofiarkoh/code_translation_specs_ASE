//2020.1.27
//s1270188 xxxmk2
//alds1_12c

#include<stdio.h>
#include<stdlib.h>
#include<limits.h>
#define N 100000
#define WHITE 0
#define GRAY 1
#define BLACK 2

//struct
struct node{
  int id;
  int d;
  struct node *next;
};

//global
int num;
struct node data[N];

//function
void insert(int a,int id,int d){
  struct node*dd, *n;

  dd=malloc(sizeof(struct node));
  dd->id=id;
  dd->d=d;
  dd->next=NULL;

  for(n=&data[a];n->next!=NULL;n=n->next);
  n->next=dd;
}

void cnt(void){
  int color[N], d[N];
  int min, i, u, ddd;
  struct node *n;
  
  //reset
  for(i=0;i<num;i++){
    color[i]=WHITE;
    d[i]=INT_MAX;
  }
  d[0]=0;

  
  while(1){
    min=INT_MAX;
    
    for(i=0;i<num;i++){
      if(color[i]!=BLACK && d[i]<min){
	min=d[i];
	u=i;
      }
    }
    
    if(min==INT_MAX) break;

    color[u]=BLACK;

    for(n=&data[u];n!=NULL;n=n->next){
      if(color[n->id]!=BLACK && d[u]+n->d<d[n->id]){
	d[n->id]=d[u]+n->d;
	color[n->id]=GRAY;
      }
      
    }
    
  }

  for(i=0;i<num;i++) printf("%d %d\n",i,d[i]);  

}



int main(){
  int i, j, a, b, c, d;
  
  //input
  scanf("%d",&num);
  for(i=0;i<num;i++) data[i].next=NULL; //reset
    
  for(i=0;i<num;i++){
    scanf("%d %d",&a,&b);
    for(j=0;j<b;j++) {
      scanf("%d %d",&c,&d);
      insert(a,c,d);
    }
  }
  
  /* input test
  struct node *n;
  for(i=0;i<num;i++){
    printf("%d: ",i);
    for(n=data[i].next;n!=NULL;n=n->next) printf("%2d ",n->id);
    printf("\n   ");
    for(n=data[i].next;n!=NULL;n=n->next) printf("%2d ",n->d);
    printf("\n\n");
  }
  */
  
  //output
  cnt();
  
  return 0;
}

