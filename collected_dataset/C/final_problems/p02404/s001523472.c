#include<stdio.h>

int main(){

  int i,j,H,W;
  while (1){
    scanf("%d %d",&H,&W);if(W==0&&H==0){break;}
for(j=0;j<H;j++){
for(i=0;i<W;i++){    
if(i!=0&&i!=W-1&&j!=0&&j!=H-1){printf(".");
}else{printf("#");}
}
printf("\n");
 }printf("\n");}
 
 return 0;
}