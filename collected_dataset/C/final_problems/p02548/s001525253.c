#include<stdio.h>
int main()
{
    int n,a,b,c,count=0;
    scanf("%d", &n);
    for(a=1;a<=n;a++){
      for(b=1;b<=n/a;b++){
          if(a*b != n){
            count++;
//          	printf("%d x %d + %d\n",a,b,n-(a*b));
          }
      }
    }
    // 出力
    printf("%d\n",count);
    return 0;
}