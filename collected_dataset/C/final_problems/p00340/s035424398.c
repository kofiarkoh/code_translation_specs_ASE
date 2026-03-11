#include<stdio.h>
int main(){
  int e[4]={0,0,0,0},i,j,k=0;
  scanf("%d%d%d%d",&e[0],&e[1],&e[2],&e[3]);
  if(e[0]==e[3]&&e[1]==e[2]||e[1]==e[2]&&e[0]==e[3]||e[2]==e[3]&&e[0]==e[1]){
    printf("yes\n");
  }else{
    printf("no\n");
  }
  return 0;
}