#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main(void)
{
  int i,j,k;//counter
  int h,w;
  int r,c;
  char ***str1,**str2;
  scanf("%d%d",&h,&w);
  str1=(char***)malloc(sizeof(char**)*h);
  for(i=0;i<h;i++)
    {
      str1[i]=(char**)malloc(sizeof(char*)*w);
      str1[i][0]=(char*)malloc(sizeof(char)*w);
      scanf("%s",str1[i][0]);
      for(j=1;j<w;j++)
        {
          str1[i][j]=(char*)malloc(sizeof(char)*(w-j));
          strcpy(str1[i][j],str1[i][j-1]+1);
        }
    }
  scanf("%d%d",&r,&c);
  str2=(char**)malloc(sizeof(char*)*r);
  for(i=0;i<r;i++)
    {
      str2[i]=(char*)malloc(sizeof(char)*(c+1));
      scanf("%s",str2[i]);
    }
  for(i=0;i<r;i++) scanf("%s\n",str2[i]);
  //patternSearch
  for(i=0;i<=h-r;i++)
    {
      for(j=0;j<=w-c;j++)
        {
          for(k=0;k<r;k++)
            {
              if(strncmp(str1[i+k][j],str2[k],c)!=0) break;
            }
          if(k==r) printf("%d %d\n",i,j);
        }
    }
  return 0;
}
