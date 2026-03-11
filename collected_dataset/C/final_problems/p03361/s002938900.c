#include <stdio.h>

int main(){
  int H, W, i, j;
  char s[50][50];
  scanf("%d %d", &H, &W);
  for(i=0;i<H;i++){
    scanf("%s", s[i]);
  }

  for(i=0;i<H;i++){
    for(j=0;j<W;j++){
      int check = 0;
      if(s[i][j] == '#'){
        if(i>0) {if(s[i-1][j] == '#') check++;}
        if(i<H) {if(s[i+1][j] == '#') check++;}
        if(j>0) {if(s[i][j-1] == '#') check++;}
        if(j<W) {if(s[i][j+1] == '#') check++;}
        if(check == 0){
          puts("No");
          return 0;
        }
      }
    }
  }
  puts("Yes");
  return 0;
}