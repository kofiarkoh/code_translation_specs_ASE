#include<stdio.h>
int main(void){
  int num=1,a=1,b=1;
  while(1){
    scanf("%d %d %d",&num,&a,&b);
    if(num==0 && a==0 && b==0){
      break;
    }else{
      int i;
      int point[num];
      for(i=1; i<=num; i++){
        scanf("%d",&point[i]);
      }
      int sa=0;
      int max=0;
      int ba=0;
      for(i=a;i<b+1;i++){
        int d=point[i];
        int c=point[i+1];
        sa=c-d;
        sa=sa*(-1);
        if(sa>=max){
          max=sa;
          ba=i;
        }
      }
      printf("%d\n",ba);
    }
  }
  return 0;
}


