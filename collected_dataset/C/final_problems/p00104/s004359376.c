#include<stdio.h>
#define H 101
#define W 101

void MagTile(int y,int x,int data[H][W],char Tiles[H][W]){
  if (data[y][x]==1){
    printf("LOOP\n");
    return ;
  }
  if (Tiles[y][x]=='.'){
    printf("%d %d\n",x,y);
    return ;
  }
  switch(Tiles[y][x]){
  case '>':data[y][x++]++;break;
  case '<':data[y][x--]++;break;
  case '^':data[y--][x]++;break;
  case 'v':data[y++][x]++;break;
  }
  MagTile(y,x,data,Tiles);
}

int main(){
  int i,j,w,h;
  int data[H][W];
  char Tiles[H][W];
  while(scanf("%d %d",&h,&w)){
    if ((h==0) && (w==0))
      break;
    for (i=0;i<h;i++)
      for (j=0;j<w;j++){
        scanf(" %c",&Tiles[i][j]);
        data[i][j]=0;
      }
    MagTile(0,0,data,Tiles);
  }

  return 0;
}