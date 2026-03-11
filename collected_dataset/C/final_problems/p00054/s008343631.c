#include <stdio.h>

int main()
{
  int a,b,i,n;
  while(scanf("%d %d %d",&a,&b,&n)!=EOF){
    int s=0;
    a%=b;
    for (i=0;i<n;i++){
      s+=a*10/b;
      a=a*10%b;
    }
    printf("%d\n",s);
  }

  return 0;
}