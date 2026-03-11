#include <stdio.h>

int a,b,c,d;
int A[100];
int res[100];
int result[100];
int k=0;

//int w,h,x,y,r;

int main(void) {

  while(scanf("%d %d",&a,&b) && a!=0 || b!=0){
      //花札作成
      for(int i=1;i<=a;i++){
        A[i]=a-i+1;
      }


      for(int i=1;i<=b;i++){
        scanf("%d %d",&c,&d);
          //花札取り置き
          for(int i=1;i<=c-1;i++){
            res[i]=A[i];
          }
          //花札入れ替え
          for(int i=c;i<=c+d-1;i++){
            A[i-c+1]=A[i];
          }
          for(int i=d+1;i<=c+d-1;i++){
            A[i]=res[i-d];
          }
      }
      result[k]=A[1];
      k++;
    }
    for(int i=0;i<k;i++){
      printf("%d\n",result[i]);
    }
    return 0;
  }

