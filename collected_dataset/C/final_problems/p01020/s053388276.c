/*
A1530.c
*/
#include <stdio.h>
#include <stdlib.h>
#define min(x,y) (((x)<(y))?(x):(y))

int t[1000];

int comp( const void * a , const void * b ) {

  if( *( int * )a < *( int * )b ) {
    return -1;
  }
  else
  if( *( int * )a == *( int * )b ) {
    return 0;
  }
  return 1;
}

int solve(int t[],int n)
{ int a,b;
  
  if(n==1)
    return(t[0]);
  else if(n==2)
    return(t[1]); 
  else if(n==3)
    return(t[0]+t[1]+t[2]);
  else
    {
      a=2*t[0]+t[n-1]+t[n-2];
      b=t[0]+2*t[1]+t[n-1];

      return(min(a,b)+solve(t,n-2)) ;
    }
}

int main()
{
  int i,n,ret;
  scanf("%d",&n);

  for(i=0;i<n;i++)
  {
    scanf("%d",&t[i]);
  }
  qsort(t,n,sizeof(int),comp);
  ret=solve(t,n);
  printf("%d\n",ret);
  return(0);
}

