#include<stdio.h>
int main(void)
{
  int n,k;
  scanf("%d %d",&n,&k);
  int a[300000];
  int tmp[300000];

  for(int i=0;i<n;i++){
    scanf("%d",&a[i]);
  }


  while(k-->0){
    int flag=0;
    for(int i=0;i<n;i++){
      tmp[i]=0;
    }

    for(int i=0;i<n;i++){
      int min=i-a[i];
      if(i-a[i]<0){
        min=0;
      }
      tmp[min]++;
      int max=i+a[i];
      if(max>n-1){
        max=n-1;
      }
      if(max+1<n){
        tmp[max+1]--;
      }
    }
    for(int i=0;i<n;i++){
      if(i!=0){
        tmp[i]+=tmp[i-1];
      }
      if(a[i]!=tmp[i]){
        flag=0;
      }
      a[i]=tmp[i];
    }
    if(flag==1)break;
  }


  for(int i=0;i<n;i++){
    printf("%d ",a[i]);
  }
  puts(" ");

  return 0;
}
