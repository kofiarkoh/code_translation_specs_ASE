#include<stdio.h>
#include <string.h>
int main(void)
{
  char s[1001];
  char S[1001];
  int w,j;
  int len;
  j=0;
 int a=1;
  scanf("%s",s);
  scanf("%d",&w);
  len=strlen(s);

    for(int i=0;i<1000;i++)
    {
      if(i%w==0)
      {

        S[j]=s[i];
        j++;
      }
      if(i>=len){
        i=1000;
      }

    }


  printf("%s",S);
  return 0;
}
