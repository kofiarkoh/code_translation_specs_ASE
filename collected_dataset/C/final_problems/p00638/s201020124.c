/*
  AOJ 1052
  Title:
  @kankichi573
*/
#include <stdio.h>
#include <stdlib.h>

int N,M;
struct bridge_st
{
  int br;
  int tr;
};

struct bridge_st brg[25];

int compare_br(const void *a, const void *b)
{
  if( ((struct bridge_st *)a)->br > ((struct bridge_st *)b)->br)
    return(-1);
  if( ((struct bridge_st *)a)->br < ((struct bridge_st *)b)->br)
    return(1);
  if( ((struct bridge_st *)a)->tr > ((struct bridge_st *)b)->tr)
    return(-1);
  if( ((struct bridge_st *)a)->tr < ((struct bridge_st *)b)->tr)
    return(1);
  if( ((struct bridge_st *)a)->tr == ((struct bridge_st *)b)->tr)
    return(0);
}

int main()
{
  int i,ret,flag,sum;
  
  while(scanf("%d",&N) && N)
    {
      for(i=sum=0;i<N;i++)
	{
	  scanf("%d %d",&brg[i].tr,&brg[i].br);
	  sum += brg[i].tr;
	}

      qsort(brg,N,sizeof(struct bridge_st),compare_br);
      //printf("sum=%d\n",sum);
      
      flag=1;
      //for(i=0;i<N;i++)
      //printf("%d %d\n",brg[i].tr,brg[i].br);
      
      for(i=0;i<N;i++)
	{
	  if(brg[i].br < sum)
	    {
	      flag=0;
	      break;
	    }
	  sum -= brg[i].tr;
	}
      printf("%s\n",flag?"Yes":"No");
    }
  return(0);
}