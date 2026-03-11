#include <stdio.h>
#include <string.h>
#define LENGTH 1001

int main(void)
{
  char st[LENGTH];
  int i, j, k, l, m, check, fix, count, kosuu, decimal=1;
  int a, b, t=0;
  char ch;

  i = 0;
  while(scanf("%1c",&ch)!=EOF)
    {
      st[i] = ch;
      i++;
    }
  st[i+1] = 0;
  st[i+2] = 0;
  st[i+3] = 0;

  k = strlen(st);


  /*
  printf("k = %d\n",strlen(st));

  for(i=0;i<=strlen(st);i++)
    {
      printf("st[%d] %d\n",i,st[i]); 
      }
  */

  j = 1;
  count = 1;
  kosuu = 0;

  i = 0;
  t = 0;
  while(1)
    {
      if(st[i]=='@')
	{
	  kosuu = st[i+1]-48;
	  for(j=0;j<=kosuu-1;j++)
	    {
	      printf("%c",st[i+2]);
	    }
	  i = i + 3;
	}
      else if(st[i]==10)
	{
	  printf("\n");
	  i++;
	}
      else if(st[i]==0)
	{
	  break;
	}
      else
	{
	  printf("%c",st[i]);
	  i++;
	}
    }
  printf("\n");
  return 0;
}