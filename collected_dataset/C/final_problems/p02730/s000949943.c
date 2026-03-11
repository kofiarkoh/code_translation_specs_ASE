#include<stdio.h>
#include<string.h>
int main()
{
  char str[100];
  gets(str);
  int l=strlen(str);
  int count =0;
  int yes=0;
  for(int i=0;i<l/2;i++) // 0 1 2 3 4 5 6
  {
      if(str[i]!=str[l-1-i])
      {
          count++;
      }
  }
  if(count==0)
  {
      yes++;
  }
  count =0;
  for(int i=0;i<l/2;i++)
  {
      if(str[i]!=str[(l-3)/2-i])
      {
          count++;
      }
  }
  if(count==0)
  {
      yes++;
  }
  count =0;
   for(int i=(l+1/2);i<l;i++)
  {
      if(str[i]!=str[l-1-i])
      {
          count++;
      }
  }
  if(count==0)
  {
      yes++;
  }
  if(yes==3)
  {
      printf("Yes");

  }
  else printf("No");
}
