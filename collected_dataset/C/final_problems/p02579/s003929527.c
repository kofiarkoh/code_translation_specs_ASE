#include<stdio.h>

int min(int a , int b){
  if(a < b){
    return a;
  }
  else{
    return b;
  }
}

int main(void){
  int h , w;
  int startx , starty , goalx ,goaly;
  scanf("%d %d %d %d %d %d",&h ,&w ,&startx ,&starty ,&goalx ,&goaly);

  char s[1010][1010];
  for(int i = 0;i < h;i ++){
    //for(int j = 0;j < w;j ++){
      scanf("%s",s[i]);
   //}
  }

  int df[h][w];
  for(int i = 0;i < h;i ++){
    for(int j = 0;j < w;j ++){
      if(s[i][j] == '#'){
        df[i][j] = -1;
      }
      else{
        df[i][j] = 100000;
      }
    }
  }
  

  df[startx-1][starty-1] = 0;
  int count = 0;
  
  /*for(int i = 0;i < h;i ++){
    for(int j = 0;j < w;j ++){
      printf("%d",df[i][j]);
    }
    putchar('\n');
  }*/
  
  while(count < 100){
    count ++;
    for(int i = 0;i < h;i ++){
     for(int j = 0;j < w;j ++){
      if(df[i][j] != -1){
        /*徒歩*/
        if(i-1 >= 0 && df[i-1][j] != -1){
          df[i-1][j] = min(df[i-1][j],df[i][j]);
        }
        if(i+1 < h && df[i+1][j] != -1){
          df[i+1][j] = min(df[i+1][j],df[i][j]);
        }
        if(j-1 >= 0 && df[i][j-1] != -1){
          df[i][j-1] = min(df[i][j-1],df[i][j]);
        }
        if(j+1 < w && df[i][j+1] != -1){
          df[i][j+1] = min(df[i][j+1],df[i][j]);
        }
        /*ワープ*/
        for(int a = -2;a <= 2;a ++){
          for(int b = -2;b <= 2;b ++){
            if(i+a >= 0 && i+a < h){
              if(j+b >= 0 && j+b < w){
                if(df[i+a][j+b] != -1){
                  df[i+a][j+b] = min(df[i+a][j+b],df[i][j]+1);
                }
              }
            }
          }
        }
      }
    }
   }
  }

  int ans = df[goalx-1][goaly-1];
  if(ans == 100000){
    printf("-1\n");
  }
  
  else{
    printf("%d\n",ans);
  }

  return 0;
}