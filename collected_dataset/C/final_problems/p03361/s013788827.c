#include<stdio.h>
#include<stdbool.h>

int main()
{
  int h,w;
  
  scanf("%d%d",&h,&w);
  
  char s[h][w];
  int i,j;
  bool b=true;
 
  for(i=0;i<h;i++)
  {
    scanf("%s",&s[i]);
  }    

  for(i=0;i<h;i++)
  {
    for(j=0;j<w;j++)
    {
      if(s[i][j]=='#')
      {
        if(i!=0||i!=h-1||j!=0||j!=w-1)
        {
          if(s[i][j+1]!='#'&&s[i][j-1]!='#'&&s[i-1][j]!='#'&&s[i+1][j]!='#')
          {
            b=false;
          }
        }
        else
        {
          if(s[i][j+1]!='#'&&s[i+1][j]!='#')
          {
            b=false;
          }
        }
      }
    }
  }
        
  if(b==true)
  {
    printf("Yes\n");
  }
  else
  {
    printf("No\n");
  }
  
  return 0;
}