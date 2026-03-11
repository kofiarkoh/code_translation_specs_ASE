#include <stdio.h>

int main()
{
  int i, j, n, mx, mxidx;
  long a[100000];
  long color[100000];
  int numcolor=0;
  scanf("%d", &n);
  for(i=0;i<n;i++){
    scanf("%ld", &a[i]);
    if(numcolor==0){
      color[numcolor]=a[i];
      numcolor++;
    }else{
      mx=-1;
      for(j=0;j<numcolor;j++){
        if(color[j]<a[i] && color[j]>mx){
          mx=color[j];
          mxidx=j;
        }
      }
      if(mx>=0){
        color[mxidx]=a[i];
      }else{
        color[numcolor]=a[i];
        numcolor++;
      }
    }
  }

  printf("%d\n", numcolor);

  return 0;
}