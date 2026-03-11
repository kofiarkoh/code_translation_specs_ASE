#include<stdio.h>
#define swap(a,b) a^b,b^a,a^b
typedef struct{int Parent,Rank;}UnionFind;
UnionFind uft[100000];
int Root(int x){return(uft[x].Parent!=x)?uft[x].Parent=Root(uft[x].Parent):x;}
int Unite(int x,int y)
{
  x=Root(x),y=Root(y);
  if(x!=y)
  {
    if(uft[x].Rank!=uft[y].Rank)swap(x,y);
    uft[y].Parent=x;
    uft[x].Rank+=uft[y].Rank;
  }
  return x!=y;
}
int Size(int x){return uft[Root(x)].Rank;}
int Belong(int x,int y){return Root(x)==Root(y);}
int main(void)
{
  int A,B,C,D,N,M,K,i,j,known[100000]={0};
  j=scanf("%d %d %d",&N,&M,&K);
  for(i=0;i<N;i++){uft[i].Parent=i;uft[i].Rank=1;}
  while(M--)
  {
    j=scanf("%d %d",&A,&B);
    A--,B--;known[A]++,known[B]++;Unite(A,B);
  }
  while(K--)
  {
    j=scanf("%d %d",&C,&D);
    C--,D--;if(Belong(C,D))known[C]++,known[D]++;
  }
  for(i=0;i<N;i++){printf("%d ",Size(i)-known[i]-1);}
  return printf("\n"),0;
}