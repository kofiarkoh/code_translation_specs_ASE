#include<stdio.h>
int main(){
  int n,i,tmp;
  scanf("%d",&n);
  int p[n],price=0;
    for(i=0;i<n;i++){
      scanf("%d",&p[i] );
    }

    for(i=0;i<n-1;i++){
      if(p[i]>p[i+1]) {
        tmp = p[i];
        p[i] = p[i+1];
        p[i+1] = tmp;
      }
    }

for(i=0;i<n-1;i++) price = price + p[i];
price = price + p[n-1]/2;

printf("%d\n",price);

return 0;
}
