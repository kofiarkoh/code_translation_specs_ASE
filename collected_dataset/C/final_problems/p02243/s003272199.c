#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define MAX 10000
#define G 819210
#define N 100000
#define INF 777333555
 
typedef struct{
  int a,b,c;
}data;
 
int n,A[N];
data I[G];
 
void cl(int u)
{
  int i,z=1;
 
  for(i=0; i<n; i++)A[i]=INF;
  
  A[0]=0;
 
  while(z!=0){
    
    z=0;
    
    for(i=0; i<u; i++){
      
      if(I[i].c+A[I[i].b]<A[I[i].a]){
	
    A[I[i].a]=I[i].c+A[I[i].b];
    
    z=1;
    
      }
    }
  }
}
 
int main()
{
  int i,j,x,y,dt=0;

  
  scanf("%d",&n);
  
  for(i=0; i<n; i++){
    
    scanf("%d%d",&y,&x);
    
    for(j=0; j<x; j++){
      
      I[dt].b=y;
      
      scanf("%d%d",&I[dt].a,&I[dt].c);
      
      dt++;
    }
  }
  
  cl(dt);
  
  for(i=0; i<n; i++)printf("%d %d\n",i,A[i]);
  
  return 0;
}

