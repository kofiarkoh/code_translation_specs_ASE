#include<stdio.h>

int main(void){
  int h,w,diag,ans_diag,tmp;
  int i,j,ans_h,ans_w;
  while(1){
    scanf("%d %d",&h,&w);
    if(h==0 && w==0) break;
    diag=h*h+w*w;
    ans_diag=150*150*2;
    for(i=1;i<=150;i++){
      for(j=i+1;j<=150;j++){
        tmp=i*i+j*j;
        if(tmp==diag && i>h){
          ans_h=i;
          ans_w=j;
          goto loop;
        }
        if(tmp>diag && tmp<ans_diag){
          ans_diag=tmp;
          ans_h=i;
          ans_w=j;
        }
      }
    }
    loop:
    printf("%d %d\n",ans_h,ans_w);
  }
  return 0;
}