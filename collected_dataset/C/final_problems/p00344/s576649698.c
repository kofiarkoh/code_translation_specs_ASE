#include<stdio.h>
int n,i,p,c,ans=0;
int a[100000];
int x[100000];
int y[100000];

int main(){
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
    x[i]=-1;
    y[i]=-1;
  }
  for(i=0;i<n;i++){
    p=i;
    c=0;
    while(x[p]==-1){
      x[p]=c++;
      y[p]=i;
      p=(p+a[p])%n;
    }
    if(y[p]==i)ans+=(c-x[p]);
  }
  printf("%d\n",ans);
  return 0;
}