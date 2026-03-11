/*
  AOJ #3017
  title:Piano
  @kankichi573
 */
#include <stdio.h>

#define max(x,y) (((x)>(y))?(x):(y))

int n,p,c,dir,pdir,ans,runlen;

int main()
{
  int i;

  scanf("%d",&n);
  
  
  scanf("%d",&p);
  ans=1;

  pdir=0;runlen=1;
  for(i=1;i<n;i++)
    {
      scanf("%d",&c);
      if(i!=0)
	{
	  if(c>p)
	    dir=1;
	  else if(c<p)
	    dir=-1;
	  else
	    continue;
	   
	  if(dir==pdir)
	    runlen++;
	  else
	    runlen=2;
      
	  ans=max(ans,runlen);

      //printf("[%c]prev=%d,current=%d,runlen=%d\n",(dir==1)?'+':'-',p,c,runlen);
	}
      p=c;
      pdir=dir;
    }
  printf("%d\n",ans) ;
  return(0);
}

