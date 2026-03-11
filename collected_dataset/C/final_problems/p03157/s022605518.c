#include <stdio.h>

int main(){
  int h,w;
  scanf("%d%d",&h,&w);
  char s[h][w+1];
  int i=0;
  for(i=0;i<h;i++){
    scanf("%s",s[i]);
  }
  int count[h][w];
  int j=0;
  for(i=0;i<w;i++){
    for(j=0;j<h;j++){
      count[j][i] = 0;
    }
  }
  int count_num = h * w;
  long long int ans = 0;
  for(i=0;i<h;i++){
    for(j=0;j<w;j++){
      if(count_num == 0){
        break;
      }
      if(count[i][j] == 0){
        //printf("(%d,%d) %c\n",i,j,s[i][j]);
        count[i][j] = 1;
        int tmp[h*w][2];
        int tmp_num = 1;
        tmp[0][0] = i;
        tmp[0][1] = j;
        int black = 0;
        int white = 0;
        if(s[i][j]=='#'){
          black++;
        }else{
          white++;
        }
        while(tmp_num > 0){
          tmp_num--;
          int tmp_i = tmp[tmp_num][0];
          int tmp_j = tmp[tmp_num][1];
          //printf("(%d %d) %c ",tmp_i,tmp_j,s[tmp_i][tmp_j]);
          if(tmp_i > 0 && count[tmp_i-1][tmp_j] == 0 && (s[tmp_i-1][tmp_j] != s[tmp_i][tmp_j])){
            if(s[tmp_i-1][tmp_j]=='#'){
              black++;
            }else{
              white++;
            }
            count[tmp_i-1][tmp_j] = 1;
            tmp[tmp_num][0] = tmp_i-1;
            tmp[tmp_num][1] = tmp_j;
            tmp_num++;
          }
          if(tmp_j > 0 && count[tmp_i][tmp_j-1] == 0 && (s[tmp_i][tmp_j-1] != s[tmp_i][tmp_j])){
            if(s[tmp_i][tmp_j-1]=='#'){
              black++;
            }else{
              white++;
            }
            count[tmp_i][tmp_j-1] = 1;
            tmp[tmp_num][0] = tmp_i;
            tmp[tmp_num][1] = tmp_j-1;
            tmp_num++;
          }
          if(tmp_i < h-1 && count[tmp_i+1][tmp_j] == 0 && (s[tmp_i+1][tmp_j] != s[tmp_i][tmp_j])){
            if(s[tmp_i+1][tmp_j]=='#'){
              black++;
            }else{
              white++;
            }
            count[tmp_i+1][tmp_j] = 1;
            tmp[tmp_num][0] = tmp_i+1;
            tmp[tmp_num][1] = tmp_j;
            tmp_num++;
          }
          if(tmp_j < w-1 && count[tmp_i][tmp_j+1] == 0 && (s[tmp_i][tmp_j+1] != s[tmp_i][tmp_j])){
            if(s[tmp_i][tmp_j+1]=='#'){
              black++;
            }else{
              white++;
            }
            count[tmp_i][tmp_j+1] = 1;
            tmp[tmp_num][0] = tmp_i;
            tmp[tmp_num][1] = tmp_j+1;
            tmp_num++;
          }
        }
        ans += (long long int) black * white;
        //printf("black %d white %d\n", black,white);
      }
    }
  }
  printf("%lld",ans);
  return 0;
}
