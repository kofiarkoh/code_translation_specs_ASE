#include <stdio.h>
#include <string.h>
#define N 1000000
#define M 10000

int main()
{
  char c[N],d[M];
  int i=0,Ccount=0,Dcount=0,t=0,A[N];

  scanf("%s",c);
  Ccount=strlen(c);
  scanf("%s",d);
  Dcount=strlen(d);
  //printf("%d %d\n",Ccount,Dcount);                                            
  for(i=0;i<Ccount;i++)
    {
      if(strncmp(c+i,d,Dcount)==0)  //Dcount??????????????????                        
                                    //????????????????????????????????????                  
        {
          A[t]=i;
          t++;
        }
    }
  for(i=0;i<t;i++)
    {
      printf("%d\n",A[i]);
    }
  return 0;
}