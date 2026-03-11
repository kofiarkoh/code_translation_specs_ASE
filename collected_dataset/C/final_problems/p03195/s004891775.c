#include <stdio.h>
int main(void){
  int n,a,i,f=0;
  scanf("%d",&n);
  for(i=1;i<=n;i++){
    scanf("%d",&a);
	f |= a%2==1;
  }
  printf(f?"first":"second");
}
