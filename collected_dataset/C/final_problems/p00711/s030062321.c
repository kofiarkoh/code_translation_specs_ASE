#include <stdio.h>
#include <string.h>

void look_for(int,int,int,int);

char map[20][20];
int cnt;

int main(){
  int i,j,w,h;
  while(1){
    cnt=0;
    scanf("%d %d",&w,&h);
    if(w==0 && h==0)break;
    for(i=0;i<h;i++){
      scanf("%s",map[i]);
    }
    for(i=0;i<h;i++){
      for(j=0;j<w;j++){
	if(map[i][j]=='@')
	  look_for(i,j,h,w);
      }
    }
    printf("%d\n",cnt);
  }
  return 0;
}

void look_for(int h,int w,int height,int width){
  int i,j;
  cnt++;
  map[h][w]='N';
  for(i=-1;i<=1;i++){
    for(j=-1;j<=1;j++){
      if(i==j || i==-j || i+h<0 || j+w<0 ||i+h>height || j+w>width || (i==0 && j==0))continue;
      if(map[i+h][w+j]=='.')look_for(i+h,j+w,height,width);
    }
  }
}