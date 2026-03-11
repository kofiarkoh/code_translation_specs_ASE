#include<stdio.h>
int main(){
  int n,i,j;
  scanf("%d",&n);
  long long c,max=0,cal[2*n+2][2];
  scanf("%lld",&c);
  cal[n][0]=0;
  cal[n][1]=0;
  for(i=0;i<n;i++){
    scanf("%lld%lld",&cal[i+n+1][0],&cal[i+n+1][1]);
    cal[i][0]=c-cal[i+n+1][0];
    cal[i][1]=cal[i+n+1][1];
  }
  for(i=n+1;i<=2*n;i++)
    cal[i][1]+=cal[i-1][1];
  for(i=n-1;i>=0;i--)
    cal[i][1]+=cal[i+1][1];
  for(i=0;i<=n;i++){
    for(j=n;j<=n+i;j++){
      c=cal[i][1]+cal[j][1];
      if(cal[i][0]<cal[j][0])
        c-=2*cal[i][0]+cal[j][0];
      else
        c-=cal[i][0]+2*cal[j][0];
      if(max<c)
        max=c;
    }
  }
  printf("%lld\n",max);
  return 0;
}
