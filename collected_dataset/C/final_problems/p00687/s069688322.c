#include<stdio.h>

char counter[10000100];

int main(){
  int a,b,n,i,count;
  do{
    scanf("%d %d %d",&n,&a,&b);
    if(a==0&&b==0&&n==0) break;
    for(i=a;i<=n;i+=a) counter[i]++;
    for(i=0,counter[b]++;i<=n;i++)
      if(counter[i]&&i+a<=n) counter[i+a]++;
    for(i=1,count=0;i<=n;i++)
      if(!counter[i]) count++;
    printf("%d\n",count);
    for(i=0;i<=n;i++) counter[i]=0;
  }while(1);
  return 0;
}