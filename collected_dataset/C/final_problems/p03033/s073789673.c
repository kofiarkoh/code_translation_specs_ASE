#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define df 0
#define MAX 200010
int n,q;

typedef struct{
  int num;
  int S;
  int X;
  int flag;
} data;


data work[MAX*2];
int comp(const void* a,const void* b){
  data p=*((data*)a), q=*((data*)b);
  return p.S-q.S;
}

typedef struct{
  int num;
  int X;
}cell;
int eq(cell a,cell b){
  return (a.num==b.num && a.X==b.X);
}

cell Q0[MAX+1];  
cell Q1[MAX+1];
int len0=0,len1=0;
#define PARENT(i) ((i)>>1)
#define RIGHT(i) ((i)<<1+1)
#define LEFT(i) ((i)<<1)
#define NULL_CELL (cell){-1,-1}

int compare(cell a,cell b){
  if(a.X!=b.X) return a.X<b.X;
  return a.num<b.num;
}

void enque(cell* Q,int* len,cell a);
void del(cell* Q,int* len);
void solve();
void print_que(cell* Q,int len);

int k=0;


int main(){
  scanf("%d%d",&n,&q);
  int i;
  for(i=0;i<n;i++){
    int s,t,x;
    scanf("%d%d%d",&s,&t,&x);
    work[i].num=i;
    work[i].S=s-x;
    work[i].X=x;
    work[i].flag=1;
    work[i+n].num=i;
    work[i+n].S=t-x;
    work[i+n].X=x;
    work[i+n].flag=0;
  }
  qsort(work,n*2,sizeof(data),comp);
  if(df){
    printf("i S X f\n");
    for(i=0;i<2*n;i++)
      printf("%d %2d %2d %d\n",work[i].num,work[i].S,work[i].X,work[i].flag);
  }
  for(i=0;i<q;i++){
    solve();
  }
  return 0;
}
void enque(cell* Q,int* len,cell a){
  (*len)++;
  int i=*len;
  while(i>1 && compare(a,Q[PARENT(i)])){
    Q[i]=Q[PARENT(i)];
    i=PARENT(i);
  }
  Q[i]=a;
}
void del(cell* Q,int* len){
  if(len==0)return; 
  cell a=Q[(*len)--];
  int i=1;
  while(LEFT(i)<=*len){
    int min=LEFT(i);
    if(RIGHT(i)<=*len) min=compare(Q[min],Q[RIGHT(i)])?min:RIGHT(i);
    if(compare(a,Q[min])) break;
    Q[i]=Q[min];
    i=min;
  }
  Q[i]=a;
}

void solve(){
  int d;
  scanf("%d",&d);
  while(k<2*n && work[k].S<=d){
    data a=work[k];
    if(a.flag){
      enque(Q1,&len1,(cell){a.num,a.X});
    }else enque(Q0,&len0,(cell){a.num,a.X});
    if(df){
    }
    k++;
  }
  if(df)print_que(Q0,len0);
  if(df)print_que(Q1,len1);
  cell c1=Q1[1],c0=Q0[1];
  while(len1!=0 && eq(c1,c0)){
    del(Q1,&len1);
    del(Q0,&len0);
    c1=Q1[1]; c0=Q0[1];
  }
  if(df)printf("\n");
  if(df)print_que(Q0,len0);
  if(df)print_que(Q1,len1);
  if(len1==0){
    printf("-1\n");
    return;
  }
  if(df)printf("[");
  printf("%d",c1.X);
  if(df)printf("]");
  printf("\n");
}

void print_que(cell* Q,int len){
  int i;
  printf("Q:[");
  for(i=1;i<= len;i++){
    printf("%d(%d)",Q[i].X,Q[i].num);
  }
  printf("]\n");
}

/// confirm df==0 ///
