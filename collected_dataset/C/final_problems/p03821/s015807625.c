#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define FOR(n) for(int i=0;i<n;i++)
#define FORJ(n) for(int j=0;j<n;j++)
#define PRN(n) printf("%d\n",n)
#define PRF(f) printf("%.10lf\n",f)
#define PRS(s) printf("%s\n",s)
#define PRC(c) printf("%c",c)
int main(void){
  int n;
  scanf("%d",&n);
  int a[n],b[n],sum=0,cnt;
  FOR(n){
    scanf("%d %d",&a[i],&b[i]);
    sum+=b[i];
  }
  for(int i=n-1;i>=0;i--){
    cnt=b[i];
    if(a[i]){
      if(a[i]%b[i]){
        sum-=a[i]%b[i];
        cnt=b[i]-a[i]%b[i];
      }else{
        sum-=b[i];
      }
    }
    FORJ(i+1)a[j]+=cnt;
  }
  PRN(sum);
  return 0;
}
