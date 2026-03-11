#include<stdio.h>

int main(){
  
  int h,w,i,j,count = 0,dark;
  char a[1024][1024];
  int b[1024][1024];
  
  scanf("%d %d",&h,&w);
  for(i = 0;i < h;i++)
    scanf("%s",a[i]);
  
  do{
    dark = 0;
    for(i = 0;i < h;i++)
      for(j = 0;j < w;j++)
        if(a[i][j] == '#'){
          dark++;
          if(b[i][j] != 1000)
            b[i][j] = 1;
        }
    if(dark != h * w){
      for(i = 0;i < h;i++)
        for(j = 0;j < w;j++)
          if(b[i][j]){
            if(j != 0)
              a[i][j-1] = '#';
            if(j != w - 1)
              a[i][j+1] = '#';
            if(i != 0)
              a[i-1][j] = '#';
            if(i != h - 1)
              a[i+1][j] = '#';
            b[i][j] = 1000;
          }
      count++;
    }
  }while(dark != h * w);
      
  printf("%d",count);
      
  return 0;
}
