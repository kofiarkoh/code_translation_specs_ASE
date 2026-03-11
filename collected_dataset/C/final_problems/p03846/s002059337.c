#include <stdio.h>
int main(){
int n;
  scanf("%d",&n);
  int a[n],fil[50000]={0};
  for(int i=0;i<n;i++){
    scanf("%d",&a[i]);
    fil[a[i]]++;
    if(fil[a[i]]>2){
        printf("0");
        return 0;
    }
  }
  if(n%2==1&&fil[0]>1){
      printf("0");
      return 0;
  }
  long long ans=1;
  for(int i=0;i<n/2;i++){
      ans*=2;
      ans%=1000000007;
  }
  printf("%lld",ans);
  return 0;
}
