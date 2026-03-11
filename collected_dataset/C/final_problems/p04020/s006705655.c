#include<stdio.h>

int main(){
  int n;
  long a[100000],cnt=0;
  scanf("%d",&n);
  scanf("%ld",&a[0]);
  cnt+=a[0]/2;
  a[0]%=2;
  for(int i=1;i<n;i++){
    scanf("%ld",&a[i]);
    if(a[i-1]&&a[i]){
      cnt++;
      a[i-1]--;
      a[i]--;
    }
    cnt+=a[i]/2;
    a[i]%=2;
  }
  printf("%ld\n",cnt);
}