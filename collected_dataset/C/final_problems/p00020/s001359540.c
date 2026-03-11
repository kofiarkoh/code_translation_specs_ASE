#include<stdio.h>
#define SIZE 200
int main()
{
  char c[SIZE];
  int i,sl;
  for(i=0;;i++)
    {
      scanf("%c",&c[i]);
      if(c[i]=='\n')
	{
	  sl=i;
	  break;
	}
      else if((c[i]>='a')&&(c[i]<='z')) c[i]-='a'-'A';
    }
  for(i=0;i<sl;i++) printf("%c",c[i]);
  printf("\n");
  return 0;
}