#include<stdio.h>
int main(void)
{
  int m, plan, set, money, year, part, total, top, answer[500]={0}; 
  double r;
  scanf("%d", &m);
  int i, j, k;

  for(i=0; i<m; i++)
    {
      scanf(" %d %d %d", &money ,&year, &part);
      for(j=0; j<part; j++)
	{
/*	  scanf(" %d %f %d", &plan, &r, &set);*/
	  scanf(" %d %lf %d", &plan, &r, &set);
	  total=money;
	  if(plan==0)  //??????
	    {
	      top=0;
	      for(k=0; k<year; k++)
		{
		  top+=total*r;
		  total-=set;
		}
	      total+=top; 
	      if(answer[i]<=total)
		{
		  answer[i]=total; //?????§???   
		}   
	    }
	  else if(plan==1)//??????
	    {
	      for(k=0; k<year; k++)
		{
		  total+=total*r-set;	  
		}
	      if(answer[i]<=total)
		{
		  answer[i]=total; //?????§???   
		}
	    }
	  else
	    {
	      break;
	    }
	}
    }
  for(i=0; i<m; i++) printf("%d\n", answer[i]);

  return 0;
}