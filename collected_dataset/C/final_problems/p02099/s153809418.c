/*
A3001.c
*/

#include <stdio.h>
#include <stdlib.h>
int n;
struct jk_st
{
  double gpa;
  int point;
  int order;
} data[100000];
  
int compA( const void * a , const void * b ) {

  if( ((struct jk_st * )a)->gpa > ((struct jk_st * )b)->gpa)
    return -1;

  else
    if( ((struct jk_st * )a)->gpa == ((struct jk_st * )b)->gpa)
    return 0;
  
  return 1;
}
int compB( const void * a , const void * b ) {

  if( ((struct jk_st * )a)->order < ((struct jk_st * )b)->order)
    return -1;

  else
    if( ((struct jk_st * )a)->order == ((struct jk_st * )b)->order)
    return 0;
  
  return 1;
}


void solve()
{
  int i,j,acc,same,pt;
  double prev;
  prev=data[0].gpa;same=1;
  acc=1;
  for(i=1,acc=0;i<=n;i++)
    {
      if(i==n || data[i].gpa!=prev)
      {
	acc += same;
	pt =(n-acc)*3+(same-1);

	for(j=1;j<=same;j++)
	  data[i-j].point=pt;
	same=1;
      }
      else
      	same++;
      

      prev=data[i].gpa;
      //printf("i=%d same=%d\n",i,same);
    }
}

void dump()
{ int i;
  for(i=n-1;i>=n-30;i--)
    printf("%d:%lf[%d][%d]\n",i,data[i].gpa,data[i].point,data[i].order);
}
void output()
{ int i;
  for(i=0;i<n;i++)
    printf("%d\n",data[i].point);
}

int main()
{
  int i;
  scanf("%d",&n);
  for(i=0;i<n;i++)
    {
      scanf("%lf",&(data[i].gpa));
      data[i].order=i;
    }
  qsort(data,n,sizeof(struct jk_st),compA);
  
  solve();
  //dump();  
  qsort(data,n,sizeof(struct jk_st),compB);

  output();
  return(0);
}

