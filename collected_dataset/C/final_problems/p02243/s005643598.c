#include<stdio.h>
#include<stdlib.h>
 
#define N 100000
#define INF 100000000
 
typedef struct{
  int g;
  int b;
  int c;
}data;
 
int n,A[N];
data D[500000];
 
void singlesource(int num)
{
  int i,ch=1;
 
  for(i=0; i<n; i++)A[i]=INF;
  A[0]=0;
 
  while(ch!=0){
    ch=0;
    for(i=0; i<num; i++){
      if(D[i].c+A[D[i].b]<A[D[i].g]){
    A[D[i].g]=D[i].c+A[D[i].b];
    ch=1;
      }
    }
  }
}
 
int main()
{
  int i,j,k,p,con=0;
 
  scanf("%d",&n);
  for(i=0; i<n; i++){
    scanf("%d%d",&p,&k);
    for(j=0; j<k; j++){
      D[con].b=p;
      scanf("%d%d",&D[con].g,&D[con].c);
      con++;
    }
  }
  singlesource(con);
  for(i=0; i<n; i++)printf("%d %d\n",i,A[i]);
  return 0;
}
